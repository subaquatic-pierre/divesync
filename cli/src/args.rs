use clap::{value_parser, Arg, ArgAction};

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
        .value_parser(value_parser!(u32))
        .required(true)
}

pub fn build_interval_arg() -> Arg {
    Arg::new("interval")
        .short('i')
        .long("interval")
        .action(ArgAction::Set)
        .value_name("interval")
        .help("Interval period at which to run algorithm, default value is 5min")
        .value_parser(value_parser!(u32))
        .default_value("5")
}

pub fn build_save_csv_arg() -> Arg {
    Arg::new("csv")
        .short('c')
        .long("csv")
        .action(ArgAction::Set)
        .value_name("csv")
        .help("Save the output to CSV")
}

pub fn build_gas_arg() -> Arg {
    Arg::new("gas")
        .short('g')
        .long("gas")
        .action(ArgAction::Set)
        .value_parser(value_parser!(String))
        .value_name("gas")
        .help("Gas mixture, oxygen and helium, in the format of -g 'O2%,He%'")
        .default_value("21,0")
}

pub fn build_plot_arg() -> Arg {
    Arg::new("plot")
        .short('p')
        .long("plot")
        .action(ArgAction::Set)
        .value_parser(value_parser!(String))
        .value_name("plot-value")
        .help("Plot values of an algorithm run over time")
}

pub fn build_algo_arg() -> Arg {
    Arg::new("algo")
        .short('a')
        .long("algorithm")
        .action(ArgAction::Set)
        .value_name("algo")
        .help("Decompression algorithm (DSAT, ZHL16-A, ZHL16-B, ZHL16-C)")
}
