use clap::Arg;

/// `--to`: the timezone the time should be translated into.
/// Defaults to the local timezone detected at startup.
pub(crate) fn to_timezone(default_timezone: String) -> Arg {
    Arg::new("to_timezone")
    .short('t')
    .long("to")
    .value_name("TO_TIMEZONE")
    .help("The target timezone (e.g. Asia/Tokyo) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html#")
    .required(false)
    .default_value(default_timezone)
}
