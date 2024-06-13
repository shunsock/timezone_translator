mod factory;
mod validator;

use chrono::prelude::*;
use chrono_tz::Tz;
use clap::ArgMatches;
use factory::command_factory::command_factory;
use std::process::exit;
use validator::command_options_validator::validate_command_options;
use validator::validated_command_options::ValidatedCommandOptions;


fn main() {
    let matches: ArgMatches = command_factory();

    let validator: ValidatedCommandOptions = match validate_command_options(&matches) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    // Localize the time to the from_tz timezone
    let from_time: DateTime<Tz> = validator
        .from_tz()
        .from_local_datetime(&validator.time())
        .single()
        .expect("Invalid local time");

    // Convert the time to the target timezone
    let to_time: DateTime<Tz> = from_time.with_timezone(&validator.to_tz());

    // Print the converted time
    println!("{}", to_time.format("%Y-%m-%d %H:%M:%S %Z"));
    exit(0);
}
