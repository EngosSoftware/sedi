use sedi::{Replacement, replace};

#[test]
fn _0001() {
    let r = vec![Replacement::new("a", "b").start_pattern("([a-z]+")];
    assert_eq!(
        "regex parse error:\n    ([a-z]+\n    ^\nerror: unclosed group",
        replace("a", &r).unwrap_err()
    );
}

#[test]
fn _0002() {
    let r = vec![Replacement::new("A", "B").end_pattern("([A-Z]+")];
    assert_eq!(
        "regex parse error:\n    ([A-Z]+\n    ^\nerror: unclosed group",
        replace("A", &r).unwrap_err()
    );
}

#[test]
fn _0003() {
    // No regions.
    let input = "A\n      B\n      C\n      D\n";
    let r = vec![Replacement::new("(^\\s*)([A-Z])", "$1#$2")];
    assert_eq!(
        "#A\n      #B\n      #C\n      #D\n",
        replace(input, &r).unwrap()
    );
}

#[test]
fn _0004() {
    // Region starts by A.
    let input = "A\n      B\n      C\n      D\n";
    let r = vec![Replacement::new("(^\\s*)([A-Z])", "$1#$2").start_pattern("^\\s*A")];
    assert_eq!(
        "#A\n      #B\n      #C\n      #D\n",
        replace(input, &r).unwrap()
    );
}

#[test]
fn _0005() {
    // Region ends by D.
    let input = r"
      A
      B
      C
      D
    ";
    let r = vec![Replacement::new("(^\\s+)([A-Z])", "$1#$2").end_pattern("^\\s+D")];
    assert_eq!(
        r"
      #A
      #B
      #C
      #D
    ",
        replace(input, &r).unwrap()
    );
}

#[test]
fn _0006() {
    // Region ends by C.
    let input = r"
      A
      B
      C
      D
    ";

    let r = vec![Replacement::new("(^\\s+)([A-Z])", "$1#$2").end_pattern("^\\s+C")];
    assert_eq!(
        r"
      #A
      #B
      #C
      D
    ",
        replace(input, &r).unwrap()
    );
}

#[test]
fn _0007() {
    // Region starts by B and ends by C.
    let input = r"
      A
      B
      C
      D
    ";

    let r = vec![
        Replacement::new("(^\\s+)([A-Z])", "$1#$2")
            .start_pattern("^\\s+B")
            .end_pattern("^\\s+C"),
    ];
    assert_eq!(
        r"
      A
      #B
      #C
      D
    ",
        replace(input, &r).unwrap()
    );
}

#[test]
fn _0008() {
    // Region starts by B and ends by empty line after C.
    let input = r"      A
      B
      C

      D
    ";

    let r = vec![
        Replacement::new("(^\\s+)([A-Z])", "$1#$2")
            .start_pattern("^\\s+B")
            .end_pattern("^$"),
    ];
    assert_eq!(
        r"      A
      #B
      #C

      D
    ",
        replace(input, &r).unwrap()
    );
}
