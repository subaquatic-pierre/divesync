mod args;
mod cmds;
mod handlers;
mod init;
mod plotter;
mod utils;

use handlers::{deco::handle_deco_cmd, ndl::handle_ndl_cmd, run::handle_run_cmd};
use init::init;

use core::{
    algorithm::get_algo,
    gas::{GasMix, PPO2},
    profile::{DiveProfile, DiveProfileLevel},
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = init();

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("ndl", sub_matches)) => handle_ndl_cmd(sub_matches)?,
        Some(("deco", _sub_matches)) => {
            println!("{matches:?}");
        }
        Some(("run", sub_matches)) => handle_run_cmd(sub_matches)?,
        _ => unreachable!("clap should ensure we don't get here"),
    };
    Ok(())
}
