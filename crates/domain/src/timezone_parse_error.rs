/// Raised when an input string is not an IANA timezone name.
///
/// Shared by `SourceTimezone` and `TargetTimezone`: the failure is the
/// same (unknown timezone name), only the role of the value differs.
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error(
    "Validation Error: Invalid timezone found {0}. @see https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html"
)]
pub struct TimezoneParseError(pub String);
