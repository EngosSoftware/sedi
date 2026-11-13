mod cli;

use crate::cli::{CliAction, get_action};
use clars::antex::ColorMode;
use clars::{get_app_and_args, get_more_info_hint};
use glob::glob;
use sedi::Replacement;
use std::fs;
use std::process::ExitCode;

fn run_task(file: &str) -> Result<(), String> {
    let content = fs::read_to_string(file).map_err(|e| format!("{e}: {file}"))?;
    let doc = diml::parse(&content).map_err(|e| e.to_string())?;
    let mut files = vec![];
    let mut replacements = vec![];
    for node in doc.children() {
        match node.name() {
            "file" => files.push(node.content().trim()),
            "replacement" => {
                let Some(pattern) = node.first_with_name("pattern") else {
                    return Err("pattern is required".to_string());
                };
                let Some(text) = node.first_with_name("text") else {
                    return Err("text is required".to_string());
                };
                let mut replacement =
                    Replacement::new(pattern.content().trim(), text.content().trim());
                if let Some(start_pattern) = node.first_with_name("start-pattern") {
                    replacement = replacement.start_pattern(start_pattern.content().trim());
                }
                if let Some(end_pattern) = node.first_with_name("end-pattern") {
                    replacement = replacement.end_pattern(end_pattern.content().trim());
                }
                replacements.push(replacement);
            }
            _ => {}
        }
    }
    for file in files {
        replace(&replacements, file)?;
    }
    Ok(())
}

fn replace(replacements: &[Replacement], files: &str) -> Result<(), String> {
    for entry in glob(files).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?;
        if path.is_file() {
            let input =
                fs::read_to_string(&path).map_err(|e| format!("{e}: {}", path.display()))?;
            let output = sedi::replace(&input, replacements)?;
            fs::write(path, output).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let (app, args) = get_app_and_args();
    match get_action(app, args) {
        CliAction::RunTask(file) => match run_task(&file) {
            Ok(()) => ExitCode::SUCCESS,
            Err(reason) => {
                eprintln!("{reason}");
                ExitCode::FAILURE
            }
        },
        CliAction::ShowHelp(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        CliAction::ShowVersion(version) => {
            println!("{version}");
            ExitCode::SUCCESS
        }
        CliAction::Replace(pattern, replacement, files) => {
            let replacements = vec![Replacement::new(pattern, replacement)];
            match replace(&replacements, &files) {
                Ok(()) => ExitCode::SUCCESS,
                Err(reason) => {
                    eprintln!("{reason}");
                    ExitCode::FAILURE
                }
            }
        }
        CliAction::Error(text) => {
            eprintln!("{text}");
            eprintln!(
                "{}",
                get_more_info_hint(ColorMode::default(), &["-h", "--help"])
            );
            ExitCode::FAILURE
        }
    }
}
