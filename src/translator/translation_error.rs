use chrono::NaiveDateTime;
use chrono_tz::Tz;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub(crate) enum TranslationError {
    #[error("Translation Error: Output time and timezone does not exist. Please check DST rules.")]
    TranslationError {
        time: NaiveDateTime,
        from_tz: Tz,
        to_tz: Tz,
    },
}
