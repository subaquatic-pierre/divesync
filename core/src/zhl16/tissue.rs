use crate::gas::GasType;
#[allow(non_snake_case)]
use crate::gas::{Gas, GasMix, GasSymbol, PPN2};
use crate::tissue::TissueCompartment;
use crate::utils::n_root;
use crate::zhl16::utils::build_air_tissue;

#[derive(Debug, Clone)]
pub enum ZHL16Variant {
    A,
    B,
    C,
}

#[derive(Debug, Clone)]
pub struct ZHL16Compartment {
    pp_n2: f32,
    pp_he: f32,
    pub cpt_num: usize,
    pub gas_mix: GasMix,
    pub variant: ZHL16Variant,
}

impl ZHL16Compartment {
    pub const N2_HALF_TIMES: [f32; 16] = [
        4.0, 8.0, 12.5, 18.5, 27.0, 38.3, 54.3, 77.0, 109.0, 146.0, 187.0, 239.0, 305.0, 390.0,
        498.0, 635.0,
    ];

    pub const HE_HALF_TIMES: [f32; 16] = [
        1.5119, 3.0237, 4.7245, 6.9923, 10.205, 14.476, 20.5234, 29.1032, 41.198, 55.1826, 70.6791,
        90.3332, 115.2788, 147.4056, 188.2256, 240.0066,
    ];

    pub fn new(cpt_num: usize, gas_mix: GasMix, variant: Option<ZHL16Variant>) -> Self {
        let variant = if variant.is_some() {
            variant.unwrap()
        } else {
            ZHL16Variant::A
        };

        Self {
            cpt_num,
            pp_n2: gas_mix.pp_n2(1.0),
            pp_he: gas_mix.pp_he(1.0),
            gas_mix,
            variant,
        }
    }

    pub fn get_a(&self) -> f32 {
        match self.gas_mix.mix_type() {
            GasType::Nitrox => self.get_n2_a(),
            GasType::Heliox => self.get_he_a(),
            GasType::Trimix => {
                let pp_he = self.gas_mix.pp_he(1.0);
                let pp_n2 = self.gas_mix.pp_n2(1.0);

                ((self.get_he_a() * pp_he) + (self.get_n2_a() * pp_n2)) / pp_he + pp_n2
            }
        }
    }

    pub fn get_b(&self) -> f32 {
        match self.gas_mix.mix_type() {
            GasType::Nitrox => self.get_n2_b(),
            GasType::Heliox => self.get_he_b(),
            GasType::Trimix => {
                let pp_he = self.gas_mix.pp_he(1.0);
                let pp_n2 = self.gas_mix.pp_n2(1.0);

                ((self.get_he_b() * pp_he) + (self.get_n2_b() * pp_n2)) / pp_he + pp_n2
            }
        }
    }

    pub fn set_variant(&mut self, variant: ZHL16Variant) {
        self.variant = variant;
    }

    pub fn set_pp(&mut self, nitrogen: f32, helium: f32) {
        self.pp_n2 = nitrogen;
        self.pp_he = helium;
    }

    // ---
    // PRIVATE METHODS
    // ---
    fn get_n2_a(&self) -> f32 {
        let ht = self.get_n2_ht();
        let denom = n_root(ht, 3); // denominator
        let a = 2.0 / denom;

        // return a early if self.gas_symbol is not Nitrogen
        // only nitrogen tissue compartment type uses variants

        // only use variant if compartment gas type is N2
        let cpt_num = self.cpt_num;
        match self.variant {
            ZHL16Variant::A => {
                return a;
            }
            ZHL16Variant::B => {
                if cpt_num == 5 {
                    return 0.5600;
                }
                if cpt_num == 6 {
                    return 0.4947;
                }
                if cpt_num == 7 {
                    return 0.4500;
                }
                if cpt_num == 12 {
                    return 0.2850;
                }
                return a;
            }
            ZHL16Variant::C => {
                if cpt_num == 4 {
                    return 0.6200;
                }
                if cpt_num == 5 {
                    return 0.5043;
                }
                if cpt_num == 6 {
                    return 0.4410;
                }
                if cpt_num == 7 {
                    return 0.4000;
                }
                if cpt_num == 8 {
                    return 0.3750;
                }
                if cpt_num == 9 {
                    return 0.3500;
                }
                if cpt_num == 10 {
                    return 0.3295;
                }
                if cpt_num == 11 {
                    return 0.3065;
                }
                if cpt_num == 12 {
                    return 0.2835;
                }
                if cpt_num == 13 {
                    return 0.2610;
                }
                if cpt_num == 14 {
                    return 0.2480;
                }
                return a;
            }
        }
    }

    fn get_n2_b(&self) -> f32 {
        let ht = self.get_n2_ht();

        let demon: f32 = n_root(ht, 2);
        let b = 1.005 - (1.0 / demon);

        if self.cpt_num == 3 {
            return 0.7825;
        }
        if self.cpt_num == 4 {
            return 0.8126;
        }

        b
    }

    fn get_he_a(&self) -> f32 {
        let ht = self.get_he_ht();
        let denom = n_root(ht, 3); // denominator
        let a = 2.0 / denom;
        a
    }

    fn get_he_b(&self) -> f32 {
        let ht = self.get_he_ht();

        let demon: f32 = n_root(ht, 2);
        let b = 1.005 - (1.0 / demon);
        b
    }

    fn get_n2_ht(&self) -> f32 {
        ZHL16Compartment::N2_HALF_TIMES[self.cpt_num]
    }

    fn get_he_ht(&self) -> f32 {
        ZHL16Compartment::HE_HALF_TIMES[self.cpt_num]
    }
}

impl TissueCompartment for ZHL16Compartment {
    fn update_pressure(&mut self, ata: f32, time: f32) {
        // update N2 pressure
        let exp: f32 = 2_f32.powf(-(time / self.get_n2_ht()));
        let gas_pp = self.gas_mix.pp_n2(ata);
        let current_pp = self.pp_n2;
        let new_pp = current_pp + (gas_pp - current_pp) * (1.0 - exp);
        self.pp_n2 = new_pp;

        // update He pressure
        let exp: f32 = 2_f32.powf(-(time / self.get_he_ht()));
        let gas_pp = self.gas_mix.pp_he(ata);
        let current_pp = self.pp_he;
        let new_pp = current_pp + (gas_pp - current_pp) * (1.0 - exp);
        self.pp_he = new_pp;
    }

    fn half_time(&self) -> f32 {
        match self.gas_mix.mix_type() {
            GasType::Nitrox => self.get_n2_ht(),
            GasType::Heliox => self.get_he_ht(),
            GasType::Trimix => {
                let pp_he = self.gas_mix.pp_he(1.0);
                let pp_n2 = self.gas_mix.pp_n2(1.0);

                ((self.get_he_ht() * pp_he) + (self.get_n2_ht() * pp_n2)) / pp_he + pp_n2
            }
        }
    }

    fn get_pp(&self) -> f32 {
        match self.gas_mix.mix_type() {
            GasType::Nitrox => self.pp_n2,
            GasType::Heliox => self.pp_he,
            GasType::Trimix => self.pp_he + self.pp_n2,
        }
    }
}

// #[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use std::clone;

    use super::*;
    use crate::{
        gas::{Gas, GasMix, PPN2},
        utils::round_f32,
        zhl16::utils::build_air_tissue,
    };

    // ---
    // TISSUE NITROGEN TESTS
    // ---

    #[test]
    fn test_tissue_nitrox_over_steps() {
        let (_, mut tissue1) = build_air_tissue(0);

        let mut tissue2 = tissue1.clone();

        tissue1.update_pressure(2.2, 30.0);

        for _ in 0..(30 * 60) {
            tissue2.update_pressure(2.2, 1.0 / 60.0)
        }

        assert_eq!(
            round_f32(tissue1.get_pp(), 3),
            round_f32(tissue2.get_pp(), 3)
        )
    }

    #[test]
    fn test_tissue_nitrox_diffuse_rate() {
        let (mix, mut tissue1) = build_air_tissue(0);

        let mut tissue2 = tissue1.clone();

        // expose tissue2 to 10m for 30min
        tissue2.update_pressure(2.0, 30.0);

        // get pp difference between surface and 10m after 30min
        let diff = tissue2.get_pp() - mix.pp_n2(1.0);

        // saturate tissue at 10m
        tissue1.update_pressure(2.0, 10000000.0);

        let pp_at_depth = tissue1.get_pp();

        // take tissue1 back to surface after 30min, get pp, should be same as initial on gassing diff
        tissue1.update_pressure(1.0, 30.0);

        let surf_diff = pp_at_depth - tissue1.get_pp();

        assert_eq!(diff, surf_diff);
    }

    #[test]
    fn test_tissue_nitrox_diffuse_rate_over_steps() {
        let (mix, mut tissue1) = build_air_tissue(0);

        let mut tissue2 = tissue1.clone();

        // expose tissue2 to 10m for 30min
        for _ in 0..(30 * 60) {
            tissue2.update_pressure(2.0, 1.0 / 60.0)
        }

        // get pp difference between surface and 10m after 30min
        let desc_diff = tissue2.get_pp() - mix.pp_n2(1.0);

        // saturate tissue at 10m
        tissue1.update_pressure(2.0, 100000.0);

        let pp_at_depth = tissue1.get_pp();

        // take tissue1 back to surface after 30min, get pp, should be same as initial on gassing diff

        for _ in 0..(30 * 60) {
            tissue1.update_pressure(1.0, 1.0 / 60.0)
        }

        let surf_diff = pp_at_depth - tissue1.get_pp();

        assert_eq!(round_f32(surf_diff, 3), round_f32(desc_diff, 3));
    }

    #[test]
    fn test_tissue_nitrox_a_variant_A() {
        let excepted: [f32; 16] = [
            1.2599, 1.0000, 0.8618, 0.7562, 0.6667, 0.5933, 0.5282, 0.4701, 0.4187, 0.3798, 0.3497,
            0.3223, 0.2971, 0.2737, 0.2523, 0.2327,
        ];

        for i in 0..16 {
            let (_, tissue) = build_air_tissue(i);
            let expected_a = excepted[i];

            assert_eq!(round_f32(expected_a, 3), round_f32(tissue.get_a(), 3));
        }
    }

    // #[test]
    // fn test_tissue_nitrox_a_variant_B() {
    //     let excepted: [f32; 16] = [
    //         1.2599, 1.0000, 0.8618, 0.7562, 0.6667, 0.5600, 0.4947, 0.4500, 0.4187, 0.3798, 0.3497,
    //         0.3223, 0.2850, 0.2737, 0.2523, 0.2327,
    //     ];

    //     for i in 0..16 {
    //         let (_, tissue) = build_air_tissue(i);
    //         let expected_a = excepted[i];

    //         assert_eq!(round_f32(expected_a, 3), round_f32(tissue.a(), 3));
    //     }
    // }

    // #[test]
    // fn test_tissue_nitrox_a_variant_C() {
    //     let excepted: [f32; 16] = [
    //         1.2599, 1.0000, 0.8618, 0.7562, 0.6200, 0.5043, 0.4410, 0.4000, 0.3750, 0.3500, 0.3295,
    //         0.3065, 0.2835, 0.2610, 0.2480, 0.2327,
    //     ];

    //     for i in 0..16 {
    //         let (_, tissue) = build_air_tissue(i);
    //         let expected_a = excepted[i];

    //         assert_eq!(round_f32(expected_a, 3), round_f32(tissue.a(), 3));
    //     }
    // }

    // ---
    // TISSUE HELIUM TESTS
    // ---

    // #[test]
    // fn test_tissue_HE_half_time() {
    //     let excepted: [f32; 16] = [
    //         1.2599, 1.0000, 0.8618, 0.7562, 0.6667, 0.5933, 0.5282, 0.4701, 0.4187, 0.3798, 0.3497,
    //         0.3223, 0.2971, 0.2737, 0.2523, 0.2327,
    //     ];

    //     for i in 0..16 {
    //         let ht = ZHL16_N2_HALF_TIMES[i];

    //         let (_, tissue) = build_n2_tissue(ht);
    //         let expected_a = excepted[i];

    //         assert_eq!(round_f32(expected_a, 3), round_f32(tissue.a(), 3));
    //     }
    // }

    // #[test]
    // fn test_tissue_HE_a() {
    //     let excepted: [f32; 16] = [
    //         1.2599, 1.0000, 0.8618, 0.7562, 0.6667, 0.5933, 0.5282, 0.4701, 0.4187, 0.3798, 0.3497,
    //         0.3223, 0.2971, 0.2737, 0.2523, 0.2327,
    //     ];

    //     for i in 0..16 {
    //         let ht = ZHL16_N2_HALF_TIMES[i];

    //         let (_, tissue) = build_n2_tissue(ht);
    //         let expected_a = excepted[i];

    //         assert_eq!(round_f32(expected_a, 3), round_f32(tissue.a(), 3));
    //     }
    // }

    // #[test]
    // fn test_tissue_HE_b() {
    //     let excepted: [f32; 16] = [
    //         1.2599, 1.0000, 0.8618, 0.7562, 0.6667, 0.5933, 0.5282, 0.4701, 0.4187, 0.3798, 0.3497,
    //         0.3223, 0.2971, 0.2737, 0.2523, 0.2327,
    //     ];

    //     for i in 0..16 {
    //         let ht = ZHL16_N2_HALF_TIMES[i];

    //         let (_, tissue) = build_n2_tissue(ht);
    //         let expected_a = excepted[i];

    //         assert_eq!(round_f32(expected_a, 3), round_f32(tissue.a(), 3));
    //     }
    // }
}
