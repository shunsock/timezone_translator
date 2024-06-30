use chrono::NaiveDateTime;
use chrono_tz::Tz;
use crate::validator::ambiguous_time_strategy::AmbiguousTimeStrategy;

pub(crate) struct ValidatedCommandOptions {
    time: NaiveDateTime,
    from_tz: Tz,
    to_tz: Tz,
    ambiguous_time_strategy: AmbiguousTimeStrategy,
}

impl ValidatedCommandOptions {
    pub fn new(
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

    pub(crate) fn time(&self) -> NaiveDateTime {
        self.time
    }

    pub(crate) fn from_tz(&self) -> Tz {
        self.from_tz
    }

    pub(crate) fn to_tz(&self) -> Tz {
        self.to_tz
    }

    pub(crate) fn ambiguous_time_strategy(&self) -> AmbiguousTimeStrategy {
        self.ambiguous_time_strategy.clone()
    }
}
