use clap::Arg;

pub(crate) fn to(timezone: &'static str) -> Arg {
    Arg::new("to_timezone")
    .short('t')
    .long("to")
    .value_name("TO_TIMEZONE")
    .help("The target timezone (e.g. Asia/Tokyo) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html#")
    .required(false)
    .default_value(timezone)
}
