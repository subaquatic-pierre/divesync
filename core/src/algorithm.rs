use csv::Writer;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::deco::DecoStop;
use crate::dsat::algorithm::DSATAlgorithm;
use crate::gas::GasMix;
use crate::profile::DiveProfile;
use crate::tissue::CompartmentSnapshot;
use crate::utils::calc_ata;
use crate::utils::timestamp;
use crate::zhl16::algorithm::ZHL16Algorithm;
use crate::zhl16::tissue::ZHL16Variant;

pub trait DecoAlgorithm {
    fn variant(&self) -> DecoAlgorithmVariant;
    fn compute_deco_stops(&self, dive_profile: DiveProfile) -> Vec<DecoStop>;
    fn compute_ndl(&self, dive_profile: DiveProfile) -> u32;
    fn snapshot(&self) -> Vec<CompartmentSnapshot>;
    fn run(&mut self, mix: GasMix, ata: f32, time: f32);
}

pub fn get_algo(algo: &str) -> Result<Box<dyn DecoAlgorithm>, Box<dyn std::error::Error>> {
    let algo: DecoAlgorithmVariant = algo.into();
    match algo {
        DecoAlgorithmVariant::Dsat => Ok(Box::new(DSATAlgorithm::new())),
        DecoAlgorithmVariant::ZHL16(ZHL16Variant::A) => {
            Ok(Box::new(ZHL16Algorithm::new(ZHL16Variant::A)))
        }
        DecoAlgorithmVariant::ZHL16(ZHL16Variant::B) => {
            Ok(Box::new(ZHL16Algorithm::new(ZHL16Variant::B)))
        }
        DecoAlgorithmVariant::ZHL16(ZHL16Variant::C) => {
            Ok(Box::new(ZHL16Algorithm::new(ZHL16Variant::C)))
        }
        _ => panic!("unable to find algorithm"),
    }
}

#[derive(Debug, Clone)]
pub enum DecoAlgorithmVariant {
    ZHL16(ZHL16Variant),
    Dsat,
}

impl From<&str> for DecoAlgorithmVariant {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dsat" => DecoAlgorithmVariant::Dsat,
            "zhl16" => DecoAlgorithmVariant::ZHL16(ZHL16Variant::A),
            "zhl16-a" => DecoAlgorithmVariant::ZHL16(ZHL16Variant::A),
            "zhl16-b" => DecoAlgorithmVariant::ZHL16(ZHL16Variant::B),
            "zhl16-c" => DecoAlgorithmVariant::ZHL16(ZHL16Variant::C),
            _ => panic!("Invalid DecoAlgorithm: {}", s),
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

mod test {
    // TODO: algorithm tests
}
