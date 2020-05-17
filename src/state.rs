use super::*;

pub struct State {
    pub world: World,
    pub world_view_mode: WorldViewMode,
    pub biome_lut: ColorLUT,
    pub is_dirty: bool,
}

impl State {
    pub fn new() -> State {
        State {
            world: World::new(),
            world_view_mode: WorldViewMode::Biome,
            biome_lut: ColorLUT::new(),
            is_dirty: true,
        }
    }
}
