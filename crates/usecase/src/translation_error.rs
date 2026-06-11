use chrono::NaiveDateTime;
use chrono_tz::Tz;

#[derive(thiserror::Error, Debug)]
pub enum TranslationError {
    /// The requested wall-clock time falls into a DST gap and never
    /// existed in the source timezone. The fields record the inputs
    /// for debugging; the message stays generic for the CLI user.
    #[error("Translation Error: Output time and timezone does not exist. Please check DST rules.")]
    NonexistentTime {
        time: NaiveDateTime,
        from_tz: Tz,
        to_tz: Tz,
    },
}
