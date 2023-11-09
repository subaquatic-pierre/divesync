use clap::{arg, value_parser, Arg, ArgAction, Command};

use crate::cmds::{build_deco_command, build_ndl_command};

pub fn init() -> Command {
    Command::new("DiveSync")
        .version("1.0")
        .author("Pierre du Toit K. <subaquatic.pierre@gmail.com>")
        .about("Command line utility to calculate all things deco")
        .long_about("Command line utility to calculate all things deco")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(build_ndl_command())
        .subcommand(build_deco_command())
}
