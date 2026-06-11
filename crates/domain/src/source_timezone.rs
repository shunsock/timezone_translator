use crate::timezone_parse_error::TimezoneParseError;
use chrono_tz::Tz;
use std::str::FromStr;

/// The timezone the input time is expressed in.
///
/// Distinct from [`crate::TargetTimezone`] so that source and target
/// cannot be swapped by mistake at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceTimezone(Tz);

impl FromStr for SourceTimezone {
    type Err = TimezoneParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.parse::<Tz>()
            .map(SourceTimezone)
            .map_err(|_| TimezoneParseError(text.to_string()))
    }
}

impl SourceTimezone {
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
        let input = "America/New_York";

        // Act
        let source: SourceTimezone = input.parse().unwrap();

        // Assert
        assert_eq!(source.timezone(), chrono_tz::America::New_York);
    }

    #[test]
    fn rejects_unknown_timezone_name() {
        // Arrange
        let input = "Invalid/Timezone";

        // Act
        let result: Result<SourceTimezone, TimezoneParseError> = input.parse();

        // Assert
        assert_eq!(result, Err(TimezoneParseError(input.to_string())));
    }
}
