use std::fmt;

use crate::gas::GasType;
#[allow(non_snake_case)]
use crate::gas::{Gas, GasMix, GasSymbol, PPN2};
use crate::tissue::TissueCompartment;
use crate::utils::n_root;
use crate::zhl16::utils::build_air_tissue;

#[derive(Debug, Clone)]
pub struct ZHL16Compartment {
    pp_n2: f32,
    pp_he: f32,
    pub cpt_num: usize,
    pub gas_mix: GasMix,
    pub variant: ZHL16Variant,
    pub elapsed_time: f32,
}

impl TissueCompartment for ZHL16Compartment {
    fn update_pressure(&mut self, ata: f32, time: f32) {
        // update N2 pressure
        let exp: f32 = 2_f32.powf(-(time / self.n2_ht()));
        let gas_pp = self.gas_mix.pp_n2(ata);
        let current_pp = self.pp_n2;
        let new_pp = current_pp + (gas_pp - current_pp) * (1.0 - exp);
        self.pp_n2 = new_pp;

        // update He pressure
        let exp: f32 = 2_f32.powf(-(time / self.he_ht()));
        let gas_pp = self.gas_mix.pp_he(ata);
        let current_pp = self.pp_he;
        let new_pp = current_pp + (gas_pp - current_pp) * (1.0 - exp);
        self.pp_he = new_pp;

        // update elapsed time
        self.elapsed_time += time;
    }

    fn half_time(&self) -> f32 {
        match self.gas_mix.mix_type() {
            GasType::Nitrox => self.n2_ht(),
            GasType::Heliox => self.he_ht(),
            GasType::Trimix => {
                let pp_he = self.gas_mix.pp_he(1.0);
                let pp_n2 = self.gas_mix.pp_n2(1.0);

                ((self.he_ht() * pp_he) + (self.n2_ht() * pp_n2)) / pp_he + pp_n2
            }
        }
    }

    fn m_value(&self) -> f32 {
        // TODO:
        // implement v_value
        match self.gas_mix.mix_type() {
            GasType::Nitrox => {
                let b = 0.0;
                return self.pp_n2;
            }
            GasType::Heliox => {
                let b = 0.0;

                return self.pp_he;
            }
            GasType::Trimix => {
                let b = 0.0;

                return self.pp_he + self.pp_n2;
            }
        }
    }

    fn n2_he_pp(&self) -> (f32, f32) {
        (self.pp_n2, self.pp_he)
    }

    fn gas_mix(&self) -> GasMix {
        self.gas_mix.clone()
    }
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
            elapsed_time: 0.0,
            cpt_num,
            pp_n2: gas_mix.pp_n2(1.0),
            pp_he: gas_mix.pp_he(1.0),
            gas_mix,
            variant,
        }
    }

    pub fn get_a(&self) -> f32 {
        match self.gas_mix.mix_type() {
            GasType::Nitrox => self.n2_a(),
            GasType::Heliox => self.he_a(),
            GasType::Trimix => {
                let pp_he = self.gas_mix.pp_he(1.0);
                let pp_n2 = self.gas_mix.pp_n2(1.0);

                ((self.he_a() * pp_he) + (self.n2_a() * pp_n2)) / pp_he + pp_n2
            }
        }
    }

    pub fn get_b(&self) -> f32 {
        match self.gas_mix.mix_type() {
            GasType::Nitrox => self.n2_b(),
            GasType::Heliox => self.he_b(),
            GasType::Trimix => {
                let pp_he = self.gas_mix.pp_he(1.0);
                let pp_n2 = self.gas_mix.pp_n2(1.0);

                ((self.he_b() * pp_he) + (self.n2_b() * pp_n2)) / pp_he + pp_n2
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

    pub fn set_gas_mix(&mut self, mix: GasMix) {
        self.gas_mix = mix;
    }

    // ---
    // PRIVATE METHODS
    // ---
    fn n2_a(&self) -> f32 {
        let ht = self.n2_ht();
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

    fn n2_b(&self) -> f32 {
        let ht = self.n2_ht();

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

    fn he_a(&self) -> f32 {
        let ht = self.he_ht();
        let denom = n_root(ht, 3); // denominator
        let a = 2.0 / denom;
        a
    }

    fn he_b(&self) -> f32 {
        let ht = self.he_ht();

        let demon: f32 = n_root(ht, 2);
        let b = 1.005 - (1.0 / demon);
        b
    }

    fn n2_ht(&self) -> f32 {
        ZHL16Compartment::N2_HALF_TIMES[self.cpt_num]
    }

    fn he_ht(&self) -> f32 {
        ZHL16Compartment::HE_HALF_TIMES[self.cpt_num]
    }
}

#[derive(Debug, Clone)]
pub enum ZHL16Variant {
    A,
    B,
    C,
}

impl fmt::Display for ZHL16Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZHL16Variant::A => write!(f, "ZHL16-A"),
            ZHL16Variant::B => write!(f, "ZHL16-B"),
            ZHL16Variant::C => write!(f, "ZHL16-C"),
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
        zhl16::utils::{build_air_tissue, build_trimix_tissue},
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

        let (t1_n2, _) = tissue1.n2_he_pp();
        let (t2_n2, _) = tissue2.n2_he_pp();

        assert_eq!(round_f32(t1_n2, 3), round_f32(t2_n2, 3))
    }

    #[test]
    fn test_tissue_nitrox_diffuse_rate() {
        let (mix, mut tissue1) = build_air_tissue(0);

        let mut tissue2 = tissue1.clone();

        // expose tissue2 to 10m for 30min
        tissue2.update_pressure(2.0, 30.0);

        // get pp difference between surface and 10m after 30min
        let (t2_n2, _) = tissue2.n2_he_pp();
        let diff = t2_n2 - mix.pp_n2(1.0);

        // saturate tissue at 10m
        tissue1.update_pressure(2.0, 10000000.0);
        let (t1_n2, _) = tissue1.n2_he_pp();
        let pp_at_depth = t1_n2;

        // take tissue1 back to surface after 30min, get pp, should be same as initial on gassing diff
        tissue1.update_pressure(1.0, 30.0);
        let (t1_n2, _) = tissue1.n2_he_pp();
        let surf_diff = pp_at_depth - t1_n2;

        assert_eq!(round_f32(diff, 6), round_f32(surf_diff, 6));
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
        let (t2_n2, _) = tissue2.n2_he_pp();
        let desc_diff = t2_n2 - mix.pp_n2(1.0);

        // saturate tissue at 10m
        tissue1.update_pressure(2.0, 100000.0);

        let (t1_n2, _) = tissue1.n2_he_pp();
        let pp_at_depth = t1_n2;

        // take tissue1 back to surface after 30min, get pp, should be same as initial on gassing diff
        for _ in 0..(30 * 60) {
            tissue1.update_pressure(1.0, 1.0 / 60.0)
        }

        let (t1_n2, _) = tissue1.n2_he_pp();
        let surf_diff = pp_at_depth - t1_n2;

        assert_eq!(round_f32(surf_diff, 3), round_f32(desc_diff, 3));
    }

    #[test]
    fn test_tissue_nitrox_half_time() {
        for i in 0..16 {
            let (_, tissue) = build_air_tissue(i);
            let expected_ht = ZHL16Compartment::N2_HALF_TIMES[i];

            assert_eq!(round_f32(tissue.half_time(), 3), round_f32(expected_ht, 3));
        }
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

    #[test]
    fn test_tissue_nitrox_a_variant_B() {
        let (_, mut tissue1) = build_air_tissue(5);
        tissue1.set_variant(ZHL16Variant::B);
        assert_eq!(round_f32(tissue1.get_a(), 5), 0.5600);

        let (_, mut tissue2) = build_air_tissue(12);
        tissue2.set_variant(ZHL16Variant::B);
        assert_eq!(round_f32(tissue2.get_a(), 5), 0.2850);
    }

    #[test]
    fn test_tissue_nitrox_a_variant_C() {
        let (_, mut tissue) = build_air_tissue(6);
        tissue.set_variant(ZHL16Variant::C);
        assert_eq!(round_f32(tissue.get_a(), 5), 0.4410);

        let (_, mut tissue) = build_air_tissue(10);
        tissue.set_variant(ZHL16Variant::C);
        assert_eq!(round_f32(tissue.get_a(), 5), 0.3295);

        let (_, mut tissue) = build_air_tissue(14);
        tissue.set_variant(ZHL16Variant::C);
        assert_eq!(round_f32(tissue.get_a(), 5), 0.2480);
    }

    #[test]
    fn test_tissue_nitrox_b() {
        let (_, t) = build_air_tissue(3);
        assert_eq!(round_f32(t.get_b(), 4), 0.7825);
        let (_, t) = build_air_tissue(4);
        assert_eq!(round_f32(t.get_b(), 4), 0.8126);
        let (_, t) = build_air_tissue(9);
        assert_eq!(round_f32(t.get_b(), 4), 0.9222);
        let (_, t) = build_air_tissue(10);
        assert_eq!(round_f32(t.get_b(), 4), 0.9319);
    }

    // TODO:
    // Test nitrox M value

    // ---
    // TISSUE HELIOX TESTS
    // ---

    #[test]
    fn test_tissue_heliox_diffuse_rate() {
        let (mix, mut t1) = build_trimix_tissue(4, 0.78, 0.21);

        let mut t2 = t1.clone();

        // expose t2 to 10m for 30min
        t2.update_pressure(2.0, 30.0);

        // get pp difference between surface and 10m after 30min
        let (_, t2_he) = t2.n2_he_pp();
        let diff = t2_he - mix.pp_he(1.0);

        // saturate tissue at 10m
        t1.update_pressure(2.0, 100000000.0);

        let (_, t1_he) = t1.n2_he_pp();
        let pp_at_depth = t1_he;

        // take t1 back to surface after 30min, get pp, should be same as initial on gassing diff
        t1.update_pressure(1.0, 30.0);

        let (_, t1_he) = t1.n2_he_pp();
        let surf_diff = pp_at_depth - t1_he;

        assert_eq!(round_f32(diff, 6), round_f32(surf_diff, 6));
    }

    #[test]
    fn test_tissue_heliox_half_time() {
        for i in 0..16 {
            let (_, tissue) = build_trimix_tissue(i, 0.78, 0.21);
            let expected_ht = ZHL16Compartment::HE_HALF_TIMES[i];

            assert_eq!(round_f32(tissue.half_time(), 3), round_f32(expected_ht, 3));
        }
    }

    #[test]
    fn test_tissue_heliox_a() {
        for i in 0..16 {
            let (_, tissue) = build_trimix_tissue(i, 0.78, 0.21);

            let he_ht = ZHL16Compartment::HE_HALF_TIMES[i];
            let denom = n_root(he_ht, 3); // denominator
            let a = 2.0 / denom;

            assert_eq!(round_f32(tissue.get_a(), 3), round_f32(a, 3));
        }
    }

    #[test]
    fn test_tissue_heliox_b() {
        for i in 0..16 {
            let (_, tissue) = build_trimix_tissue(i, 0.78, 0.21);

            let he_ht = ZHL16Compartment::HE_HALF_TIMES[i];
            let demon: f32 = n_root(he_ht, 2);
            let b = 1.005 - (1.0 / demon);

            assert_eq!(round_f32(tissue.get_b(), 3), round_f32(b, 3));
        }
    }

    // TODO:
    // Test Heliox M Value

    // TODO:
    // ---
    // TEST TISSUE TRIMIX
    // ---
}
