use clap::{arg, value_parser, Arg, ArgAction, Command};

use crate::args::{build_algo_arg, build_depth_arg};

pub fn build_ndl_command() -> Command {
    Command::new("ndl")
        .about("Compute no decompression limits")
        .arg(build_depth_arg())
        .arg(build_algo_arg())
}

pub fn build_deco_command() -> Command {
    Command::new("deco")
        .about("Compute deco stops")
        .arg(arg!([NAME]))
}
