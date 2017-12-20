use super::*;

#[derive(Debug, Clone)]
pub struct Pasture {
    pub tiles: Vec<usize>,
    pub stables: usize,
    pub capacity: usize,
    pub kind: Option<Animal>
}

impl Pasture {
    pub fn new(tiles: Vec<usize>, stables: usize) -> Pasture {
        Pasture {
            stables: stables,
            capacity: tiles.len() * (2 + stables),
            tiles: tiles,
            kind: None
        }
    }
}
