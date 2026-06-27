//! End-to-end tests of the `tzt` binary.
//!
//! These tests run the real binary with real timezone data:
//! no mocks, no internal APIs.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;

/// Fixture: the `tzt` binary under test.
fn tzt() -> Command {
    cargo_bin_cmd!("tzt")
}

#[test]
fn converts_time_from_new_york_to_utc() {
    // Arrange
    let mut cmd = tzt();
    cmd.args([
        "-T",
        "2024-01-01 12:00:00",
        "-f",
        "America/New_York",
        "-t",
        "UTC",
    ]);

    // Act & Assert
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2024-01-01 17:00:00 UTC"));
}

#[test]
fn falls_back_to_local_timezone_when_no_timezone_given() {
    // Arrange
    let mut cmd = tzt();
    cmd.args(["-T", "2024-01-01 12:00:00"]);

    // Act & Assert
    // source and target default to the same local timezone,
    // so the wall-clock time must not change
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2024-01-01 12:00:00"));
}

#[test]
fn prints_target_timezone_abbreviation() {
    // Arrange
    let mut cmd = tzt();
    cmd.args(["-T", "2024-01-01 12:00:00", "-f", "UTC", "-t", "Asia/Tokyo"]);

    // Act & Assert
    // the output must carry the target timezone (JST),
    // not just echo the input timezone information
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2024-01-01 21:00:00 JST"));
}

#[test]
fn accepts_iso_8601_input() {
    // Arrange
    let mut cmd = tzt();
    cmd.args([
        "-T",
        "2024-01-01T12:00:00",
        "-f",
        "America/New_York",
        "-t",
        "UTC",
    ]);

    // Act & Assert
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2024-01-01 17:00:00 UTC"));
}

#[test]
fn selects_first_occurrence_of_ambiguous_time_by_default() {
    // Arrange
    // 01:30 on 2024-11-03 occurs twice in New York (DST ends);
    // the default strategy (earliest) must pick 05:30 UTC
    let mut cmd = tzt();
    cmd.args([
        "--time",
        "2024-11-03 01:30:00",
        "--from",
        "America/New_York",
        "--to",
        "UTC",
    ]);

    // Act & Assert
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2024-11-03 05:30:00 UTC"));
}

#[test]
fn selects_second_occurrence_of_ambiguous_time_with_latest_strategy() {
    // Arrange
    // same ambiguous input as above; "latest" must pick 06:30 UTC
    let mut cmd = tzt();
    cmd.args([
        "--time",
        "2024-11-03 01:30:00",
        "--from",
        "America/New_York",
        "--to",
        "UTC",
        "--ambiguous-time-strategy",
        "latest",
    ]);

    // Act & Assert
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2024-11-03 06:30:00 UTC"));
}

#[test]
fn fails_with_invalid_source_timezone() {
    // Arrange
    let mut cmd = tzt();
    cmd.args(["-T", "2024-01-01 12:00:00", "-f", "NOT_EXIST", "-t", "UTC"]);

    // Act & Assert
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Validation Error"));
}

#[test]
fn fails_with_invalid_target_timezone() {
    // Arrange
    let mut cmd = tzt();
    cmd.args(["-T", "2024-01-01 12:00:00", "-f", "UTC", "-t", "NOT_EXIST"]);

    // Act & Assert
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Validation Error"));
}

#[test]
fn fails_when_time_does_not_exist_due_to_dst_gap() {
    // Arrange
    // 02:30 on 2024-03-10 does not exist in New York:
    // clocks jump from 02:00 to 03:00 when DST starts
    let mut cmd = tzt();
    cmd.args([
        "--time",
        "2024-03-10 02:30:00",
        "--from",
        "America/New_York",
        "--to",
        "America/Los_Angeles",
    ]);

    // Act & Assert
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Translation Error"));
}
