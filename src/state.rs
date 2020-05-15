use super::*;

pub struct State {
    pub map: Map,
}

impl State {
    pub fn new() -> State {
        State { map: Map::new() }
    }
}
