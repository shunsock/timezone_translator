mod command;
mod validator;

use self::command::receiver::receive_user_input;
use self::validator::command_options_validator::validate_command_options;
use crate::domain::TranslationRequest;
use crate::usecase::TimezoneTranslator;
use clap::ArgMatches;
use std::process::ExitCode;

/// Entry point of the CLI.
///
/// Receives user input, parses it into a `TranslationRequest`,
/// translates the time, and prints the result (or an error to stderr).
pub fn run() -> ExitCode {
    let user_input_options: ArgMatches = receive_user_input();

    let request: TranslationRequest = match validate_command_options(&user_input_options) {
        Ok(request) => request,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    match TimezoneTranslator::new(request).convert() {
        Ok(translated) => {
            println!("{}", translated);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
