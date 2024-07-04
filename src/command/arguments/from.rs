use clap::Arg;

pub(crate) fn from(
    timezone: &'static str
) -> Arg{
    Arg::new("from_timezone")
    .short('f')
    .long("from")
    .value_name("FROM_TIMEZONE")
    .help("The original timezone (e.g. America/New_York) @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html")
    .required(false)
    .default_value(timezone)
}