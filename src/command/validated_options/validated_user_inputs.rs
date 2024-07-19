use crate::command::validated_options::ambiguous_time_strategy::AmbiguousTimeStrategy;
use chrono::NaiveDateTime;
use chrono_tz::Tz;

pub(crate) struct ValidatedCommandOptions {
    time: NaiveDateTime,
    from_tz: Tz,
    to_tz: Tz,
    ambiguous_time_strategy: AmbiguousTimeStrategy,
}

impl ValidatedCommandOptions {
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

    pub(crate) fn get_param_time(&self) -> NaiveDateTime {
        self.time
    }

    pub(crate) fn get_param_from_tz(&self) -> Tz {
        self.from_tz
    }

    pub(crate) fn get_param_to_tz(&self) -> Tz {
        self.to_tz
    }

    pub(crate) fn ambiguous_time_strategy(&self) -> AmbiguousTimeStrategy {
        self.ambiguous_time_strategy
    }
}
