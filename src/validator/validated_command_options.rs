use chrono::NaiveDateTime;
use chrono_tz::Tz;

pub(crate) struct ValidatedCommandOptions {
    time: NaiveDateTime,
    from_tz: Tz,
    to_tz: Tz,
}

impl ValidatedCommandOptions {
    pub fn new(time: NaiveDateTime, from_tz: Tz, to_tz: Tz) -> Self {
        Self {
            time,
            from_tz,
            to_tz,
        }
    }

    pub(crate) fn time(&self) -> NaiveDateTime {
        self.time
    }

    pub(crate) fn from_tz(&self) -> Tz {
        self.from_tz
    }

    pub(crate) fn to_tz(&self) -> Tz {
        self.to_tz
    }
}
