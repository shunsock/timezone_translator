mod command;
mod validator;

use clap::ArgMatches;
use command::receiver::receive_user_input;
use command::validated_options::validated_user_inputs::ValidatedCommandOptions;
use std::process::ExitCode;
use usecase::TimezoneTranslator;
use validator::command_options_validator::validate_command_options;

/// Entry point of the CLI.
///
/// Receives user input, validates it, translates the time,
/// and prints the result (or an error message to stderr).
pub fn run() -> ExitCode {
    let user_input_options: ArgMatches = receive_user_input();

    let validated_options: ValidatedCommandOptions =
        match validate_command_options(&user_input_options) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            }
        };

    let translation_result = TimezoneTranslator::new(
        validated_options.get_param_time(),
        validated_options.get_param_from_tz(),
        validated_options.get_param_to_tz(),
        validated_options.ambiguous_time_strategy(),
    )
    .convert();

    match translation_result {
        Ok(mapped) => {
            println!("{}", mapped);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
