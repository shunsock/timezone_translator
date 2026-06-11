mod domain;
mod infrastructure;
mod presentation;
mod usecase;

use std::process::ExitCode;

fn main() -> ExitCode {
    presentation::run()
}
