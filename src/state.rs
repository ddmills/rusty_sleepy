use super::*;

pub struct State {
    pub map: Map,
    pub map_view_mode: MapViewMode,
}

impl State {
    pub fn new() -> State {
        State {
            map: Map::new(),
            map_view_mode: MapViewMode::Pretty,
        }
    }
}
