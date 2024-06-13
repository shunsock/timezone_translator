use chrono::{NaiveDateTime, ParseResult};
use chrono_tz::Tz;
use clap::ArgMatches;

fn validate_time(time: &str) -> Result<NaiveDateTime, String> {
    let datetime: ParseResult<NaiveDateTime> = NaiveDateTime::parse_from_str(
        time,
        "%Y-%m-%d %H:%M:%S"
    );

    match datetime {
        Ok(dt) => Ok(dt),
        Err(_) => Err(String::from("Validation Error: Invalid time format")),
    }
}

fn validate_timezone(tz: &str) -> Result<Tz, String> {
    tz.parse::<Tz>().map_err(
        |_| String::from("Validation Error: Invalid timezone")
    )
}
pub(crate) struct Validator {
    time: NaiveDateTime,
    from_tz: Tz,
    to_tz: Tz,
}

impl Validator {
    fn new(time: NaiveDateTime, from_tz: Tz, to_tz: Tz) -> Self {
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

pub(crate) fn command_validator(arg: &ArgMatches) -> Result<Validator, String> {
    let time_str = match arg.get_one::<String>("time") {
        Some(time) => time,
        None => return Err(String::from("Time is required")),
    };

    let time_validated = match validate_time(&time_str) {
        Ok(time) => time,
        Err(e) => {
            eprintln!("{}", e);
            return Err(String::from("Invalid time"));
        }
    };

    let from_tz_str = match arg.get_one::<String>("from_timezone") {
        Some(tz) => tz,
        None => return Err(String::from("From timezone is required")),
    };

    let from_tz_validated = match validate_timezone(&from_tz_str) {
        Ok(from_timezone) => from_timezone,
        Err(e) => {
            eprintln!("{}", e);
            return Err(String::from("Invalid from timezone"));
        }
    };

    let to_tz_str = match arg.get_one::<String>("to_timezone") {
        Some(tz) => tz,
        None => return Err(String::from("To timezone is required")),
    };

    let to_tz_validated: Tz = match validate_timezone(&to_tz_str) {
        Ok(to_timezone) => to_timezone,
        Err(e) => {
            eprintln!("{}", e);
            return Err(String::from("Invalid to timezone"));
        }
    };

    Ok(
        Validator::new(
            time_validated,
            from_tz_validated,
            to_tz_validated,
        )
    )
}
