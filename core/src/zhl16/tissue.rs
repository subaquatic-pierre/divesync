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
    pub cpt_num: usize, // Half-time of the tissue compartment in minutes
    pp: f32,            // Current partial pressure of gas in the tissue compartment
    pub gas_mix: GasMix,
    pub variant: ZHL16Variant,
}

impl ZHL16Compartment {
    pub const N2_HALF_TIMES: [f32; 16] = [
        4.0, 8.0, 12.5, 18.5, 27.0, 38.3, 54.3, 77.0, 109.0, 146.0, 187.0, 239.0, 305.0, 390.0,
        498.0, 635.0,
    ];

    pub fn new(cpt_num: usize, gas_mix: GasMix) -> Self {
        // TODO:
        // ensure correct handle of unwrap if gas symbol not found
        let pp = match gas_mix.mix_type() {
            GasType::Heliox => gas_mix.pp_he(1.0),
            GasType::Nitrox => gas_mix.pp_n2(1.0),
            GasType::Trimix => {
                // TODO:
                // get pp of trimix
                gas_mix.pp_n2(1.0)
            }
        };

        Self {
            cpt_num,
            pp: 1.0,
            gas_mix,
            // TODO: take variant in constructor
            variant: ZHL16Variant::A,
        }
    }

    pub fn a(&self) -> f32 {
        let den = n_root(self.half_time(), 3); // denominator
        let a = 2.0 / den;

        let ht = self.half_time();

        // return a early if self.gas_symbol is not Nitrogen
        // only nitrogen tissue compartment type uses variants

        // only use variant if compartment gas type is N2
        match self.variant {
            ZHL16Variant::A => {
                return a;
            }
            ZHL16Variant::B => {
                if ht == 18.5 {
                    return 0.7825;
                }
                if ht == 27.0 {
                    return 0.8126;
                }

                unimplemented!();
            }
            ZHL16Variant::C => {
                unimplemented!();
            }
        }

        // calculate actual a value
        todo!()
    }

    pub fn b(&self) -> f32 {
        // NOTE:
        // special case for compartments 4 and 5
        let ht = self.half_time();

        if ht == 18.5 {
            return 0.7825;
        }
        if ht == 27.0 {
            return 0.8126;
        }

        // calculate actual a value
        unimplemented!()
    }

    pub fn set_variant(&mut self, variant: ZHL16Variant) {
        self.variant = variant;
    }
}

impl TissueCompartment for ZHL16Compartment {
    fn update_pressure(&mut self, ata: f32, time: f32) {
        // TODO:
        // ensure correct handle of unwrap if gas symbol not found

        let exp: f32 = 2_f32.powf(-(time / self.half_time()));

        let gas_pp = match self.gas_mix.mix_type() {
            GasType::Heliox => self.gas_mix.pp_he(ata),
            GasType::Nitrox => self.gas_mix.pp_n2(ata),
            GasType::Trimix => {
                // TODO:
                // get pp of trimix
                self.gas_mix.pp_n2(ata)
            }
        };

        let current_pp = self.pp;

        let new_pp = current_pp + (gas_pp - current_pp) * (1.0 - exp);

        self.pp = new_pp
    }

    fn half_time(&self) -> f32 {
        // TODO:
        // handle case of helium or trimix

        // TODO:
        // match on gas_mix.mix_type

        ZHL16Compartment::N2_HALF_TIMES[self.cpt_num]

        // self.half_time
    }

    fn get_pp(&self) -> f32 {
        self.pp
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

        assert_eq!(round_f32(tissue1.pp, 3), round_f32(tissue2.pp, 3))
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
        let desc_diff = tissue2.pp - mix.pp_n2(1.0);

        // saturate tissue at 10m
        tissue1.update_pressure(2.0, 100000.0);

        let pp_at_depth = tissue1.pp;

        // take tissue1 back to surface after 30min, get pp, should be same as initial on gassing diff

        for _ in 0..(30 * 60) {
            tissue1.update_pressure(1.0, 1.0 / 60.0)
        }

        let surf_diff = pp_at_depth - tissue1.pp;

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

            assert_eq!(round_f32(expected_a, 3), round_f32(tissue.a(), 3));
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
