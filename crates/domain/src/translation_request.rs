use crate::ambiguous_time_strategy::AmbiguousTimeStrategy;
use crate::conversion_time::ConversionTime;
use crate::source_timezone::SourceTimezone;
use crate::target_timezone::TargetTimezone;
use chrono::NaiveDateTime;
use chrono_tz::Tz;

/// A complete, validated request to translate a time between timezones.
///
/// Every field is a value object, so an instance can only exist
/// when all inputs were valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TranslationRequest {
    time: ConversionTime,
    source: SourceTimezone,
    target: TargetTimezone,
    strategy: AmbiguousTimeStrategy,
}

impl TranslationRequest {
    pub fn new(
        time: ConversionTime,
        source: SourceTimezone,
        target: TargetTimezone,
        strategy: AmbiguousTimeStrategy,
    ) -> Self {
        Self {
            time,
            source,
            target,
            strategy,
        }
    }

    pub fn naive_datetime(&self) -> NaiveDateTime {
        self.time.naive_datetime()
    }

    pub fn source_timezone(&self) -> Tz {
        self.source.timezone()
    }

    pub fn target_timezone(&self) -> Tz {
        self.target.timezone()
    }

    pub fn strategy(&self) -> AmbiguousTimeStrategy {
        self.strategy
    }
}
