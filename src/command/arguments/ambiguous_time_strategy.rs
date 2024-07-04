use clap::Arg;

pub(crate) fn ambiguous_time_strategy() -> Arg {
    Arg::new("ambiguous_time_strategy")
        .short('a')
        .long("ambiguous-time-strategy")
        .value_name("STRATEGY")
        .help("Strategy to use for ambiguous times (earliest, latest)")
        .default_value("earliest")
        .required(false)
}
