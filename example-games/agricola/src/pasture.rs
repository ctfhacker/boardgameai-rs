use super::*;

#[derive(Debug, Clone)]
pub struct Pasture {
    pub tiles: Vec<usize>,
    pub stables: usize,
    pub capacity: usize
}

impl Pasture {
    pub fn new(tiles: Vec<usize>, stables: usize) -> Pasture {
        Pasture {
            stables: stables,
            capacity: 2 + stables,
            tiles: tiles
        }
    }
}
