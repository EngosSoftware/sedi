#![doc = include_str!("../docs/lib.rs.md")]

//! Inplace stream editor

use regex::Regex;
use std::fmt::Write;

#[derive(Debug, Default, Clone)]
pub struct Replacement {
    /// Pattern to search for.
    pattern: String,
    /// Replacement text for the pattern.
    text: String,
    /// Pattern of the line that starts applying replacement.
    start_pattern: Option<String>,
    /// Pattern of the line that ends applying replacement.
    end_pattern: Option<String>,
}

impl Replacement {
    /// Creates a new replacement.
    pub fn new(pattern: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
            text: text.into(),
            start_pattern: None,
            end_pattern: None,
        }
    }

    #[must_use]
    pub fn start_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.start_pattern = Some(pattern.into());
        self
    }

    #[must_use]
    pub fn end_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.end_pattern = Some(pattern.into());
        self
    }
}

struct RegexReplacement {
    active: bool,
    regex: Regex,
    text: String,
    start_pattern: Option<Regex>,
    end_pattern: Option<Regex>,
}

impl TryFrom<&Replacement> for RegexReplacement {
    type Error = regex::Error;

    fn try_from(value: &Replacement) -> Result<Self, Self::Error> {
        Ok(Self {
            active: value.start_pattern.is_none(),
            regex: Regex::new(&value.pattern)?,
            text: value.text.clone(),
            start_pattern: value.start_pattern.as_deref().map(Regex::new).transpose()?,
            end_pattern: value.end_pattern.as_deref().map(Regex::new).transpose()?,
        })
    }
}

struct RegexReplacements {
    items: Vec<RegexReplacement>,
}

impl RegexReplacements {
    fn new(replacements: &[Replacement]) -> Result<Self, regex::Error> {
        let mut items: Vec<RegexReplacement> = vec![];
        for replacement in replacements {
            items.push(replacement.try_into()?);
        }
        Ok(Self { items })
    }

    fn apply(&self, mut line: String) -> String {
        for rr in &self.items {
            if rr.active && rr.regex.is_match(&line) {
                line = rr.regex.replace_all(&line, &rr.text).to_string();
            }
        }
        line
    }

    fn activate(&mut self, line: &str) {
        for item in &mut self.items {
            if item
                .start_pattern
                .as_ref()
                .is_some_and(|re| re.is_match(line))
            {
                item.active = true;
            }
        }
    }

    fn deactivate(&mut self, line: &str) {
        for item in &mut self.items {
            if item
                .end_pattern
                .as_ref()
                .is_some_and(|re| re.is_match(line))
            {
                item.active = false;
            }
        }
    }
}

/// Replaces text patterns with substitutions.
///
/// # Errors
///
/// //TODO Describe errors.
pub fn replace(input: &str, replacements: &[Replacement]) -> Result<String, String> {
    let mut replacements = RegexReplacements::new(replacements).map_err(|e| e.to_string())?;
    let mut output = String::new();
    let mut line = String::new();
    let mut ending = String::new();
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if match ch {
            '\n' => {
                ending.push('\n');
                true
            }
            '\r' => {
                ending.push('\r');
                if chars.peek() == Some(&'\n') {
                    chars.next();
                    ending.push('\n');
                }
                true
            }
            other => {
                line.push(other);
                false
            }
        } {
            replacements.activate(&line);
            _ = write!(
                &mut output,
                "{}{}",
                replacements.apply(line.clone()),
                std::mem::take(&mut ending)
            );
            replacements.deactivate(&line);
            line.clear();
        }
    }
    if !line.is_empty() {
        replacements.activate(&line);
        _ = write!(
            &mut output,
            "{}{}",
            replacements.apply(std::mem::take(&mut line)),
            std::mem::take(&mut ending)
        );
    }
    Ok(output)
}
