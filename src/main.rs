mod command;
mod infrastructure;
mod translator;
mod validator;

use chrono::prelude::*;
use chrono_tz::Tz;
use clap::ArgMatches;
use command::receiver::receive_user_input;
use command::validated_options::validated_user_inputs::ValidatedCommandOptions;
use std::process::exit;
use translator::translation_error::TranslationError;
use translator::TimezoneTranslator;
use validator::command_options_validator::validate_command_options;

fn main() {
    let user_input_options: ArgMatches = receive_user_input();

    let validated_options: ValidatedCommandOptions =
        match validate_command_options(&user_input_options) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        };

    let date_time_mapped: Result<DateTime<Tz>, TranslationError> = TimezoneTranslator::new(
        validated_options.get_param_time(),
        validated_options.get_param_from_tz(),
        validated_options.get_param_to_tz(),
        validated_options.ambiguous_time_strategy(),
    )
    .convert();

    match date_time_mapped {
        Ok(mapped) => {
            println!("{}", mapped);
            exit(0);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    /// This test verifies that the program can correctly convert a given time
    /// from one timezone (America/New_York) to another (UTC).
    #[test]
    fn converts_time_from_new_york_to_utc() {
        let mut cmd = Command::cargo_bin("tzt").unwrap();
        cmd.args(&[
            "-T",
            "2024-01-01 12:00:00",
            "-f",
            "America/New_York",
            "-t",
            "UTC",
        ]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("2024-01-01 17:00:00 UTC"));
    }

    #[test]
    fn converts_time_with_no_timezone() {
        let mut cmd = Command::cargo_bin("tzt").unwrap();
        cmd.args(&["-T", "2024-01-01 12:00:00"]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("2024-01-01 12:00:00"));
    }

    /// This test verifies that the program correctly converts the given time
    /// from UTC to the specified timezone (Asia/Tokyo) and returns the
    /// corresponding standard time.
    ///
    /// Specifically, it checks that the output is not just the input timezone information,
    /// but properly represents the standard time in the target timezone (JST in this case).
    #[test]
    fn converts_time_from_utc_to_tokyo() {
        let mut cmd = Command::cargo_bin("tzt").unwrap();
        cmd.args(&["-T", "2024-01-01 12:00:00", "-f", "UTC", "-t", "Asia/Tokyo"]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("2024-01-01 21:00:00 JST"));
    }

    /// This test verifies that the program can correctly handle input provided
    /// in the ISO 8601 format. ISO 8601 is an international standard for
    /// date and time representation, and it typically takes the form
    /// "YYYY-MM-DDTHH:MM:SS" (e.g., "2024-01-01T12:00:00").
    ///
    /// The test ensures that the program accurately parses the date and time
    /// from this format and converts it to the specified target timezone (UTC).
    ///
    /// For more details about ISO 8601 format, refer to the
    /// [Wikipedia article](https://en.wikipedia.org/wiki/ISO_8601).
    #[test]
    fn handles_iso_8601_format() {
        let mut cmd = Command::cargo_bin("tzt").unwrap();
        cmd.args(&[
            "-T",
            "2024-01-01T12:00:00",
            "-f",
            "America/New_York",
            "-t",
            "UTC",
        ]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("2024-01-01 17:00:00 UTC"));
    }

    /// This test checks the program's handling of ambiguous times caused by
    /// daylight saving time (DST) changes. When DST ends, clocks are set back
    /// one hour, resulting in a repeated hour that can be ambiguous.
    /// This test uses the "earliest" strategy, which means that the program
    /// should select the first occurrence of the ambiguous time.
    ///
    /// For example, on November 3, 2024, in the "America/New_York" timezone,
    /// the time "01:30:00" can occur twice—once before the DST ends and once after.
    /// This test ensures that the program correctly handles this situation
    /// by selecting the earlier occurrence of "01:30:00".
    #[test]
    fn handles_ambiguous_time_with_earliest_strategy() {
        let mut cmd = Command::cargo_bin("tzt").unwrap();
        cmd.args(&[
            "--time",
            "2024-11-03 01:30:00",
            "--from",
            "America/New_York",
            "--to",
            "UTC",
        ]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("2024-11-03 05:30:00 UTC"));
    }

    /// This test verifies the program's handling of ambiguous times due to
    /// daylight saving time (DST) changes, specifically using the "latest" strategy.
    /// When DST ends and clocks are set back one hour, an ambiguous time can occur
    /// twice. This strategy ensures that the program selects the second occurrence
    /// of the ambiguous time.
    ///
    /// For example, on November 3, 2024, in the "America/New_York" timezone,
    /// the time "01:30:00" occurs twice. The first occurrence happens during
    /// DST, and the second occurrence happens after DST ends. This test confirms
    /// that the program correctly handles this situation by selecting the later
    /// occurrence of "01:30:00".
    #[test]
    fn handles_ambiguous_time_with_latest_strategy() {
        let mut cmd = Command::cargo_bin("tzt").unwrap();
        cmd.args(&[
            "--time",
            "2024-11-03 01:30:00",
            "--from",
            "America/New_York",
            "--to",
            "UTC",
            "--ambiguous-time-strategy",
            "latest",
        ]);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("2024-11-03 06:30:00 UTC"));
    }

    /// This test validates that the program correctly identifies
    /// and reports an error when an invalid "from" timezone is provided.
    #[test]
    fn fails_with_invalid_from_timezone() {
        let mut cmd = Command::cargo_bin("tzt").unwrap();
        cmd.args(&["-T", "2024-01-01 12:00:00", "-f", "NOT_EXIST", "-t", "UTC"]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Validation Error"));
    }

    /// This test checks that the program correctly identifies
    /// and reports an error when an invalid "to" timezone is provided.
    #[test]
    fn fails_with_invalid_to_timezone() {
        let mut cmd = Command::cargo_bin("tzt").unwrap();
        cmd.args(&["-T", "2024-01-01 12:00:00", "-f", "UTC", "-t", "NOT_EXIST"]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Validation Error"));
    }

    /// This test verifies that the program correctly handles cases where the resulting
    /// time does not exist due to daylight saving time (DST) transitions. Specifically,
    /// during the spring transition, clocks are set forward one hour, causing a gap in time
    /// where certain times do not exist.
    ///
    /// For example, on March 10, 2024, in the "America/New_York" timezone, the local time
    /// skips from 02:00:00 to 03:00:00, meaning that any time between 02:00:00 and 02:59:59
    /// does not exist on that day. This test checks that the program correctly identifies
    /// this scenario and returns an appropriate error, ensuring robust handling of such
    /// edge cases.
    ///
    /// The expected behavior is for the program to detect the non-existent time and
    /// provide a clear error message indicating the issue.
    #[test]
    fn fails_nonexistent_time_due_to_dst() {
        let mut cmd = Command::cargo_bin("tzt").unwrap();
        cmd.args(&[
            "--time",
            "2024-03-10 02:30:00",
            "--from",
            "America/New_York",
            "--to",
            "America/Los_Angeles",
        ]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Translation Error"));
    }
}
