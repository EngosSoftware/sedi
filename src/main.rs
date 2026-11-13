mod cli;

use crate::cli::{CliAction, get_action};
use clars::antex::ColorMode;
use clars::{get_app_and_args, get_more_info_hint};
use glob::glob;
use sedi::Replacement;
use std::fs;
use std::process::ExitCode;

fn replace(pattern: &str, replacement: &str, files: &str) -> Result<(), String> {
    for entry in glob(files).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?;
        if path.is_file() {
            let input = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let replacement = Replacement::new(pattern, replacement);
            let output = sedi::replace(&input, &[replacement])?;
            fs::write(path, output).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let (app, args) = get_app_and_args();
    match get_action(app, args) {
        CliAction::ShowHelp(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        CliAction::ShowVersion(version) => {
            println!("{version}");
            ExitCode::SUCCESS
        }
        CliAction::Replace(pattern, replacement, files) => {
            match replace(&pattern, &replacement, &files) {
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
