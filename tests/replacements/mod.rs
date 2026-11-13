use sedi::{Replacement, replace};

fn eq(expected: &str, input: &str, replacements: &[Replacement]) {
    assert_eq!(expected, replace(input, replacements).unwrap());
}

#[test]
fn _0001() {
    // No replacements preserve the original.
    eq("", "", &[]);
    eq("lorem ipsum dolor", "lorem ipsum dolor", &[]);
    eq("lorem\nipsum dolor", "lorem\nipsum dolor", &[]);
    eq("lorem ipsum\rdolor", "lorem ipsum\rdolor", &[]);
    eq("lorem\n\ripsum dolor", "lorem\n\ripsum dolor", &[]);
    eq("lorem\r\nipsum dolor", "lorem\r\nipsum dolor", &[]);
    eq("lorem dolor\n", "lorem dolor\n", &[]);
    eq("lorem dolor\r", "lorem dolor\r", &[]);
    eq("lorem dolor\r\n", "lorem dolor\r\n", &[]);
    eq("lorem dolor\n\r", "lorem dolor\n\r", &[]);
}

#[test]
fn _0002() {
    // Single replacement.
    let r = vec![Replacement::new("lore", "Lore")];
    eq("", "", &r);
    eq("Lorem ipsum dolor", "lorem ipsum dolor", &r);
    eq("Lorem\nipsum dolor", "lorem\nipsum dolor", &r);
    eq("Lorem ipsum\rdolor", "lorem ipsum\rdolor", &r);
    eq("Lorem\n\ripsum dolor", "lorem\n\ripsum dolor", &r);
    eq("Lorem\r\nipsum dolor", "lorem\r\nipsum dolor", &r);
}

#[test]
fn _0003() {
    // Multiple replacements.
    let r = vec![Replacement::new("l", "d"), Replacement::new("o", "u")];
    eq("durem ipsum dudur", "lorem ipsum dolor", &r);
    eq("durem\nipsum dudur", "lorem\nipsum dolor", &r);
    eq("durem ipsum\rdudur", "lorem ipsum\rdolor", &r);
    eq("durem\n\ripsum dudur", "lorem\n\ripsum dolor", &r);
    eq("durem\r\nipsum dudur", "lorem\r\nipsum dolor", &r);
}

#[test]
fn _0004() {
    // Cascaded replacements.
    let r = vec![Replacement::new("^l", "L"), Replacement::new("Lo", "Du")];
    eq("Durem ipsum dolor", "lorem ipsum dolor", &r);
    eq("Durem\nipsum dolor", "lorem\nipsum dolor", &r);
    eq("Durem ipsum\rdolor", "lorem ipsum\rdolor", &r);
    eq("Durem\n\ripsum dolor", "lorem\n\ripsum dolor", &r);
    eq("Durem\r\nipsum dolor", "lorem\r\nipsum dolor", &r);
}

#[test]
fn _0005() {
    // Little-bit more sophisticated replace pattern.
    let r = vec![Replacement::new("[a-z]+", "W")];
    eq("W W W", "lorem ipsum dolor", &r);
    eq("W\nW W", "lorem\nipsum dolor", &r);
    eq("W W\rW", "lorem ipsum\rdolor", &r);
    eq("W\n\rW W", "lorem\n\ripsum dolor", &r);
    eq("W\r\nW W", "lorem\r\nipsum dolor", &r);
}

#[test]
fn _0006() {
    // More complex replacements.
    let r = vec![
        Replacement::new("([a-z]+)", ".$1"),
        Replacement::new(".lorem", "Borem"),
        Replacement::new("\\.ipsum", "ups"),
        Replacement::new("\\..+", "end"),
    ];
    eq("Borem ups end", "lorem ipsum dolor", &r);
    eq("Borem\nups end", "lorem\nipsum dolor", &r);
    eq("Borem ups\rend", "lorem ipsum\rdolor", &r);
    eq("Borem\n\rups end", "lorem\n\ripsum dolor", &r);
    eq("Borem\r\nups end", "lorem\r\nipsum dolor", &r);
}

#[test]
fn _0007() {
    // Inserting at the beginning of each line.
    let r = vec![Replacement::new("^", ">")];
    eq("", "", &r);
    eq(">a\n>b\n", "a\nb\n", &r);
    eq(">a\n>\n>b", "a\n\nb", &r);
    eq(">a\n>\n>b\n", "a\n\nb\n", &r);
}

#[test]
fn _0008() {
    // Inserting at the end of each line.
    let r = vec![Replacement::new("$", ";")];
    eq("", "", &r);
    eq("a;\nb;\n", "a\nb\n", &r);
    eq("a;\n;\nb;", "a\n\nb", &r);
    eq("a;\n;\nb;\n", "a\n\nb\n", &r);
}

#[test]
fn _0009() {
    // Invalid pattern.
    let r = vec![Replacement::new("([a-z]+", "A")];
    assert_eq!(
        "regex parse error:\n    ([a-z]+\n    ^\nerror: unclosed group",
        replace("alfa beta gamma", &r).unwrap_err()
    );
}
