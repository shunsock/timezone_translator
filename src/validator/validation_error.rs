#[derive(thiserror::Error, Debug, PartialEq)]
pub(crate) enum ValidationError {
    #[error("Invalid time format found: {0} (expected: YYYY-MM-DD hh:mm:ss)")]
    TimeFormat(String),

    #[error(
        "Invalid timezone found: {0}. @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html"
    )]
    Timezone(String),

    #[error("Invalid ambiguous time strategy found: {ambiguous_time_strategy} (expected: earliest, latest)")]
    AmbiguousTimeStrategy { ambiguous_time_strategy: String },
}
