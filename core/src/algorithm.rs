use crate::deco::DecoStop;
use crate::profile::DiveProfile;

use crate::dsat::algo::DSATAlgorithm;
use crate::tissue::TissueCompartment;
use crate::zhl16::algo::ZHL16Algorithm;

pub trait DecoAlgorithm {
    fn name(&self) -> &str;
    fn compute_deco_stops(&self, dive_profile: DiveProfile) -> Vec<DecoStop>;
    fn compute_ndl(&self, dive_profile: DiveProfile) -> u32;
}

pub fn get_algo(algo: &str) -> Option<Box<dyn DecoAlgorithm>> {
    match algo {
        "dsat" => Some(Box::new(DSATAlgorithm::new())),
        "zhl16" => Some(Box::new(ZHL16Algorithm::new())),
        _ => None,
    }
}
