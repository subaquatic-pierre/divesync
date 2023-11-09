use crate::deco::DecoStop;
use crate::profile::DiveProfile;
use crate::zhl16::tissue::ZHL16Variant;
use std::fmt;

use crate::dsat::algorithm::DSATAlgorithm;
use crate::tissue::{CompartmentSnapshot, TissueCompartment};
use crate::zhl16::algorithm::ZHL16Algorithm;

pub trait DecoAlgorithm {
    fn variant(&self) -> DecoAlgorithmVariant;
    fn compute_deco_stops(&self, dive_profile: DiveProfile) -> Vec<DecoStop>;
    fn compute_ndl(&self, dive_profile: DiveProfile) -> u32;
    fn snapshot(&self) -> Vec<CompartmentSnapshot>;
    fn run(&mut self, ata: f32, time: f32);
}

pub struct AlgorithmRunner {
    algo: Box<dyn DecoAlgorithm>,
}

impl AlgorithmRunner {
    pub fn new(algo: Box<dyn DecoAlgorithm>) -> Self {
        Self { algo }
    }

    /// Run the algorithm given profile
    /// end result returns resultant TissueCompartments
    pub fn run(&mut self, dive_profile: DiveProfile) -> Vec<CompartmentSnapshot> {
        self.algo.snapshot()
    }
}

pub fn get_algo(algo: &str) -> Option<Box<dyn DecoAlgorithm>> {
    let algo: DecoAlgorithmVariant = algo.into();
    match algo {
        DecoAlgorithmVariant::Dsat => Some(Box::new(DSATAlgorithm::new())),
        DecoAlgorithmVariant::ZHL16(ZHL16Variant::A) => {
            Some(Box::new(ZHL16Algorithm::new(ZHL16Variant::A)))
        }
        DecoAlgorithmVariant::ZHL16(ZHL16Variant::B) => {
            Some(Box::new(ZHL16Algorithm::new(ZHL16Variant::B)))
        }
        DecoAlgorithmVariant::ZHL16(ZHL16Variant::C) => {
            Some(Box::new(ZHL16Algorithm::new(ZHL16Variant::C)))
        }
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum DecoAlgorithmVariant {
    ZHL16(ZHL16Variant),
    Dsat,
}

impl From<&str> for DecoAlgorithmVariant {
    fn from(s: &str) -> Self {
        match s {
            "dsat" => DecoAlgorithmVariant::Dsat,
            "zhl16" => DecoAlgorithmVariant::ZHL16(ZHL16Variant::A),
            "zhl16-a" => DecoAlgorithmVariant::ZHL16(ZHL16Variant::A),
            "zhl16-b" => DecoAlgorithmVariant::ZHL16(ZHL16Variant::B),
            "zhl16-c" => DecoAlgorithmVariant::ZHL16(ZHL16Variant::C),
            _ => panic!("Invalid DecoAlgorithmEnum variant: {}", s),
        }
    }
}

impl fmt::Display for DecoAlgorithmVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecoAlgorithmVariant::Dsat => write!(f, "DSAT"),
            DecoAlgorithmVariant::ZHL16(ZHL16Variant::A) => write!(f, "ZHL16-A"),
            DecoAlgorithmVariant::ZHL16(ZHL16Variant::B) => write!(f, "ZHL16-B"),
            DecoAlgorithmVariant::ZHL16(ZHL16Variant::C) => write!(f, "ZHL16-C"),
        }
    }
}
