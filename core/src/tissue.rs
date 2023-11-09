use std::fmt;

use crate::gas::{Gas, GasMix, GasSymbol};

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
