use std::fs;
use chrono::{DateTime, Local, Offset, TimeZone};
use chrono_tz::Tz;
use super::get_env_var_tz::EnvironmentVariableTzProvider;
use super::get_system_timezone_from_etc_localtime::get_system_timezone_from_etc_localtime;

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
pub(crate) fn provide_local_timezone_string() -> String {
    // 環境変数からタイムゾーンを取得
    let env_var_tz: Option<String> = EnvironmentVariableTzProvider::new(None).get_env_var_tz();
    if env_var_tz != None {
        return env_var_tz.unwrap()
    }

    // 環境変数TZが設定されていない場合、/etc/localtimeをチェックする
    let timezone: Option<String> = get_system_timezone_from_etc_localtime();

    println!("System timezone: {}", timezone.clone().unwrap());

    timezone.unwrap()

    // システムのタイムゾーンを取得する
    // This should never happen because input timezone is
    // let error_message = "Unexpected error:
    // Could not find local timezone. Please report this issue.
    // https://github.com/shunsock/timezone_translator/issues
    // ";
    // panic!("{}", error_message);
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
