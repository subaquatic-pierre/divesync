use crate::gas::{Gas, GasMix, GasSymbol, PPN2};
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

    println!("helium half times: {v:?}");

    v
}

pub fn build_air_tissue(cpt_num: usize) -> (GasMix, ZHL16Compartment) {
    let mix = GasMix::new_nitrox(PPN2);

    let tissue1 = ZHL16Compartment::new(cpt_num, mix.clone());

    (mix, tissue1)
}

pub fn build_nitrox_tissue(cpt_num: usize, oxygen: f32) -> (GasMix, ZHL16Compartment) {
    let mix = GasMix::new_nitrox(oxygen);

    let tissue1 = ZHL16Compartment::new(cpt_num, mix.clone());

    (mix, tissue1)
}

pub fn build_trimix_tissue(cpt_num: usize, helium: f32, oxygen: f32) -> (GasMix, ZHL16Compartment) {
    let mix = GasMix::new_trimix(helium, oxygen);

    let tissue1 = ZHL16Compartment::new(cpt_num, mix.clone());

    (mix, tissue1)
}
