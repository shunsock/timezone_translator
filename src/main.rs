use chrono::prelude::*;
use chrono_tz::Tz;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("timezone_converter")
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

    let time_str = matches.get_one::<String>("time").unwrap();
    let from_tz_str = matches.get_one::<String>("from_timezone").unwrap();
    let to_tz_str = matches.get_one::<String>("to_timezone").unwrap();

    // Parse the input time
    let naive_time = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S").expect("Invalid time format");

    // Parse the timezones
    let from_tz: Tz = from_tz_str.parse().expect("Invalid from timezone");
    let to_tz: Tz = to_tz_str.parse().expect("Invalid to timezone");

    // Localize the time to the from_tz timezone
    let from_time = from_tz.from_local_datetime(&naive_time).single().expect("Invalid local time");

    // Convert the time to the target timezone
    let to_time = from_time.with_timezone(&to_tz);

    // Print the converted time
    println!("Converted time: {}", to_time.format("%Y-%m-%d %H:%M:%S %Z"));
}

