use super::command_definition::command_provider;
use clap::ArgMatches;

pub(crate) fn receive_user_input() -> ArgMatches {
    command_provider().get_matches()
}
