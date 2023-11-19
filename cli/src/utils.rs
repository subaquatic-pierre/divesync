use core::gas::GasMix;
use std::error::Error;

pub fn str_to_gas(gas_str: &str) -> Result<GasMix, Box<dyn Error>> {
    let split: Vec<&str> = gas_str.split(",").collect();

    if split.len() == 2 {
        let o2 = split[0].parse::<f32>()? / 100.0;
        let he = split[1].parse::<f32>()? / 100.0;
        Ok(GasMix::new_trimix(he, o2))
    } else {
        let o2 = split[0].parse::<f32>()? / 100.0;
        Ok(GasMix::new_nitrox(o2))
    }
}
