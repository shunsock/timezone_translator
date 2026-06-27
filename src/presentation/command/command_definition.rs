use super::arguments::{
    ambiguous_time_strategy::ambiguous_time_strategy, from_timezone::from_timezone, time::time,
    to_timezone::to_timezone,
};
use crate::infrastructure::provide_local_timezone_string;
use clap::Command;

/// # About:
/// Provides the command definition for the `tzt` command.
///
/// # Returns:
/// `Command` struct containing the command definition.
///
/// # Example:
/// ```ignore
/// use clap::ArgMatches;
/// use command::command_definition::command_provider;
/// let user_input: ArgMatches = command_provider().get_matches();
/// ```
pub(crate) fn command_provider() -> Command {
    let local_timezone: String = provide_local_timezone_string();

    Command::new("tzt - Timezone Translator")
        .version(env!("CARGO_PKG_VERSION"))
        .author("shunsock")
        .about("translate time from one timezone to another")
        .arg(time())
        .arg(from_timezone(local_timezone.clone()))
        .arg(to_timezone(local_timezone))
        .arg(ambiguous_time_strategy())
}
