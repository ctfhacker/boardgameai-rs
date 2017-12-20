use super::*;
use std::fmt;
use std::fmt::Display;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy)]
pub struct BoardTile {
    pub occupied: Option<usize>,
    pub items: usize,
    pub reset_amount: usize
}

#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: HashMap<AgricolaTile, Box<BoardTile>>,
    pub future_tiles: Vec<(AgricolaTile, Box<BoardTile>)>,
}

impl Board {
    pub fn new() -> Board {
        let mut future_tiles = Vec::new();

        let round_1_tiles = vec!(
            (AgricolaTile::Fences, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
            (AgricolaTile::Sheep, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1})),
            (AgricolaTile::Sow_BakeBread, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
            (AgricolaTile::MajorImprovement, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
        );

        let round_2_tiles = vec!(
            (AgricolaTile::Stone_1, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1})),
            (AgricolaTile::FamilyGrowth, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
            (AgricolaTile::Renovation_MajorImprovement, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
        );

        let round_3_tiles = vec!(
            (AgricolaTile::Vegetable, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
            (AgricolaTile::Boar, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1})),
        );

        let round_4_tiles = vec!(
            (AgricolaTile::Cattle, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1})),
            (AgricolaTile::Stone_2, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1})),
        );

        let round_5_tiles = vec!(
            (AgricolaTile::Plow_Sow, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
            (AgricolaTile::FamilyGrowth_NoSpace, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
        );

        let round_6_tiles = vec!(
            (AgricolaTile::Renovation_Fences, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
        );

        future_tiles.extend(round_6_tiles);
        future_tiles.extend(round_5_tiles);
        future_tiles.extend(round_4_tiles);
        future_tiles.extend(round_3_tiles);
        future_tiles.extend(round_2_tiles);
        future_tiles.extend(round_1_tiles);

        let mut board = HashMap::new();
        board.insert(AgricolaTile::BuildRoom_BuildStables, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::StartingPlayer_Food, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1}));
        board.insert(AgricolaTile::Grain, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::Plow, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::BuildStable_BakeBread, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::DayLaborer, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::Wood, Box::new(BoardTile { occupied: None, items: 3, reset_amount: 3}));
        board.insert(AgricolaTile::Clay, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1}));
        board.insert(AgricolaTile::Reed, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1}));
        board.insert(AgricolaTile::Fishing, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1}));

        // Insert first action
        if let Some((next_card, next_tile)) = future_tiles.pop() {
            // println!("Next action: {:?} {:?}", next_card, next_tile);
            board.insert(next_card, next_tile);
        }

        Board {
            tiles: board,
            future_tiles: future_tiles
        }
    }

    /// Add more counters to the board and add the next action card to the board
    pub fn reset(&mut self) {
        for (name, mut tile) in &mut self.tiles {
            // println!("{:?}: {:?} -> {:?}", name, tile.items, tile.items+tile.reset_amount);
            tile.items += tile.reset_amount;
            tile.occupied = None;
        }

        if let Some((next_card, next_tile)) = self.future_tiles.pop() {
            // println!("Next action: {:?} {:?}", next_card, next_tile);
            self.tiles.insert(next_card, next_tile);
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (tile, data) in &self.tiles {
            let player = match data.occupied {
                Some(x) => format!("{}", x),
                None => String::from("NA")
            };
            write!(f, "{:?} -- Player: {:?} Items: {:?}\n", tile, player, data.items);
        }
        Ok(())
    }
}
