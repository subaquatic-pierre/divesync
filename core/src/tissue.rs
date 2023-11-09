use std::fmt;

use crate::{
    algorithm::DecoAlgorithmVariant,
    gas::{Gas, GasMix, GasSymbol, GasType},
};

pub trait TissueCompartment {
    /// Get the half time used by the compartment
    fn half_time(&self) -> f32;

    /// Main update method
    fn update_pressure(&mut self, ata: f32, time: f32);

    /// Get the gas mix currently in use
    fn gas_mix(&self) -> GasMix;

    /// Get M-value at any given point in time
    fn m_value(&self) -> f32;

    /// Get the partial pressure of tissue at current point in time
    fn n2_he_pp(&self) -> (f32, f32);
}

impl fmt::Display for dyn TissueCompartment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (pp_n2, pp_he) = self.n2_he_pp();
        write!(
            f,
            "Tissue Compartment:\n  
                Half-Time: {} minutes\n
                Current N2 Pressure: {} bar\n
                Current He Pressure: {} bar\n
                GasMix : {}",
            self.half_time(),
            pp_n2,
            pp_he,
            self.gas_mix()
        )
    }
}

pub struct CompartmentSnapshot {
    pub variant: DecoAlgorithmVariant,
    pub elapsed_time: f32,
    pub pp_n2: f32,
    pub pp_he: f32,
    pub gas_mix: GasMix,
    pub gas_type: GasType,
    pub half_time: f32,
    pub cpt_num: usize,
    pub last_depth: f32,
}

impl fmt::Display for CompartmentSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (pp_n2, pp_he) = (self.pp_n2, self.pp_he); // Assuming pp_n2 and pp_he are fields of CompartmentSnapshot struct
        write!(
            f,
            "Tissue Compartment:\n
            Half-Time: {} minutes\n
            Current N2 Pressure: {} bar\n
            Current He Pressure: {} bar\n
            Gas Mix: {}\n
            Gas Type: {:?}\n
            Variant: {:?}\n
            CPT Number: {}\n,
            Last Depth: {}",
            self.half_time,
            pp_n2,
            pp_he,
            self.gas_mix, // Assuming GasMix implements Display trait
            self.gas_type,
            self.variant,
            self.cpt_num,
            self.last_depth
        )
    }
}
