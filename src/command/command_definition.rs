use super::arguments::{
    ambiguous_time_strategy::ambiguous_time_strategy, from::from, time::time, to::to,
};
use super::helper::local_timezone_string_provider::provide_local_timezone_string;
use clap::Command;

/// # About:
/// Provides the command definition for the `tzt` command.
///
/// # Returns:
/// `Command` struct containing the command definition.
///
/// # Example:
/// ```
/// use clap::ArgMatches;
/// use command::command_definition::command_provider;
/// let user_input: ArgMatches = command_provider().get_matches();
/// ```
pub(crate) fn command_provider() -> Command {
    let now: String = provide_local_timezone_string();
    let now_str: &'static str = Box::leak(now.into_boxed_str());

    Command::new("tzt - Timezone Translator")
        .version("0.2.0")
        .author("s.tsuchiya.business@gmail.com")
        .about("Converts time between time zones")
        .arg(time())
        .arg(from(now_str))
        .arg(to(now_str))
        .arg(ambiguous_time_strategy())
}
