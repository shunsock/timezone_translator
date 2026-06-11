#[derive(thiserror::Error, Debug, PartialEq)]
pub(crate) enum ValidationError {
    #[error("Validation Error: Invalid time format found. {0} (expected: YYYY-MM-DD hh:mm:ss)")]
    TimeFormat(String),

    #[error(
        "Validation Error: Invalid timezone found {0}. @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html"
    )]
    Timezone(String),

    #[error("Validation Error: Invalid ambiguous time strategy found. {ambiguous_time_strategy} (expected: earliest, latest)")]
    AmbiguousTimeStrategy { ambiguous_time_strategy: String },
}
