use clars::antex::{StyledText, Text};
use clars::{Clar, ClarArgument, ClarOption};

const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum CliAction {
    ShowHelp(Text),
    ShowVersion(Text),
    Replace(String, String, String),
    Error(Text),
}

fn get_cli(app: String) -> Clar {
    Clar::new(app).description(DESCRIPTION).options_arguments(
        [
            ClarOption::short("version", 'V')
                .long_label("version")
                .standalone()
                .help("Print version"),
            ClarOption::short("help", 'h')
                .long_label("help")
                .standalone()
                .help("Print help"),
        ],
        [
            ClarArgument::new("pattern")
                .required()
                .help("Regular expression to search for"),
            ClarArgument::new("replacement")
                .required()
                .help("String to replace each match with"),
            ClarArgument::new("file")
                .required()
                .help("File to edit in place"),
        ],
    )
}

pub fn get_action(app: String, args: Vec<String>) -> CliAction {
    match get_cli(app).resolve(args) {
        Ok(matches) => {
            if matches.is_short("help") {
                return CliAction::ShowHelp(matches.get_help());
            }
            if matches.is_long("help") {
                return CliAction::ShowHelp(matches.get_help_long());
            }
            if matches.is_present("version") {
                let version = Text::default()
                    .bright_cyan()
                    .bold()
                    .s(NAME)
                    .r(" v")
                    .s(VERSION);
                return CliAction::ShowVersion(version);
            }
            if let (Some(pattern), Some(replacement), Some(file)) = (
                matches.get_first_value("pattern"),
                matches.get_first_value("replacement"),
                matches.get_first_value("file"),
            ) {
                return CliAction::Replace(pattern, replacement, file);
            }
            CliAction::ShowHelp(matches.get_help())
        }
        Err(diagnostic) => CliAction::Error(diagnostic.text().clone()),
    }
}
