use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("Required Parameter is not found: {name:?} ({help:?})")]
    ParameterNotFound {
        name: String,
        help: String,
    },

    #[error("Invalid time format found: {0} (expected: YYYY-MM-DD hh:mm:ss)")]
    InvalidTimeFormat(String),

    #[error("Invalid timezone found: {0}. @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html")]
    InvalidTimezone(String),
}