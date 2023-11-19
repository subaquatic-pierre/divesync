use std::error::Error;

use clap::ArgMatches;

use core::{
    algorithm::get_algo,
    gas::{GasMix, PPO2},
    profile::{DiveProfile, DiveProfileLevel},
};

pub fn handle_deco_cmd(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // TODO: implement handle deco cmd
    Ok(())
}
