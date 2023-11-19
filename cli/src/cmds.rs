use clap::{arg, Command};

use crate::args::{
    build_algo_arg, build_depth_arg, build_gas_arg, build_interval_arg, build_plot_arg,
    build_save_csv_arg, build_time_arg,
};

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

pub fn build_run_command() -> Command {
    Command::new("run")
        .about("Run a given dive profile")
        .arg(build_depth_arg())
        .arg(build_algo_arg())
        .arg(build_time_arg())
        .arg(build_save_csv_arg())
        .arg(build_plot_arg())
        .arg(build_gas_arg())
        .arg(build_interval_arg())
}
