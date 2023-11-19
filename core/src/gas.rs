use std::{
    collections::HashMap,
    fmt::{self, Display},
};

pub const PPN2: f32 = 0.78;
pub const PPO2: f32 = 0.21;

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

impl Display for Gas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gas: {} (Base Partial Pressure: {})",
            self.symbol, self.base_pp
        )
    }
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
        let pp_n2 = if oxygen <= PPO2 {
            PPN2
        } else {
            PPN2 - (oxygen - PPO2)
        };
        let nitrogen = Gas::new(pp_n2, GasSymbol::Nitrogen);
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
        let pp_n2 = if oxygen <= PPO2 {
            PPN2 - helium
        } else {
            PPN2 - (helium + (oxygen - PPO2))
        };
        let nitrogen = Gas::new(pp_n2, GasSymbol::Nitrogen);
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

#[derive(Clone, PartialEq, Debug)]
pub enum GasType {
    Nitrox,
    Heliox,
    Trimix,
}

impl Display for GasType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GasType::Nitrox => write!(f, "Nitrox"),
            GasType::Heliox => write!(f, "Heliox"),
            GasType::Trimix => write!(f, "Trimix"),
        }
    }
}

impl Display for GasMix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the Display trait for GasMix
        write!(
            f,
            "Gas Mix:\n
               (Oxygen: {}, Nitrogen: {}, Helium: {})",
            self.oxygen, self.nitrogen, self.helium
        )
    }
}

mod test {
    use crate::utils::round_f32;

    use super::*;
    #[test]
    fn test_gas_pp() {
        let gas = Gas::new(PPN2, GasSymbol::Nitrogen);

        let pp = gas.get_pp(2.0);

        assert_eq!(PPN2 * 2.0, pp);
    }
    #[test]
    fn test_gas_is_none() {
        let gas = Gas::new(0.0, GasSymbol::Helium);

        assert!(gas.is_none());
    }
    #[test]
    fn test_gas_is_some() {
        let gas = Gas::new(PPN2, GasSymbol::Nitrogen);

        assert!(gas.is_some());
    }

    #[test]
    fn test_mix_new_nitrox() {
        let mix = GasMix::new_nitrox(0.21);

        let pp_o2 = mix.pp_o2(1.0);
        let pp_n2 = mix.pp_n2(1.0);

        assert_eq!(PPO2, pp_o2);
        assert_eq!(PPN2, pp_n2);
    }

    #[test]
    fn test_mix_new_trimix() {
        let mix = GasMix::new_trimix(0.30, 0.16);

        let pp_o2 = mix.pp_o2(1.0);
        let pp_n2 = mix.pp_n2(1.0);
        let pp_he = mix.pp_he(1.0);

        assert_eq!(0.16, pp_o2);
        assert_eq!(0.48, round_f32(pp_n2, 3));
        assert_eq!(0.30, pp_he);
    }

    #[test]
    fn test_mix_new_trimix_2() {
        let mix = GasMix::new_trimix(0.50, 0.12);

        let pp_o2 = mix.pp_o2(1.0);
        let pp_n2 = mix.pp_n2(1.0);
        let pp_he = mix.pp_he(1.0);

        assert_eq!(0.12, pp_o2);
        assert_eq!(0.28, round_f32(pp_n2, 3));
        assert_eq!(0.50, pp_he);
    }

    #[test]
    fn test_mix_new_trimix_3() {
        let mix = GasMix::new_trimix(0.10, 0.30);

        let pp_o2 = mix.pp_o2(1.0);
        let pp_n2 = mix.pp_n2(1.0);
        let pp_he = mix.pp_he(1.0);

        assert_eq!(0.30, pp_o2);
        assert_eq!(0.59, round_f32(pp_n2, 3));
        assert_eq!(0.10, pp_he);
    }

    #[test]
    fn test_mix_type() {
        let mix = GasMix::new_nitrox(0.21);
        assert_eq!(mix.mix_type(), GasType::Nitrox);

        let mix = GasMix::new_trimix(0.30, 0.21);
        assert_eq!(mix.mix_type(), GasType::Trimix);

        let mix = GasMix::new_trimix(0.78, 0.21);
        assert_eq!(mix.mix_type(), GasType::Heliox);
    }
}
