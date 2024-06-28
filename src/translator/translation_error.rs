use chrono::NaiveDateTime;
use chrono_tz::Tz;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub(crate) enum TranslationError {
    #[error("Translation Error: {time} from {from_tz} to {to_tz}")]
    TranslationError {
        time: NaiveDateTime,
        from_tz: Tz,
        to_tz: Tz,
    },
}
