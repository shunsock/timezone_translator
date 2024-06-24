use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

pub(crate) struct Converter {
    time: NaiveDateTime,
    from_tz: Tz,
    to_tz: Tz,
}

impl Converter {
    pub(crate) fn new(time: NaiveDateTime, from_tz: Tz, to_tz: Tz) -> Self {
        Self {
            time,
            from_tz,
            to_tz,
        }
    }

    pub(crate) fn convert(&self) -> Result<DateTime<Tz>, String> {
        // Extract the time from the `time` field with `from_tz` field
        let from_time_with_timezone : Option<DateTime<Tz>> = self.from_tz
            .from_local_datetime(&self.time)
            .single();

        // Check if the time exists in the timezone
        return match from_time_with_timezone.is_none() {
            // Return an error if the time does not exist in the timezone
            true => {
                Err(
                    format!(
                        "Converter Error: {} does not exist in timezone {}",
                        self.time, self.from_tz
                    )
                )
            }
            // Convert the time to the `to_tz` timezone
            false => Ok(from_time_with_timezone.unwrap().with_timezone(&self.to_tz)),
        }
    }
}