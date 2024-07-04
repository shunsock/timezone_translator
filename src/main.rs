mod command;
mod translator;
mod validator;

use chrono::prelude::*;
use chrono_tz::Tz;
use clap::ArgMatches;
use command::receiver::receive_user_input;
use command::validated_options::validated_user_inputs::ValidatedCommandOptions;
use std::process::exit;
use translator::translation_error::TranslationError;
use translator::translator::TimezoneTranslator;
use validator::command_options_validator::validate_command_options;

fn main() {
    let user_input_options: ArgMatches = receive_user_input();

    let validated_options: ValidatedCommandOptions = match validate_command_options(&user_input_options) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    let date_time_mapped: Result<DateTime<Tz>, TranslationError> = TimezoneTranslator::new(
        validated_options.time(),
        validated_options.from_tz(),
        validated_options.to_tz(),
        validated_options.ambiguous_time_strategy(),
    )
    .convert();

    match date_time_mapped {
        Ok(mapped) => println!("{}", mapped),
        Err(e) => eprintln!("{}", e),
    }
}
