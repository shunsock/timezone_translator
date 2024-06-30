use super::translation_error::TranslationError;
use chrono::{
    DateTime,
    MappedLocalTime,
    NaiveDateTime,
    LocalResult,
    TimeZone
};
use chrono_tz::Tz;

pub(crate) struct TimezoneTranslator {
    time: NaiveDateTime,
    from_tz: Tz,
    to_tz: Tz,
}

impl TimezoneTranslator {
    pub(crate) fn new(time: NaiveDateTime, from_tz: Tz, to_tz: Tz) -> Self {
        Self {
            time,
            from_tz,
            to_tz,
        }
    }

    pub(crate) fn convert(&self) -> Result<DateTime<Tz>, TranslationError> {
        // Extract the time from the `time` field with `from_tz` field
        let mapped: MappedLocalTime<DateTime<Tz>> = self.from_tz.from_local_datetime(&self.time);

        match mapped {
            LocalResult::Single(time) => Ok(time.with_timezone(&self.to_tz)),
            LocalResult::Ambiguous(time, _) => Ok(time.with_timezone(&self.to_tz)),
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

        let translator: TimezoneTranslator = TimezoneTranslator::new(time, from_tz, to_tz);

        assert_eq!(translator.time, time);
        assert_eq!(translator.from_tz, from_tz);
        assert_eq!(translator.to_tz, to_tz);
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

        // expected result
        // +4 hours from America/New_York to UTC
        let expected_time = Utc
            .with_ymd_and_hms(2024, 6, 27, 16, 0, 0)
            .unwrap()
            .with_timezone(&to_tz);

        // calculate the actual result
        let translator: TimezoneTranslator = TimezoneTranslator::new(time, from_tz, to_tz);
        let actual_converted_time: Result<DateTime<Tz>, TranslationError> = translator.convert();

        assert!(actual_converted_time.is_ok());

        // confirm if the actual result is the same as the expected result
        assert_eq!(actual_converted_time.unwrap(), expected_time);
    }
}
