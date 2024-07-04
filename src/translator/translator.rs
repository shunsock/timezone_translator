use super::translation_error::TranslationError;
use crate::command::validated_options::ambiguous_time_strategy::AmbiguousTimeStrategy;
use chrono::{DateTime, LocalResult, MappedLocalTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

pub(crate) struct TimezoneTranslator {
    time: NaiveDateTime,
    from_tz: Tz,
    to_tz: Tz,
    ambiguous_time_strategy: AmbiguousTimeStrategy,
}

impl TimezoneTranslator {
    pub(crate) fn new(
        time: NaiveDateTime,
        from_tz: Tz,
        to_tz: Tz,
        ambiguous_time_strategy: AmbiguousTimeStrategy,
    ) -> Self {
        Self {
            time,
            from_tz,
            to_tz,
            ambiguous_time_strategy,
        }
    }

    pub(crate) fn convert(&self) -> Result<DateTime<Tz>, TranslationError> {
        // Extract the time from the `time` field with `from_tz` field
        let mapped: MappedLocalTime<DateTime<Tz>> = self.from_tz.from_local_datetime(&self.time);

        match mapped {
            LocalResult::Single(time) => Ok(time.with_timezone(&self.to_tz)),
            LocalResult::Ambiguous(time_earliest, time_latest) => {
                Ok(select_time_with_ambiguous_time_strategy(
                    self.ambiguous_time_strategy,
                    self.to_tz,
                    time_earliest,
                    time_latest,
                ))
            }
            LocalResult::None => {
                let error = TranslationError::TranslationError {
                    time: self.time,
                    from_tz: self.from_tz,
                    to_tz: self.to_tz,
                };
                Err(error)
            }
        }
    }
}

fn select_time_with_ambiguous_time_strategy(
    strategy: AmbiguousTimeStrategy,
    timezone: Tz,
    time_earliest: DateTime<Tz>,
    time_latest: DateTime<Tz>,
) -> DateTime<Tz> {
    match strategy {
        AmbiguousTimeStrategy::Earliest => time_earliest.with_timezone(&timezone),
        AmbiguousTimeStrategy::Latest => time_latest.with_timezone(&timezone),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};
    use chrono_tz::Tz;

    /// This test checks if the `TimezoneTranslator` struct is created correctly.
    /// It checks if the `time`, `from_tz`, and `to_tz` fields are set correctly.
    /// expected: `TimezoneTranslator`
    #[test]
    fn test_new() {
        let date: NaiveDate = NaiveDate::from_ymd_opt(2024, 6, 27).unwrap();
        let time: NaiveDateTime = date.and_hms_opt(12, 0, 0).unwrap();
        let from_tz: Tz = "America/New_York".parse().unwrap();
        let to_tz: Tz = "Europe/London".parse().unwrap();
        let ambiguous_time_strategy: AmbiguousTimeStrategy = AmbiguousTimeStrategy::Earliest;

        let translator: TimezoneTranslator =
            TimezoneTranslator::new(time, from_tz, to_tz, ambiguous_time_strategy);

        assert_eq!(translator.time, time);
        assert_eq!(translator.from_tz, from_tz);
        assert_eq!(translator.to_tz, to_tz);
        assert_eq!(translator.ambiguous_time_strategy, ambiguous_time_strategy);
    }

    /// This test checks if the `convert` method works correctly.
    /// It checks if the method returns a `DateTime<Tz>` object.
    /// expected: `DateTime<Tz>`
    #[test]
    fn test_convert() {
        // input for the test
        let date: NaiveDate = NaiveDate::from_ymd_opt(2024, 6, 27).unwrap();
        let time: NaiveDateTime = date.and_hms_opt(12, 0, 0).unwrap();
        let from_tz: Tz = "America/New_York".parse().unwrap();
        let to_tz: Tz = "UTC".parse().unwrap();
        let ambiguous_time_strategy: AmbiguousTimeStrategy = AmbiguousTimeStrategy::Earliest;

        // expected result
        // +4 hours from America/New_York to UTC
        let expected_time = Utc
            .with_ymd_and_hms(2024, 6, 27, 16, 0, 0)
            .unwrap()
            .with_timezone(&to_tz);

        // calculate the actual result
        let translator: TimezoneTranslator =
            TimezoneTranslator::new(time, from_tz, to_tz, ambiguous_time_strategy);
        let actual_converted_time: Result<DateTime<Tz>, TranslationError> = translator.convert();

        assert!(actual_converted_time.is_ok());

        // confirm if the actual result is the same as the expected result
        assert_eq!(actual_converted_time.unwrap(), expected_time);
    }

    /// This test check option `AmbiguousTimeStrategy::Earliest` works correctly.
    /// American/New_York is UTC-4 but `2024-11-03 01:30:00` is ambiguous. (after DST ends)
    /// `2024-11-03 05:30:00` is the earliest time.
    /// expected: `DateTime<Tz>`
    #[test]
    fn test_earliest_ambiguous_time_strategy() {
        // input for the test
        let date: NaiveDate = NaiveDate::from_ymd_opt(2024, 11, 03).unwrap();
        let time: NaiveDateTime = date.and_hms_opt(01, 30, 0).unwrap();
        let from_tz: Tz = "America/New_York".parse().unwrap();
        let to_tz: Tz = "UTC".parse().unwrap();
        let ambiguous_time_strategy: AmbiguousTimeStrategy = AmbiguousTimeStrategy::Earliest;

        // expected result
        // +4, +5 hours from America/New_York to UTC (DST ends)
        // in this case, the earliest time is 5:30
        let expected_time = Utc
            .with_ymd_and_hms(2024, 11, 03, 5, 30, 0)
            .unwrap()
            .with_timezone(&to_tz);

        // calculate the actual result
        let translator: TimezoneTranslator =
            TimezoneTranslator::new(time, from_tz, to_tz, ambiguous_time_strategy);
        let actual_converted_time: Result<DateTime<Tz>, TranslationError> = translator.convert();

        assert!(actual_converted_time.is_ok());

        // confirm if the actual result is the same as the expected result
        assert_eq!(actual_converted_time.unwrap(), expected_time);
    }

    /// This test check option `AmbiguousTimeStrategy::Latest` works correctly.
    /// American/New_York is UTC-4 but `2024-11-03 01:30:00` is ambiguous. (after DST ends)
    /// `2024-11-03 06:30:00` is the latest time.
    /// expected: `DateTime<Tz>`
    #[test]
    fn test_latest_ambiguous_time_strategy() {
        // input for the test
        let date: NaiveDate = NaiveDate::from_ymd_opt(2024, 11, 03).unwrap();
        let time: NaiveDateTime = date.and_hms_opt(01, 30, 0).unwrap();
        let from_tz: Tz = "America/New_York".parse().unwrap();
        let to_tz: Tz = "UTC".parse().unwrap();
        let ambiguous_time_strategy: AmbiguousTimeStrategy = AmbiguousTimeStrategy::Latest;

        // expected result
        // +4, +5 hours from America/New_York to UTC (DST ends)
        // in this case, the latest time is 6:30
        let expected_time = Utc
            .with_ymd_and_hms(2024, 11, 03, 6, 30, 0)
            .unwrap()
            .with_timezone(&to_tz);

        // calculate the actual result
        let translator: TimezoneTranslator =
            TimezoneTranslator::new(time, from_tz, to_tz, ambiguous_time_strategy);
        let actual_converted_time: Result<DateTime<Tz>, TranslationError> = translator.convert();

        assert!(actual_converted_time.is_ok());

        // confirm if the actual result is the same as the expected result
        assert_eq!(actual_converted_time.unwrap(), expected_time);
    }

    /// This test checks if the `convert` method returns an error when the output time does not exist.
    /// expected: `TranslationError`
    #[test]
    fn test_output_timestamp_does_not_exist() {
        // input for the test
        let date: NaiveDate = NaiveDate::from_ymd_opt(2024, 03, 10).unwrap();
        let time: NaiveDateTime = date.and_hms_opt(02, 30, 0).unwrap();
        let from_tz: Tz = "America/New_York".parse().unwrap();
        let to_tz: Tz = "America/Los_Angeles".parse().unwrap();
        let ambiguous_time_strategy: AmbiguousTimeStrategy = AmbiguousTimeStrategy::Latest;

        // calculate the actual result
        let translator: TimezoneTranslator =
            TimezoneTranslator::new(time, from_tz, to_tz, ambiguous_time_strategy);
        let actual_converted_time: Result<DateTime<Tz>, TranslationError> = translator.convert();

        // check result is error
        assert!(actual_converted_time.is_err());
    }
}
