use crate::{
    algorithm::{DecoAlgorithm, DecoAlgorithmVariant},
    deco::DecoStop,
    gas::{self, GasMix},
    profile::DiveProfile,
    tissue::{CompartmentSnapshot, TissueCompartment},
    utils::calc_ata,
    zhl16::tissue::ZHL16Compartment,
};

use super::tissue::ZHL16Variant;

pub struct ZHL16Algorithm {
    tissues: Vec<ZHL16Compartment>,
    variant: ZHL16Variant,
}

impl DecoAlgorithm for ZHL16Algorithm {
    fn variant(&self) -> DecoAlgorithmVariant {
        DecoAlgorithmVariant::ZHL16(self.variant.clone())
    }

    fn compute_deco_stops(&self, dive_profile: DiveProfile) -> Vec<DecoStop> {
        vec![]
    }

    fn compute_ndl(&self, dive_profile: DiveProfile) -> u32 {
        42
    }

    fn snapshot(&self) -> Vec<CompartmentSnapshot> {
        let mut snaps = vec![];
        for t in &self.tissues {
            let (pp_n2, pp_he) = t.n2_he_pp();
            snaps.push(CompartmentSnapshot {
                elapsed_time: t.elapsed_time,
                pp_n2,
                pp_he,
                gas_mix: t.gas_mix(),
                gas_type: t.gas_mix.mix_type(),
                half_time: t.half_time(),
                cpt_num: t.cpt_num,
                variant: self.variant(),
                last_depth: t.last_depth,
            })
        }

        snaps
    }

    fn run(&mut self, mix: GasMix, ata: f32, time: f32) {
        if self.tissues.len() == 0 {
            self.init(mix.clone());
        }

        for t in &mut self.tissues {
            t.set_gas_mix(mix.clone());
            t.update_pressure(ata, time);
        }
    }
}

impl ZHL16Algorithm {
    pub fn new(variant: ZHL16Variant) -> Self {
        Self {
            tissues: vec![],
            variant,
        }
    }

    pub fn init(&mut self, mix: GasMix) {
        if self.tissues.len() > 0 {
            panic!(
                "Cannot re-initialize the ZHL16 algorithm after it has already bean initialized."
            )
        }

        let mut tissues: Vec<ZHL16Compartment> = vec![];

        for i in 0..16 {
            let t = ZHL16Compartment::new(i, mix.clone(), Some(self.variant.clone()));
            tissues.push(t)
        }

        self.tissues = tissues;
    }
}

mod test {
    // TODO: ZHL16 algorithm tests
}
