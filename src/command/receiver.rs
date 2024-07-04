use super::command_definition::define_command;
use clap::ArgMatches;

pub(crate) fn receive_user_input() -> ArgMatches {
    define_command().get_matches()
}
