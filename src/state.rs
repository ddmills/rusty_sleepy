use super::*;

pub struct State {
    pub map: Map,
    pub map_view_mode: MapViewMode,
    pub biome_lut: ColorLUT,
    pub seed: u64,
}

impl State {
    pub fn new() -> State {
        State {
            map: Map::new(),
            map_view_mode: MapViewMode::Biome,
            biome_lut: ColorLUT::new(),
            seed: 1,
        }
    }
}
