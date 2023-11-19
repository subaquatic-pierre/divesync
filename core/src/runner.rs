use csv::Writer;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::algorithm::DecoAlgorithm;
use crate::profile::DiveProfile;
use crate::tissue::CompartmentSnapshot;
use crate::utils::calc_ata;
use crate::utils::home_dir;
use crate::utils::timestamp;

#[derive(Clone, Debug)]
pub struct AlgorithmRunResult {
    pub interval_period: u32,
    pub snapshots: Vec<Vec<CompartmentSnapshot>>,
}

pub struct AlgorithmRunner {
    algo: Box<dyn DecoAlgorithm>,
    result: Option<AlgorithmRunResult>,
}

impl AlgorithmRunner {
    pub fn new(algo: Box<dyn DecoAlgorithm>) -> Self {
        Self { algo, result: None }
    }

    /// Run the algorithm given profile
    /// end result returns resultant TissueCompartments
    pub fn run(&mut self, interval_period: u32, dive_profile: DiveProfile) -> AlgorithmRunResult {
        let mut snapshots = vec![];

        // calculate number of interval periods in dive profile
        for level in dive_profile.levels {
            let steps = level.time / interval_period;
            let remainder = level.time % interval_period;

            for _ in 0..steps {
                self.algo.run(
                    level.gas_mix.clone(),
                    calc_ata(level.depth),
                    interval_period as f32,
                );

                snapshots.push(self.algo.snapshot());
            }

            if remainder > 0 {
                self.algo.run(
                    level.gas_mix.clone(),
                    calc_ata(level.depth),
                    remainder as f32,
                );

                snapshots.push(self.algo.snapshot());
            }
        }

        let result = AlgorithmRunResult {
            interval_period,
            snapshots,
        };

        self.result = Some(result.clone());

        result
    }

    pub fn save_results(&self) -> Result<String, Box<dyn Error>> {
        let ts: u64 = timestamp();
        let data_dir = Path::new(&home_dir())
            .join(".divesync")
            .join("data")
            .join(format!("{ts}"));

        fs::create_dir_all(&data_dir)?;
        let filename = data_dir.join("result.csv");

        let mut wtr = Writer::from_path(&filename)?;

        if let Some(result) = &self.result {
            for interval in &result.snapshots {
                for cpt in interval {
                    wtr.serialize(&cpt)?;
                }
            }

            wtr.flush()?;
        }

        Ok(filename.to_str().unwrap().to_string())
    }

    pub fn result(&self) -> Option<AlgorithmRunResult> {
        self.result.clone()
    }
}

mod test {
    use super::*;
    use crate::{
        gas::{GasMix, PPO2},
        zhl16::{algorithm::ZHL16Algorithm, tissue::ZHL16Variant},
    };

    #[test]
    fn test_algorithm_runner_steps() {
        let algo = ZHL16Algorithm::new(ZHL16Variant::A);
        let mut runner = AlgorithmRunner::new(Box::new(algo));
        let mut profile = DiveProfile::new();
        let mix = GasMix::new_nitrox(PPO2);
        profile.add_level(20.0, 20, mix);

        runner.run(3, profile);
        let res = runner.result();

        assert!(res.is_some());

        let result = res.unwrap();

        assert_eq!(result.snapshots.len(), 7);
    }

    #[test]
    fn test_algorithm_runner_elapsed_time() {
        let algo = ZHL16Algorithm::new(ZHL16Variant::A);
        let mut runner = AlgorithmRunner::new(Box::new(algo));
        let mut profile = DiveProfile::new();
        let mix = GasMix::new_nitrox(PPO2);
        profile.add_level(20.0, 38, mix);

        runner.run(7, profile);
        let res = runner.result();

        assert!(res.is_some());

        let result = res.unwrap();

        assert_eq!(result.snapshots[0][0].elapsed_time, 7.0);
        assert_eq!(result.snapshots.last().unwrap()[0].elapsed_time, 38.0);
    }

    #[test]
    fn test_algorithm_runner_save() {
        let algo = ZHL16Algorithm::new(ZHL16Variant::A);
        let mut runner = AlgorithmRunner::new(Box::new(algo));
        let mut profile = DiveProfile::new();
        let mix = GasMix::new_nitrox(PPO2);
        profile.add_level(24.0, 20, mix);

        runner.run(3, profile);

        let path = runner.save_results().unwrap();
        println!("path: {path}");
    }
}
