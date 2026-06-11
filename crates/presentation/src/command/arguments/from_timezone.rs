use clap::Arg;

/// `--from`: the timezone the input time is expressed in.
/// Defaults to the local timezone detected at startup.
pub(crate) fn from_timezone(default_timezone: String) -> Arg {
    Arg::new("from_timezone")
    .short('f')
    .long("from")
    .value_name("FROM_TIMEZONE")
    .help("The original timezone (e.g. America/New_York) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html")
    .required(false)
    .default_value(default_timezone)
}
