use crate::gas::{Gas, GasMix};

pub struct DiveProfileLevel {
    pub gas_mix: GasMix,
    pub depth: f32, // Depth in meters
    pub time: u32,  // Time in minutes at a given depth
}

pub struct DiveProfile {
    pub levels: Vec<DiveProfileLevel>,
}

impl DiveProfile {
    pub fn new() -> Self {
        Self { levels: vec![] }
    }

    pub fn add_level(&mut self, depth: f32, time: u32, mix: GasMix) {
        self.levels.push(DiveProfileLevel {
            gas_mix: mix,
            depth,
            time,
        })
    }
}
