use super::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct State {
    pub tiles: Vec<TileType>,
}

impl State {
    pub fn new() -> State {
        State {
            tiles: vec![TileType::Floor; (WIDTH * HEIGHT) as usize],
        }
    }
}
