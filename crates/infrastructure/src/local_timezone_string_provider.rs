use crate::system_timezone_sources::{
    timezone_from_env_var, timezone_from_etc_localtime, timezone_from_etc_timezone,
};

/// Returns the name of the local timezone as a `String`.
///
/// Sources are tried in order: the `TZ` environment variable,
/// the `/etc/localtime` symlink, then `/etc/timezone`.
///
/// # Panics
///
/// Panics when none of the sources can answer. The CLI cannot offer
/// a default timezone without this information, so starting up
/// makes no sense.
pub fn provide_local_timezone_string() -> String {
    timezone_from_env_var()
        .or_else(timezone_from_etc_localtime)
        .or_else(timezone_from_etc_timezone)
        .unwrap_or_else(|| {
            panic!(
                "System Timezone Not Found:
    Could not find local timezone. Please set TZ environment variable.
    "
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn provides_an_iana_like_timezone_name_from_the_real_system() {
        // Arrange
        // no setup: this test reads the real TZ variable or /etc files
        let iana_name_pattern: Regex = Regex::new(r"^[a-zA-Z_/]+$").unwrap();

        // Act
        let local_timezone = provide_local_timezone_string();

        // Assert
        assert!(iana_name_pattern.is_match(&local_timezone));
    }
}
