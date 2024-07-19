use super::get_system_timezone_from_env_var_tz::EnvironmentVariableTzProvider;
use super::get_system_timezone_from_etc_localtime::get_system_timezone_from_etc_localtime;
use super::get_system_timezone_from_etc_timezone::get_system_timezone_from_etc_timezone;

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
/// if environment variable TZ and following file links are not found,
/// this function return panic
pub(crate) fn provide_local_timezone_string() -> String {
    // read environment variable TZ
    let env_var_tz: Option<String> = EnvironmentVariableTzProvider::new(None).get_env_var_tz();
    if let Some(env_var_tz) = env_var_tz {
        return env_var_tz;
    }

    // read /etc/localtime
    let tz_from_etc_localtime: Option<String> = get_system_timezone_from_etc_localtime();
    if let Some(tz_from_etc_localtime) = tz_from_etc_localtime {
        return tz_from_etc_localtime;
    }

    // read /etc/timezone
    let tz_from_etc_timezone: Option<String> = get_system_timezone_from_etc_timezone();
    if let Some(tz_from_etc_timezone) = tz_from_etc_timezone {
        return tz_from_etc_timezone;
    }

    let error_message = "System Timezone Not Found:
    Could not find local timezone. Please set TZ environment variable.
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
