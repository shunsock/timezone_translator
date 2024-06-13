mod factory;
mod validator;
mod converter;

use chrono::prelude::*;
use chrono_tz::Tz;
use clap::ArgMatches;
use converter::converter::Converter;
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

    let res: Result<DateTime<Tz>, String> = Converter::new(
        validator.time(),
        validator.from_tz(),
        validator.to_tz(),
    ).convert();

    match res {
        Ok(_) => {
            println!("{}", res.unwrap());
            exit(0);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
