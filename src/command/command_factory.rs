use super::local_timezone_string_provider::provide_local_timezone_string;
use clap::{Arg, ArgMatches, Command};

pub(crate) fn command_factory() -> ArgMatches {
    let now: String = provide_local_timezone_string();
    let now_str: &'static str = Box::leak(now.into_boxed_str());

    Command::new("timezone_translator")
        .version("1.0")
        .author("Your Name s.tsuchiya.business@gmail.com")
        .about("Converts time between time zones")
        .arg(Arg::new("time")
            .short('T')
            .long("time")
            .value_name("TIME")
            .help("Time in the format YYYY-MM-DD HH:MM:SS (you can omit HH:MM:SS) or YYYY-MM-DDTHH:MM:SS")
            .required(true)
        )
        .arg(Arg::new("from_timezone")
            .short('f')
            .long("from")
            .value_name("FROM_TIMEZONE")
            .help("The original timezone (e.g. America/New_York) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html")
            .required(false)
            .default_value(now_str)
        )
        .arg(Arg::new("to_timezone")
            .short('t')
            .long("to")
            .value_name("TO_TIMEZONE")
            .help("The target timezone (e.g. Asia/Tokyo) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html#")
            .required(false)
            .default_value(now_str)
        )
        .arg(Arg::new("ambiguous_time_strategy")
            .short('a')
            .long("ambiguous-time-strategy")
            .value_name("STRATEGY")
            .help("Strategy to use for ambiguous times (earliest, latest)")
            .default_value("earliest")
            .required(false)
        )
        .get_matches()
}
