use chrono::{DateTime, Local, Offset};
use chrono_tz::Tz;
use chrono_tz::TZ_VARIANTS;

/// Returns the name of the local timezone as a `String`.
///
/// # Examples
///
/// ```
/// let timezone = local_timezone_string();
/// println!("Local timezone: {}", timezone);
/// ```
///
/// # Panics
///
/// If no matching timezone is found, the function panics with an error message
/// indicating the issue and a link to report the problem.
///
/// # See Also
///
/// - [Chrono crate documentation](https://docs.rs/chrono)
/// - [Chrono-tz crate documentation](https://docs.rs/chrono-tz)
pub(super) fn provide_local_timezone_string() -> String {
    let local_datetime_now: DateTime<Local> = Local::now();

    for tz in TZ_VARIANTS {
        let datetime_with_tz: DateTime<Tz> = local_datetime_now.with_timezone(&tz);
        if datetime_with_tz.offset().fix() == local_datetime_now.offset().fix() {
            return tz.name().to_string();
        }
    }

    // This should never happen because input timezone is
    let error_message = "Unexpected error:
    Could not find local timezone. Please report this issue.
    https://github.com/shunsock/timezone_translator/issues
    ";
    panic!("{}", error_message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_check_output_match_timezone() {
        let local_timezone_str = provide_local_timezone_string();
        let re: Regex = Regex::new(r"^[a-zA-Z_/]+$").unwrap();
        assert!(re.is_match(&local_timezone_str));
    }
}
