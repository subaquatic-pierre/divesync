mod cli;
use cli::init;

use core::{algorithm::get_algo, profile::DiveProfile};

fn main() {
    let cmd = init();

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("ndl", sub_matches)) => {
            let depth = sub_matches
                .get_one::<f64>("depth")
                .expect("depth is required");
            let algo = sub_matches
                .get_one::<String>("algo")
                .expect("algorithm is required");

            let algo = get_algo(algo);

            if algo.is_none() {
                println!("algorithm not found")
            }

            let algo = algo.unwrap();
            let profile = DiveProfile {
                depth: *depth,
                time: 0,
            };

            let ndl = algo.compute_ndl(profile);

            println!(
                "No decompression limit for depth: {depth}m is {ndl}, with algorithm: {}",
                algo.name()
            );
        }
        Some(("deco", _sub_matches)) => {
            println!("{matches:?}");
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
