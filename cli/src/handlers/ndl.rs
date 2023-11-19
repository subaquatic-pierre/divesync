use std::error::Error;

use clap::ArgMatches;

use core::{
    algorithm::get_algo,
    gas::{GasMix, PPO2},
    profile::{DiveProfile, DiveProfileLevel},
};

pub fn handle_ndl_cmd(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let depth = args.get_one::<f32>("depth").expect("depth is required");
    let algo = args
        .get_one::<String>("algo")
        .expect("algorithm is required");

    let algo = get_algo(algo)?;

    let profile = DiveProfile {
        levels: vec![{
            DiveProfileLevel {
                depth: *depth,
                time: 0,
                gas_mix: GasMix::new_nitrox(PPO2),
            }
        }],
    };

    let ndl = algo.compute_ndl(profile);

    println!(
        "No decompression limit for depth: {depth}m is {ndl}, with algorithm: {}",
        algo.variant()
    );
    Ok(())
}
