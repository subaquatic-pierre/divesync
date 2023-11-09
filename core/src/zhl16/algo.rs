use crate::{
    algorithm::DecoAlgorithm,
    deco::DecoStop,
    gas::{self, GasMix},
    profile::DiveProfile,
    tissue::TissueCompartment,
    utils::calc_ata,
    zhl16::tissue::ZHL16Compartment,
};

pub struct ZHL16Algorithm {
    tissues: Vec<ZHL16Compartment>,
}

impl DecoAlgorithm for ZHL16Algorithm {
    fn name(&self) -> &str {
        "ZHL16"
    }

    fn compute_deco_stops(&self, dive_profile: DiveProfile) -> Vec<DecoStop> {
        vec![]
    }

    fn compute_ndl(&self, dive_profile: DiveProfile) -> u32 {
        42
    }
}

impl ZHL16Algorithm {
    pub fn new() -> Self {
        Self { tissues: vec![] }
    }

    pub fn tissues(&self) -> Vec<ZHL16Compartment> {
        self.tissues.clone()
    }

    pub fn init(&mut self, mix: GasMix) {
        let mut tissues: Vec<ZHL16Compartment> = vec![];

        for i in 0..16 {
            let t = ZHL16Compartment::new(i, mix.clone(), None);
            tissues.push(t)
        }

        self.tissues = tissues;
    }
}

pub struct ZHL16AlgorithmRunner {
    algo: ZHL16Algorithm,
}

impl ZHL16AlgorithmRunner {
    pub fn new(algo: ZHL16Algorithm) -> Self {
        Self { algo }
    }

    /// Run the algorithm given profile
    /// end result returns resultant TissueCompartments
    pub fn run(&mut self, dive_profile: DiveProfile) -> Vec<ZHL16Compartment> {
        if self.algo.tissues().len() == 0 && dive_profile.levels.len() > 0 {
            self.algo.init(dive_profile.levels[0].gas_mix.clone())
        }

        for level in dive_profile.levels {
            for t in &mut self.algo.tissues {
                t.set_gas_mix(level.gas_mix.clone());
                t.update_pressure(calc_ata(level.depth), level.time as f32)
            }
        }

        self.algo.tissues()
    }
}
