use chrono::{DateTime, NaiveDateTime, TimeZone};
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

    pub(crate) fn convert(&self) -> DateTime<Tz> {
        // Extract the time from the `time` field with `from_tz` field
        self.from_tz
            .from_local_datetime(&self.time)
            .single()
            .with_timezone(&self.from_tz);
    }
}