use clap::{arg, value_parser, Arg, ArgAction, Command};

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

fn build_ndl_command() -> Command {
    let depth = Arg::new("depth")
        .short('d')
        .long("depth")
        .action(ArgAction::Set)
        .value_name("depth")
        .help("Dive depth in meters")
        .value_parser(value_parser!(f64))
        .required(true);

    let algo = Arg::new("algo")
        .required(true)
        .short('a')
        .long("algorithm")
        .action(ArgAction::Set)
        .value_name("algo")
        .required(true)
        .help("Decompression algorithm (dsat, zhl-16, etc.)");

    Command::new("ndl")
        .about("Compute no decompression limits")
        .arg(depth)
        .arg(algo)
}
fn build_deco_command() -> Command {
    Command::new("deco")
        .about("Compute deco stops")
        .arg(arg!([NAME]))
}
