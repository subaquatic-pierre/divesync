use crate::{algorithm::DecoAlgorithm, deco::DecoStop, profile::DiveProfile};

pub struct ZHL16Algorithm;

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
        Self {}
    }
}
