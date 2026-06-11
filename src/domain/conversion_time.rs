use chrono::NaiveDateTime;
use std::str::FromStr;

/// The wall-clock time to convert, before a timezone is attached.
///
/// Construction is only possible through `FromStr`, so holding a
/// `ConversionTime` proves the input matched one of the accepted formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionTime(NaiveDateTime);

/// Accepted input formats.
///
/// Each entry pairs a strict regex (rejects unpadded digits, which
/// chrono alone would accept) with the chrono format used for parsing.
/// A date without a time is completed with midnight before parsing.
const ACCEPTED_FORMATS: [AcceptedFormat; 3] = [
    AcceptedFormat {
        pattern: r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$",
        chrono_format: "%Y-%m-%d %H:%M:%S",
        completes_midnight: false,
    },
    AcceptedFormat {
        pattern: r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$",
        chrono_format: "%Y-%m-%dT%H:%M:%S",
        completes_midnight: false,
    },
    AcceptedFormat {
        pattern: r"^\d{4}-\d{2}-\d{2}$",
        chrono_format: "%Y-%m-%d %H:%M:%S",
        completes_midnight: true,
    },
];

struct AcceptedFormat {
    pattern: &'static str,
    chrono_format: &'static str,
    completes_midnight: bool,
}

impl AcceptedFormat {
    fn parse(&self, text: &str) -> Option<NaiveDateTime> {
        let matches_pattern = regex::Regex::new(self.pattern).unwrap().is_match(text);
        if !matches_pattern {
            return None;
        }

        let completed_text = if self.completes_midnight {
            format!("{} 00:00:00", text)
        } else {
            text.to_string()
        };

        NaiveDateTime::parse_from_str(&completed_text, self.chrono_format).ok()
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Validation Error: Invalid time format found. {0} (expected: YYYY-MM-DD hh:mm:ss)")]
pub struct ConversionTimeParseError(pub String);

impl FromStr for ConversionTime {
    type Err = ConversionTimeParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        ACCEPTED_FORMATS
            .iter()
            .find_map(|format| format.parse(text))
            .map(ConversionTime)
            .ok_or_else(|| ConversionTimeParseError(text.to_string()))
    }
}

impl ConversionTime {
    pub fn naive_datetime(&self) -> NaiveDateTime {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn naive_datetime_of(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, mo, d)
            .unwrap()
            .and_hms_opt(h, mi, s)
            .unwrap()
    }

    #[test]
    fn parses_date_and_time_separated_by_space() {
        // Arrange
        let input = "2024-06-27 12:34:56";

        // Act
        let conversion_time: ConversionTime = input.parse().unwrap();

        // Assert
        assert_eq!(
            conversion_time.naive_datetime(),
            naive_datetime_of(2024, 6, 27, 12, 34, 56)
        );
    }

    #[test]
    fn parses_iso_8601_date_and_time() {
        // Arrange
        let input = "2024-06-27T12:34:56";

        // Act
        let conversion_time: ConversionTime = input.parse().unwrap();

        // Assert
        assert_eq!(
            conversion_time.naive_datetime(),
            naive_datetime_of(2024, 6, 27, 12, 34, 56)
        );
    }

    #[test]
    fn completes_date_only_input_with_midnight() {
        // Arrange
        let input = "2024-06-27";

        // Act
        let conversion_time: ConversionTime = input.parse().unwrap();

        // Assert
        assert_eq!(
            conversion_time.naive_datetime(),
            naive_datetime_of(2024, 6, 27, 0, 0, 0)
        );
    }

    #[test]
    fn rejects_unpadded_digits() {
        // Arrange
        let input = "2024-6-27";

        // Act
        let result: Result<ConversionTime, ConversionTimeParseError> = input.parse();

        // Assert
        assert_eq!(result, Err(ConversionTimeParseError(input.to_string())));
    }

    #[test]
    fn rejects_nonexistent_calendar_date() {
        // Arrange
        let input = "2024-02-30 12:00:00";

        // Act
        let result: Result<ConversionTime, ConversionTimeParseError> = input.parse();

        // Assert
        assert_eq!(result, Err(ConversionTimeParseError(input.to_string())));
    }

    #[test]
    fn rejects_empty_string() {
        // Arrange
        let input = "";

        // Act
        let result: Result<ConversionTime, ConversionTimeParseError> = input.parse();

        // Assert
        assert_eq!(result, Err(ConversionTimeParseError(input.to_string())));
    }
}
