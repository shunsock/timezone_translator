use super::command_definition::command_provider;
use clap::ArgMatches;

/// # About:
/// Receives user input from the command line and returns it as a `ArgMatches` struct.
/// This function is used to receive user input from the command line.
///
/// # Example:
/// ```
/// use clap::ArgMatches;
/// use command::receiver::receive_user_input;
/// let user_input: ArgMatches = receive_user_input();
/// ```
///
/// # Returns:
/// `ArgMatches` struct containing the user input.
///
/// # Note:
/// if you want to change command settings, see `command_definition` module.
pub(crate) fn receive_user_input() -> ArgMatches {
    command_provider().get_matches()
}
