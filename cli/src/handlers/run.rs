use std::error::Error;

use clap::ArgMatches;

use core::{
    algorithm::get_algo,
    gas::{GasMix, PPO2},
    profile::{DiveProfile, DiveProfileLevel},
    runner::AlgorithmRunner,
};

use crate::{plotter::CliPlotter, utils::str_to_gas};

pub fn handle_run_cmd(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let depth = args.get_one::<f32>("depth").expect("depth is required");

    let time = args.get_one::<u32>("time").expect("time is required");
    let interval = match args.get_one::<u32>("interval") {
        Some(&num) => num,
        None => 5,
    };

    let gas = match args.get_one::<String>("gas") {
        Some(txt) => str_to_gas(txt)?,
        None => GasMix::new_nitrox(0.21),
    };

    let algo = match args.get_one::<String>("algo") {
        Some(txt) => get_algo(txt)?,
        None => get_algo("zhl16-a")?,
    };

    let mut profile = DiveProfile::new();
    profile.add_level(*depth, *time, gas);

    let mut runner = AlgorithmRunner::new(algo);
    runner.run(interval, profile);

    // println!("{:?}", runner.result());

    if let Some(val) = args.get_one::<String>("plot") {
        let plotter = CliPlotter::new();
        plotter.plot()?;
    }

    Ok(())
}
