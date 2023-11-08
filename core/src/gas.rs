use std::{collections::HashMap, fmt::Display};

pub const PPN2: f32 = 0.78;
pub const PP02: f32 = 0.21;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum GasSymbol {
    Oxygen,
    Helium,
    Nitrogen,
}

impl Display for GasSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GasSymbol::Oxygen => write!(f, "O2"),
            GasSymbol::Helium => write!(f, "He"),
            GasSymbol::Nitrogen => write!(f, "N2"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gas {
    pub base_pp: f32,
    pub symbol: GasSymbol,
}

impl Gas {
    pub fn new(base_pp: f32, symbol: GasSymbol) -> Self {
        Self { base_pp, symbol }
    }

    pub fn get_pp(&self, ata: f32) -> f32 {
        ata * self.base_pp
    }

    pub fn is_some(&self) -> bool {
        return self.base_pp > 0.0;
    }

    pub fn is_none(&self) -> bool {
        return self.base_pp == 0.0;
    }
}

// pub type GasMix = HashMap<GasSymbol, Gas>;

#[derive(Debug, Clone)]
pub struct GasMix {
    oxygen: Gas,
    nitrogen: Gas,
    helium: Gas,
}

impl GasMix {
    pub fn new(oxygen: Gas, nitrogen: Gas, helium: Gas) -> Self {
        Self {
            oxygen,
            nitrogen,
            helium,
        }
    }

    pub fn new_nitrox(oxygen: f32) -> Self {
        // TODO:
        // error handling on wrong base values
        let nitrogen = Gas::new(0.78 - oxygen, GasSymbol::Nitrogen);
        let oxygen = Gas::new(oxygen, GasSymbol::Oxygen);
        let helium = Gas::new(0.0, GasSymbol::Helium);

        Self {
            oxygen,
            nitrogen,
            helium,
        }
    }

    pub fn new_trimix(helium: f32, oxygen: f32) -> Self {
        // TODO:
        // error handling on wrong base values
        let nitrogen = Gas::new(0.78 - (oxygen + helium), GasSymbol::Nitrogen);
        let oxygen = Gas::new(oxygen, GasSymbol::Oxygen);
        let helium = Gas::new(helium, GasSymbol::Helium);

        Self {
            oxygen,
            nitrogen,
            helium,
        }
    }

    pub fn pp_o2(&self, ata: f32) -> f32 {
        self.oxygen.get_pp(ata)
    }

    pub fn pp_n2(&self, ata: f32) -> f32 {
        self.nitrogen.get_pp(ata)
    }

    pub fn pp_he(&self, ata: f32) -> f32 {
        self.helium.get_pp(ata)
    }

    pub fn mix_type(&self) -> GasType {
        if self.helium.is_some() && self.nitrogen.is_none() {
            return GasType::Heliox;
        } else if self.helium.is_some() && self.nitrogen.is_some() {
            return GasType::Trimix;
        } else {
            return GasType::Nitrox;
        }
    }
}

pub enum GasType {
    Nitrox,
    Heliox,
    Trimix,
}

mod test {
    use super::*;
    #[test]
    fn test_gas_pp() {
        let gas = Gas::new(0.78, GasSymbol::Nitrogen);

        let pp = gas.get_pp(2.0);

        assert_eq!(1.56, pp);
    }
}
