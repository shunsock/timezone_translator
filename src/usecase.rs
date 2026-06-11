pub mod translation_error;

use self::translation_error::TranslationError;
use crate::domain::{AmbiguousTimeStrategy, TranslationRequest};
use chrono::{DateTime, LocalResult, MappedLocalTime, TimeZone};
use chrono_tz::Tz;

/// Translates the requested wall-clock time from its source timezone
/// into the target timezone.
pub struct TimezoneTranslator {
    request: TranslationRequest,
}

impl TimezoneTranslator {
    pub fn new(request: TranslationRequest) -> Self {
        Self { request }
    }

    /// Attaches the source timezone to the naive time, then converts
    /// to the target timezone.
    ///
    /// Returns `TranslationError` when the time does not exist in the
    /// source timezone (a DST gap).
    pub fn convert(&self) -> Result<DateTime<Tz>, TranslationError> {
        let mapped: MappedLocalTime<DateTime<Tz>> = self
            .request
            .source_timezone()
            .from_local_datetime(&self.request.naive_datetime());

        let time_in_source_timezone: DateTime<Tz> = match mapped {
            LocalResult::Single(time) => time,
            LocalResult::Ambiguous(earliest, latest) => match self.request.strategy() {
                AmbiguousTimeStrategy::Earliest => earliest,
                AmbiguousTimeStrategy::Latest => latest,
            },
            LocalResult::None => {
                return Err(TranslationError::NonexistentTime {
                    time: self.request.naive_datetime(),
                    from_tz: self.request.source_timezone(),
                    to_tz: self.request.target_timezone(),
                })
            }
        };

        Ok(time_in_source_timezone.with_timezone(&self.request.target_timezone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    /// Fixture: builds a `TranslationRequest` from raw strings.
    fn translation_request(
        time: &str,
        source: &str,
        target: &str,
        strategy: AmbiguousTimeStrategy,
    ) -> TranslationRequest {
        TranslationRequest::new(
            time.parse().unwrap(),
            source.parse().unwrap(),
            target.parse().unwrap(),
            strategy,
        )
    }

    fn utc_datetime(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> DateTime<Tz> {
        Utc.with_ymd_and_hms(y, mo, d, h, mi, s)
            .unwrap()
            .with_timezone(&chrono_tz::UTC)
    }

    #[test]
    fn converts_unambiguous_time_to_target_timezone() {
        // Arrange
        // New York is UTC-4 on 2024-06-27 (EDT)
        let request = translation_request(
            "2024-06-27 12:00:00",
            "America/New_York",
            "UTC",
            AmbiguousTimeStrategy::Earliest,
        );

        // Act
        let converted = TimezoneTranslator::new(request).convert();

        // Assert
        assert_eq!(converted.unwrap(), utc_datetime(2024, 6, 27, 16, 0, 0));
    }

    #[test]
    fn selects_first_occurrence_with_earliest_strategy() {
        // Arrange
        // 01:30 on 2024-11-03 occurs twice in New York (DST ends);
        // the first occurrence is 05:30 UTC
        let request = translation_request(
            "2024-11-03 01:30:00",
            "America/New_York",
            "UTC",
            AmbiguousTimeStrategy::Earliest,
        );

        // Act
        let converted = TimezoneTranslator::new(request).convert();

        // Assert
        assert_eq!(converted.unwrap(), utc_datetime(2024, 11, 3, 5, 30, 0));
    }

    #[test]
    fn selects_second_occurrence_with_latest_strategy() {
        // Arrange
        // 01:30 on 2024-11-03 occurs twice in New York (DST ends);
        // the second occurrence is 06:30 UTC
        let request = translation_request(
            "2024-11-03 01:30:00",
            "America/New_York",
            "UTC",
            AmbiguousTimeStrategy::Latest,
        );

        // Act
        let converted = TimezoneTranslator::new(request).convert();

        // Assert
        assert_eq!(converted.unwrap(), utc_datetime(2024, 11, 3, 6, 30, 0));
    }

    #[test]
    fn fails_when_time_does_not_exist_in_source_timezone() {
        // Arrange
        // 02:30 on 2024-03-10 does not exist in New York (DST gap)
        let request = translation_request(
            "2024-03-10 02:30:00",
            "America/New_York",
            "America/Los_Angeles",
            AmbiguousTimeStrategy::Latest,
        );

        // Act
        let converted = TimezoneTranslator::new(request).convert();

        // Assert
        assert!(converted.is_err());
    }
}
