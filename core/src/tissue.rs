use std::fmt;

use crate::gas::{Gas, GasMix, GasSymbol};

pub trait TissueCompartment {
    fn update_pressure(&mut self, ata: f32, time: f32);
    fn half_time(&self) -> f32;
    fn get_pp(&self) -> f32;
}

impl fmt::Display for dyn TissueCompartment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tissue Compartment:\n  Half-Time: {} minutes\n  Current Pressure: {} bar\n",
            self.half_time(),
            self.get_pp()
        )
    }
}
