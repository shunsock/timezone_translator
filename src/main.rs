mod validator;

use std::process::exit;
use chrono::prelude::*;
use chrono_tz::Tz;
use clap::{Arg, ArgMatches, Command};
use validator::validator::{command_validator, Validator};


fn main() {
    let matches: ArgMatches = Command::new("timezone_converter")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Converts time between time zones")
        .arg(Arg::new("time")
            .short('T')
            .long("time")
            .value_name("TIME")
            .help("Time in the format YYYY-MM-DD HH:MM:SS")
            .required(true)
        )
        .arg(Arg::new("from_timezone")
            .short('f')
            .long("from")
            .value_name("FROM_TIMEZONE")
            .help("The original timezone (e.g., America/New_York)")
            .required(true)
        )
        .arg(Arg::new("to_timezone")
            .short('t')
            .long("to")
            .value_name("TO_TIMEZONE")
            .help("The target timezone (e.g., Asia/Tokyo)")
            .required(true)
        )
        .get_matches();

    let validator: Validator = match command_validator(&matches) {
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
