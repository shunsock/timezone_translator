use clap::Arg;

pub(crate) fn time() -> Arg {
    Arg::new("time")
        .short('T')
        .long("time")
        .value_name("TIME")
        .help(
            "Time in the format YYYY-MM-DD HH:MM:SS (you can omit HH:MM:SS) or YYYY-MM-DDTHH:MM:SS",
        )
        .required(true)
}
