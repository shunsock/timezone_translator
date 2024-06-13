use chrono::{NaiveDateTime, ParseResult};

pub(super) fn validate_string_for_native_datetime(time: &str) -> Result<NaiveDateTime, String> {
    let datetime: ParseResult<NaiveDateTime> = NaiveDateTime::parse_from_str(
        time,
        "%Y-%m-%d %H:%M:%S"
    );

    match datetime {
        Ok(dt) => Ok(dt),
        Err(_) => Err(String::from("Validation Error: Invalid time format")),
    }
}