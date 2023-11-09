use crate::gas::{Gas, GasMix, GasSymbol, PPN2, PPO2};
use crate::utils::{n_root, round_f32};
use crate::zhl16::tissue::ZHL16Compartment;

// NOTE:
/// according to Graham's Law, the speed of diffusion (or effusion) of two gases under the same conditions of temperature and pressure is inversely proportional to the square root of their molar mass (28.0184 g/mol for N2 and 4.0026 g/mol for He), which means that He molecules diffuse 2.645 times faster than N2 molecules.
pub fn generate_he_half_times() -> [f32; 16] {
    // generate coef used to convert proportional diffusion of helium
    // from nitrogen
    let coef = n_root(28.0184 / 4.0026, 2);

    let mut v: [f32; 16] = [0_f32; 16];

    for i in 0..16 {
        v[i] = round_f32(ZHL16Compartment::N2_HALF_TIMES[i] / coef, 4);
    }

    v
}

pub fn build_air_tissue(cpt_num: usize) -> (GasMix, ZHL16Compartment) {
    let mix = GasMix::new_nitrox(PPO2);

    let tissue1 = ZHL16Compartment::new(cpt_num, mix.clone(), None);

    (mix, tissue1)
}

pub fn build_nitrox_tissue(cpt_num: usize, oxygen: f32) -> (GasMix, ZHL16Compartment) {
    let mix = GasMix::new_nitrox(oxygen);

    let tissue1 = ZHL16Compartment::new(cpt_num, mix.clone(), None);

    (mix, tissue1)
}

pub fn build_trimix_tissue(cpt_num: usize, helium: f32, oxygen: f32) -> (GasMix, ZHL16Compartment) {
    let mix = GasMix::new_trimix(helium, oxygen);

    let tissue1 = ZHL16Compartment::new(cpt_num, mix.clone(), None);

    (mix, tissue1)
}

mod test {
    use crate::tissue::TissueCompartment;

    use super::*;

    #[test]
    fn test_build_he_half_times() {
        let he_ht = generate_he_half_times();

        assert_eq!(ZHL16Compartment::HE_HALF_TIMES, he_ht);
    }

    #[test]
    fn test_build_air_tissue() {
        let (_, t): (GasMix, ZHL16Compartment) = build_air_tissue(0);
        let (n2, _) = t.n2_he_pp();
        assert_eq!(n2, PPN2);
    }

    #[test]
    fn test_build_nitrox_tissue() {
        let (_, t): (GasMix, ZHL16Compartment) = build_nitrox_tissue(0, 0.32);

        let exp = PPN2 - (0.32 - PPO2);
        let (n2, _) = t.n2_he_pp();
        assert_eq!(n2, exp);
    }

    #[test]
    fn test_build_trimix_tissue() {
        let helium = 0.10;
        let oxygen = 0.30;
        let (_, t): (GasMix, ZHL16Compartment) = build_trimix_tissue(0, helium, oxygen);

        let exp = (PPN2 - (helium + (oxygen - PPO2))) + helium;
        let (n2, he) = t.n2_he_pp();

        assert_eq!(n2 + he, exp);
    }
}
