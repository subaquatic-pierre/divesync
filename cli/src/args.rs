use clap::{arg, value_parser, Arg, ArgAction, Command};

pub fn build_depth_arg() -> Arg {
    Arg::new("depth")
        .short('d')
        .long("depth")
        .action(ArgAction::Set)
        .value_name("depth")
        .help("Dive depth in meters")
        .value_parser(value_parser!(f32))
        .required(true)
}

pub fn build_time_arg() -> Arg {
    Arg::new("time")
        .short('t')
        .long("time")
        .action(ArgAction::Set)
        .value_name("time")
        .help("Dive time in minutes")
        .value_parser(value_parser!(i32))
        .required(true)
}

pub fn build_algo_arg() -> Arg {
    Arg::new("algo")
        .required(true)
        .short('a')
        .long("algorithm")
        .action(ArgAction::Set)
        .value_name("algo")
        .required(true)
        .help("Decompression algorithm (dsat, zhl-16, etc.)")
}
