use normalized_line_endings::Normalized;
use std::fmt::Write;

fn main() {
  let args = std::env::args().skip(1).collect::<Vec<String>>();
  if args.len() != 3 {
    eprintln!("error: invalid number of arguments");
    eprintln!("\nUsage: sedi <PATTERN> <REPLACEMENT> <FILE>");
    std::process::exit(1);
  }
  let pattern = &args[0];
  let replacement = &args[1];
  let file_name = &args[2];

  let regex = regex::Regex::new(pattern).unwrap();
  let input = std::fs::read_to_string(file_name).unwrap();
  let mut output = String::new();
  let mut line = String::new();

  for ch in input.chars().normalized() {
    if ch == '\n' {
      _ = writeln!(&mut output, "{}", regex.replace_all(&line, replacement));
      line.clear();
    } else {
      line.push(ch);
    }
  }
  if !line.is_empty() {
    _ = write!(&mut output, "{}", regex.replace_all(&line, replacement));
    line.clear();
  }
  std::fs::write(file_name, output).unwrap();
}
