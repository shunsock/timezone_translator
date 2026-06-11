use crate::timezone_parse_error::TimezoneParseError;
use chrono_tz::Tz;
use std::str::FromStr;

/// The timezone the time should be translated into.
///
/// Distinct from [`crate::SourceTimezone`] so that source and target
/// cannot be swapped by mistake at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetTimezone(Tz);

impl FromStr for TargetTimezone {
    type Err = TimezoneParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.parse::<Tz>()
            .map(TargetTimezone)
            .map_err(|_| TimezoneParseError(text.to_string()))
    }
}

impl TargetTimezone {
    pub fn timezone(&self) -> Tz {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_iana_timezone_name() {
        // Arrange
        let input = "Asia/Tokyo";

        // Act
        let target: TargetTimezone = input.parse().unwrap();

        // Assert
        assert_eq!(target.timezone(), chrono_tz::Asia::Tokyo);
    }

    #[test]
    fn rejects_unknown_timezone_name() {
        // Arrange
        let input = "Invalid/Timezone";

        // Act
        let result: Result<TargetTimezone, TimezoneParseError> = input.parse();

        // Assert
        assert_eq!(result, Err(TimezoneParseError(input.to_string())));
    }
}
