#![feature(exclusive_range_pattern)]
#[macro_use]
extern crate lazy_static;
extern crate boardgameai_rs;
extern crate rand;

use boardgameai_rs::*;
use boardgameai_rs::state::State;
use boardgameai_rs::action::Action;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Display;
use rand::{thread_rng, Rng, sample};

lazy_static!{
    // Create a HashMap of each fence edge that correlates the surrounding
    // fences. By checking if both ends of a fence are occupied, we can
    // guarentee we have a fence that is closed.
    //
    // For example: Fence 6 North:
    // One end point has (1 West, 6 West, 5 North) and 
    // the other end point has (1 East, 6 east, 7 North)
    //
    // We can also leverage this mapping to randomly select a fence to place.
    // Flatten the two endpoint vectors to have a vector of all possible
    // surrounding fences and then randomly select one of those, ensures
    // that the random fence structure is contiguous.
    // static ref FENCE_MAP: HashMap<(&(usize, &str), &str> = {
    static ref FENCE_MAP: HashMap<(usize, &'static str), (Vec<(usize, &'static str)>, Vec<(usize, &'static str)>)> = {
        let mut adjacent_fences = HashMap::new();
        adjacent_fences.insert((0, "north"), (vec!((0, "west")), vec!((0, "east"), (1, "north"))));
        adjacent_fences.insert((1, "north"), (vec!((0, "north"), (1, "west")), vec!((1, "east"), (2, "north"))));
        adjacent_fences.insert((2, "north"), (vec!((1, "north"), (2, "west")), vec!((2, "east"), (3, "north"))));
        adjacent_fences.insert((3, "north"), (vec!((2, "north"), (3, "west")), vec!((3, "east"), (4, "north"))));
        adjacent_fences.insert((4, "north"), (vec!((3, "north"), (4, "west")), vec!((4, "east"))));

        adjacent_fences.insert((0, "west"), (vec!((0, "north")), vec!((0, "south"), (5, "west"))));
        adjacent_fences.insert((0, "east"), (vec!((0, "north")), vec!((0, "south"), (1, "north"), (1, "south"), (5, "east"))));
        adjacent_fences.insert((1, "east"), (vec!((1, "north")), vec!((1, "south"), (2, "north"), (2, "south"), (6, "east"))));
        adjacent_fences.insert((2, "east"), (vec!((2, "north")), vec!((2, "south"), (3, "north"), (3, "south"), (7, "east"))));
        adjacent_fences.insert((3, "east"), (vec!((3, "north")), vec!((3, "south"), (4, "north"), (4, "south"), (8, "east"))));
        adjacent_fences.insert((4, "east"), (vec!((4, "north")), vec!((4, "south"), (9, "east"))));

        adjacent_fences.insert((5, "north"), (vec!((0, "west"), (5, "west")), vec!((0, "east"), (5, "east"), (6, "north"))));
        adjacent_fences.insert((6, "north"), (vec!((1, "west"), (6, "west"), (5, "north")), vec!((1, "east"), (6, "east"), (7, "north"))));
        adjacent_fences.insert((7, "north"), (vec!((2, "west"), (7, "west"), (6, "north")), vec!((2, "east"), (7, "east"), (8, "north"))));
        adjacent_fences.insert((8, "north"), (vec!((3, "west"), (8, "west"), (7, "north")), vec!((3, "east"), (8, "east"), (9, "north"))));
        adjacent_fences.insert((9, "north"), (vec!((4, "west"), (8, "north"), (9, "west")), vec!((4, "east"), (9, "east"))));

        adjacent_fences.insert((5, "west"), (vec!((5, "north"), (0, "west")), vec!((5, "south"), (10, "west"))));
        adjacent_fences.insert((5, "east"), (vec!((5, "north"), (0, "east"), (6, "north")), vec!((5, "south"), (6, "south"), (10, "east"))));
        adjacent_fences.insert((6, "east"), (vec!((6, "north"), (1, "east"), (7, "north")), vec!((6, "south"), (7, "south"), (11, "east"))));
        adjacent_fences.insert((7, "east"), (vec!((7, "north"), (2, "east"), (8, "north")), vec!((7, "south"), (8, "south"), (12, "east"))));
        adjacent_fences.insert((8, "east"), (vec!((8, "north"), (3, "east"), (9, "north")), vec!((8, "south"), (9, "south"), (13, "east"))));
        adjacent_fences.insert((9, "east"), (vec!((9, "north"), (4, "east")), vec!((9, "south"), (14, "east"))));

        adjacent_fences.insert((10, "north"), (vec!((5, "west"), (10, "west")), vec!((5, "east"), (10, "east"), (11, "north"))));
        adjacent_fences.insert((11, "north"), (vec!((6, "west"), (11, "west"), (10, "north")), vec!((11, "east"), (6, "east"), (12, "north"))));
        adjacent_fences.insert((12, "north"), (vec!((7, "west"), (12, "west"), (11, "north")), vec!((12, "east"), (7, "east"), (13, "north"))));
        adjacent_fences.insert((13, "north"), (vec!((8, "west"), (13, "west"), (12, "north")), vec!((13, "east"), (8, "east"), (14, "north"))));
        adjacent_fences.insert((14, "north"), (vec!((9, "west"), (14, "west"),  (13, "north")), vec!((14, "east"), (9, "east"))));

        adjacent_fences.insert((10, "west"), (vec!((10, "north"), (5, "west")), vec!((10, "south"))));
        adjacent_fences.insert((10, "east"), (vec!((10, "north"), (11, "north"), (5, "east")), vec!((10, "south"), (11, "south"))));
        adjacent_fences.insert((11, "east"), (vec!((11, "north"), (12, "north"), (6, "east")), vec!((11, "south"), (12, "south"))));
        adjacent_fences.insert((12, "east"), (vec!((12, "north"), (13, "north"), (7, "east")), vec!((12, "south"), (13, "south"))));
        adjacent_fences.insert((13, "east"), (vec!((13, "north"), (14, "north"), (8, "east")), vec!((13, "south"), (14, "south"))));
        adjacent_fences.insert((14, "east"), (vec!((14, "north"), (9, "west")), vec!((14, "south"))));

        adjacent_fences.insert((10, "south"), (vec!((10, "west")), vec!((10, "east"), (11, "south"))));
        adjacent_fences.insert((11, "south"), (vec!((11, "west"), (10, "south")), vec!((11, "east"), (12, "south"))));
        adjacent_fences.insert((12, "south"), (vec!((12, "west"), (11, "south")), vec!((12, "east"), (13, "south"))));
        adjacent_fences.insert((13, "south"), (vec!((13, "west"), (12, "south")), vec!((13, "east"), (14, "south"))));
        adjacent_fences.insert((14, "south"), (vec!((14, "west"), (13, "south")), vec!((14, "east"))));
        adjacent_fences
    };
}

#[derive(Debug, Clone)]
struct Player {
    food: usize,
    fields: usize,
    grains: usize,
    vegetables: usize,
    wood: usize,
    clay: usize,
    reed: usize,
    stone: usize,
    sheep: usize,
    cattle: usize,
    boar: usize,
    actions: usize,
    total_actions: usize,
    actions_taken: Vec<String>,
    player_mat: PlayerMat,
    house_type: HouseType,
    beggers: usize,
    children: usize,
    fences: usize,
    pastures: Vec<Vec<usize>>
}

impl Player {
    fn new(food: usize) -> Player {
        Player {
            food: food,
            fields: 0,
            grains: 0,
            vegetables: 0,
            wood: 0,
            clay: 0,
            reed: 0,
            stone: 0,
            sheep: 0,
            boar: 0,
            cattle: 0,
            actions: 2,
            total_actions: 2,
            actions_taken: Vec::new(),
            player_mat: PlayerMat::new(),
            house_type: HouseType::Wood,
            beggers: 0,
            children: 0,
            fences: 0
        }
    }

    fn score(&self) -> i32 {
        let mut result: i32 = 0;
        match self.fields {
            0|1 => result -= 1,
            2   => result += 1,
            3   => result += 2,
            4   => result += 3,
            _   => result += 4
        }
        let grain_in_fields: usize = self.player_mat.tiles.iter()
                                            .filter(|t| t.field.is_some())
                                            .map(|t| t.clone().field.unwrap().count)
                                            .sum();

        match (self.grains + grain_in_fields) {
            0     => result -= 1,
            1|2|3 => result += 1,
            4|5   => result += 2,
            6|7   => result += 3,
            _     => result += 4,
        };
        match self.vegetables {
            0 => result -= 1,
            1 => result += 1,
            2 => result += 2,
            3 => result += 3,
            _ => result += 4,
        };
        match self.sheep {
            0     => result -= 1,
            1|2|3 => result += 1,
            4|5   => result += 2,
            6|7   => result += 3,
            _     => result += 4,
        };
        match self.boar {
            0   => result -= 1,
            1|2 => result += 1,
            3|4 => result += 2,
            5|6 => result += 3,
            _   => result += 4,
        };
        match self.cattle {
            0   => result -= 1,
            1   => result += 1,
            2|3 => result += 2,
            4|5 => result += 3,
            _   => result += 4,
        };

        let empty_spaces: Vec<&FarmTile> = self.player_mat.tiles.iter()
                                                                .filter(|&t| t.is_empty())
                                                                .collect(); 

        result -= empty_spaces.len() as i32;

        // TODO fenced in stables
        let num_rooms = self.player_mat.tiles.iter()
                                             .filter(|t| t.house.is_some())
                                             .count();
        
        match self.house_type {
            HouseType::Wood => {},
            HouseType::Clay => result += (num_rooms * 1) as i32,
            HouseType::Stone => result += (num_rooms * 2) as i32,
        }

        result -= (self.beggers * 3) as i32;

        result += (self.fences * 3) as i32;

        result += (self.total_actions * 3) as i32;
        result
    }

    /// Randomly plow a field if none exists. If a field already exists, plow a random field
    /// connected to an existing field
    fn plow(&mut self) {
        {
            let empty_spaces: Vec<&FarmTile> = self.player_mat.tiles.iter()
                                                                    .filter(|t| t.is_empty() && 
                                                                                t.north_fence == false &&
                                                                                t.south_fence == false &&
                                                                                t.west_fence == false &&
                                                                                t.east_fence == false)
                                                                    .collect(); 
            if empty_spaces.len() == 0 {
                // println!("Cannot plow field... no more empty spaces");
                return;
            }
        }
        if self.fields == 0 {
            loop {
                let num = thread_rng().gen_range(0, 15);
                if self.player_mat.tiles[num].is_empty() {
                    self.player_mat.tiles[num].plow();
                    break;
                }
            }
        } else {
            self.player_mat.plow_random_field();
        }
        self.fields += 1;
    }

    fn can_build_room(&self) -> bool {
        if self.reed < 2 { return false; }
        match self.house_type {
            HouseType::Wood => if self.wood < 5 { return false; },
            HouseType::Clay => if self.clay < 5 { return false; },
            HouseType::Stone => if self.stone < 5 { return false; },
        }
        return true;
    }

    fn build_room(&mut self) {
        if !self.can_build_room() {
            // println!("Not enough resources to build room.. no action");
        }

        let curr_rooms: Vec<usize> = self.player_mat.tiles
                                            .iter()
                                            .enumerate()
                                            .filter(|&(i, t)| !(t.house.is_none()))
                                            .map(|(i, t)| i)
                                            .collect();

        // Filter surrounding tiles if they are empty
        let possible_rooms: Vec<usize> = curr_rooms.iter()
                                                    .flat_map(|&i| self.player_mat.tiles[i].surrounding_tiles.clone())
                                                    .filter(|&i| self.player_mat.tiles[i].is_empty() && 
                                                                 self.player_mat.tiles[i].north_fence == false &&
                                                                 self.player_mat.tiles[i].south_fence == false &&
                                                                 self.player_mat.tiles[i].west_fence == false &&
                                                                 self.player_mat.tiles[i].east_fence == false)
                                                    .collect();

        if possible_rooms.len() == 0 {
            return;
        }

        let random_room = rand::thread_rng().choose(&possible_rooms).unwrap();
        self.player_mat.tiles[*random_room].build_room(self.house_type.clone());
    }

    fn build_stables(&mut self) {
        let max_stables = self.wood / 2;
        let num_stables = rand::random::<usize>() % (max_stables+1);
        for _ in 0..num_stables {
            let possibles: Vec<usize> = self.player_mat.tiles
                                                .iter()
                                                .enumerate()
                                                .filter(|&(i, t)| t.is_empty())
                                                .map(|(i, t)| i)
                                                .collect();

            if possibles.len() == 0 {
                return;
            }

            let random_tile = rand::thread_rng().choose(&possibles).unwrap();
            self.player_mat.tiles[*random_tile].stable();
            self.wood -= 2;
        }
    }

    fn build_stable(&mut self) {
        if self.wood == 0 {
            // Not enough wood to buy one stable
            return;
        }

        let possibles: Vec<usize> = self.player_mat.tiles
                                                .iter()
                                                .enumerate()
                                                .filter(|&(i, t)| t.is_empty())
                                                .map(|(i, t)| i)
                                                .collect();

        if possibles.len() == 0 {
            return;
        }

        let random_tile = rand::thread_rng().choose(&possibles).unwrap();
        self.player_mat.tiles[*random_tile].stable();
        self.wood -= 1;
    }

    fn sow(&mut self) {
        let mut empty_fields: Vec<usize> = self.player_mat.tiles.iter()
                                                                .enumerate()
                                                                .filter(|&(i, t)| t.field.is_some() && t.clone().field.unwrap().count == 0)
                                                                .map(|(i, f)| i)
                                                                .collect();

        while empty_fields.len() > 0 && self.vegetables > 0 {
            let curr_field_index = empty_fields.pop().unwrap();
            self.sow_veg(curr_field_index);
            self.vegetables -= 1;
        }

        while empty_fields.len() > 0 && self.grains > 0 {
            let curr_field_index = empty_fields.pop().unwrap();
            self.sow_grain(curr_field_index);
            self.grains -= 1;
        }
    }

    fn sow_veg(&mut self, index: usize) {
        self.player_mat.tiles[index].sow_veg();
    }

    fn sow_grain(&mut self, index: usize) {
        self.player_mat.tiles[index].sow_grain();
    }

    fn upgrade_house(&mut self) {
        for tile in self.player_mat.tiles.iter_mut() {
            if tile.house.is_some() {
                tile.upgrade()
            }
        }

        match self.house_type {
            HouseType::Wood => self.house_type = HouseType::Clay,
            HouseType::Clay => self.house_type = HouseType::Stone,
            HouseType::Stone => {}
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Food: {} Wood: {} Clay: {} Reed: {} Stone: {} Actions: {}/{} Fields: {} Beggers: {}]\n",
                self.food, self.wood, self.clay, self.reed, self.stone, self.actions, self.total_actions, self.fields, self.beggers);
        write!(f, "[Grain: {} Veg: {}]\n", self.grains, self.vegetables);
        write!(f, "[Sheep: {} Boar: {} Cattle: {}]\n", self.sheep, self.boar, self.cattle);
        write!(f, "[Fences: {}]\n", self.fences);
        write!(f, "{}", self.player_mat)
    }
}

#[derive(Debug, Clone, Copy)]
struct PlayerId {
    index: usize
}

#[derive(Debug, Clone, Copy)]
struct BoardTile {
    occupied: Option<usize>,
    items: usize,
    reset_amount: usize
}



#[derive(Debug, Clone)]
pub struct Board {
    tiles: HashMap<AgricolaTile, Box<BoardTile>>,
    future_tiles: Vec<(AgricolaTile, Box<BoardTile>)>,
}

impl Board {
    fn new() -> Board {
        let mut future_tiles = Vec::new();

        let round_1_tiles = vec!(
            (AgricolaTile::Sow_BakeBread, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
            (AgricolaTile::Fences, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
            (AgricolaTile::MajorImprovement, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
            (AgricolaTile::Sheep, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1})),
        );

        let round_2_tiles = vec!(
            (AgricolaTile::FamilyGrowth, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0})),
            (AgricolaTile::Stone_1, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1})),
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

    fn reset(&mut self) {
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum AgricolaTile {
    BuildRoom_BuildStables = 1,
    StartingPlayer_Food = 2,
    Grain = 3,
    Plow = 4,
    BuildStable_BakeBread = 5,
    DayLaborer = 6,
    Sow_BakeBread = 7, 
    Wood = 8,
    Clay = 9,
    Reed = 10,
    Fishing = 11,
    Fences = 12,
    MajorImprovement = 13,
    Sheep = 14,
    FamilyGrowth = 15,
    Stone_1 = 16,
    Renovation_MajorImprovement = 17,
    Vegetable = 18,
    Boar = 19,
    Cattle = 20,
    Stone_2 = 21,
    Plow_Sow = 22,
    FamilyGrowth_NoSpace = 23,
    Renovation_Fences = 24
}

#[derive(Debug)]
pub enum AgricolaAction {
    BuildRoom_BuildStables = 1,
    BuildRoom = 15,
    BuildStables = 16,
    StartingPlayer_Food = 2,
    Grain = 3,
    Plow = 4,
    BuildStable_BakeBread = 5,
    BuildStable = 19,
    BakeBread_NoStable = 20,
    DayLaborer_Food_Wood = 6,
    DayLaborer_Food_Clay = 7,
    DayLaborer_Food_Reed = 8,
    DayLaborer_Food_Stone = 9,
    Sow_BakeBread = 10,
    Sow = 17,
    BakeBread_NotSow = 18,
    Wood = 11,
    Clay = 12,
    Reed = 13,
    Fishing = 14,
    Fences_1 = 21,
    Fences_2 = 36,
    Fences_3 = 37,
    Fences_4 = 38,
    Fences_5 = 39,
    Fences_6 = 40,
    Fences_7 = 41,
    Fences_8 = 42,
    Fences_9 = 43,
    Fences_10 = 44,
    Fences_11 = 45,
    Fences_12 = 46,
    Fences_13 = 47,
    Fences_14 = 48,
    Fences_15 = 49,
    MajorImprovement = 22,
    Sheep = 23,
    FamilyGrowth = 24,
    Stone_1 = 25,
    Renovation_MajorImprovement = 26,
    Vegetable = 27,
    Boar = 28,
    Cattle = 29,
    Stone_2 = 30,
    Plow_Sow = 31,
    Plow_NoSow = 34,
    Sow_NoPlow = 35,
    FamilyGrowth_NoSpace = 32,
    Renovation_Fences = 33,
}

impl AgricolaAction {
    pub fn from_u32(x: u32) -> Option<AgricolaAction> {
        match x {
            1 => Some(AgricolaAction::BuildRoom_BuildStables),
            15 => Some(AgricolaAction::BuildRoom),
            16 => Some(AgricolaAction::BuildStables),
            2 => Some(AgricolaAction::StartingPlayer_Food),
            3 => Some(AgricolaAction::Grain),
            4 => Some(AgricolaAction::Plow),
            5 => Some(AgricolaAction::BuildStable_BakeBread),
            19 => Some(AgricolaAction::BuildStable),
            20 => Some(AgricolaAction::BakeBread_NoStable),
            6 => Some(AgricolaAction::DayLaborer_Food_Wood),
            7 => Some(AgricolaAction::DayLaborer_Food_Clay),
            8 => Some(AgricolaAction::DayLaborer_Food_Reed),
            9 => Some(AgricolaAction::DayLaborer_Food_Stone),
            10 => Some(AgricolaAction::Sow_BakeBread),
            17 => Some(AgricolaAction::Sow),
            18 => Some(AgricolaAction::BakeBread_NotSow),
            11 => Some(AgricolaAction::Wood),
            12 => Some(AgricolaAction::Clay),
            13 => Some(AgricolaAction::Reed),
            14 => Some(AgricolaAction::Fishing),
            21 => Some(AgricolaAction::Fences_1),
            36 => Some(AgricolaAction::Fences_2),
            37 => Some(AgricolaAction::Fences_3),
            38 => Some(AgricolaAction::Fences_4),
            39 => Some(AgricolaAction::Fences_5),
            40 => Some(AgricolaAction::Fences_6),
            41 => Some(AgricolaAction::Fences_7),
            42 => Some(AgricolaAction::Fences_8),
            43 => Some(AgricolaAction::Fences_9),
            44 => Some(AgricolaAction::Fences_10),
            45 => Some(AgricolaAction::Fences_11),
            46 => Some(AgricolaAction::Fences_12),
            47 => Some(AgricolaAction::Fences_13),
            48 => Some(AgricolaAction::Fences_14),
            49 => Some(AgricolaAction::Fences_15),
            22 => Some(AgricolaAction::MajorImprovement),
            23 => Some(AgricolaAction::Sheep),
            24 => Some(AgricolaAction::FamilyGrowth),
            25 => Some(AgricolaAction::Stone_1),
            26 => Some(AgricolaAction::Renovation_MajorImprovement),
            27 => Some(AgricolaAction::Vegetable),
            28 => Some(AgricolaAction::Boar),
            29 => Some(AgricolaAction::Cattle),
            30 => Some(AgricolaAction::Stone_2),
            31 => Some(AgricolaAction::Plow_Sow),
            34 => Some(AgricolaAction::Plow_NoSow),
            35 => Some(AgricolaAction::Sow_NoPlow),
            32 => Some(AgricolaAction::FamilyGrowth_NoSpace),
            33 => Some(AgricolaAction::Renovation_Fences),
            _ => None
        }
    }
}


#[derive(Debug, Clone)]
pub struct AgricolaState {
    players: Vec<Player>,
    player_just_moved: usize,
    pub current_player: usize,
    starting_player_token: Option<usize>,
    pub board: Board,
    rounds: usize,
    total_rounds: usize,
    actions_taken: Vec<String>
}

impl State for AgricolaState {
    fn get_actions(&self) -> Vec<u32> {
        if self.rounds > self.total_rounds {
            // Game over!
            return Vec::new();
        }

        let mut actions = Vec::new();

        for (tile, board_tile) in &(self.board.tiles) {
            if board_tile.occupied.is_none() {
                match tile {
                    &AgricolaTile::DayLaborer => {
                        actions.push(AgricolaAction::DayLaborer_Food_Wood as u32);
                        actions.push(AgricolaAction::DayLaborer_Food_Clay as u32);
                        actions.push(AgricolaAction::DayLaborer_Food_Reed as u32);
                        actions.push(AgricolaAction::DayLaborer_Food_Stone as u32);
                    },
                    &AgricolaTile::BuildRoom_BuildStables => {
                        actions.push(AgricolaAction::BuildRoom as u32);
                        actions.push(AgricolaAction::BuildStables as u32);
                        actions.push(AgricolaAction::BuildRoom_BuildStables as u32);
                    }
                    &AgricolaTile::StartingPlayer_Food => actions.push(AgricolaAction::StartingPlayer_Food as u32),
                    &AgricolaTile::Grain => actions.push(AgricolaAction::Grain as u32),
                    &AgricolaTile::Plow  => actions.push(AgricolaAction::Plow as u32),
                    &AgricolaTile::BuildStable_BakeBread  => {
                        actions.push(AgricolaAction::BuildStable_BakeBread as u32);
                        actions.push(AgricolaAction::BuildStable as u32);
                        actions.push(AgricolaAction::BakeBread_NoStable as u32);
                    }
                    &AgricolaTile::Sow_BakeBread  => {
                        actions.push(AgricolaAction::Sow_BakeBread as u32);
                        actions.push(AgricolaAction::BakeBread_NotSow as u32);
                        actions.push(AgricolaAction::Sow as u32);
                    },
                    &AgricolaTile::Wood  => actions.push(AgricolaAction::Wood as u32),
                    &AgricolaTile::Clay  => actions.push(AgricolaAction::Clay as u32),
                    &AgricolaTile::Reed  => actions.push(AgricolaAction::Reed as u32),
                    &AgricolaTile::Fishing  => actions.push(AgricolaAction::Fishing as u32),
                    &AgricolaTile::Fences  => {
                        actions.push(AgricolaAction::Fences_1 as u32);
                        actions.push(AgricolaAction::Fences_2 as u32);
                        actions.push(AgricolaAction::Fences_3 as u32);
                        actions.push(AgricolaAction::Fences_4 as u32);
                        actions.push(AgricolaAction::Fences_5 as u32);
                        actions.push(AgricolaAction::Fences_6 as u32);
                        actions.push(AgricolaAction::Fences_7 as u32);
                        actions.push(AgricolaAction::Fences_8 as u32);
                        actions.push(AgricolaAction::Fences_9 as u32);
                        actions.push(AgricolaAction::Fences_10 as u32);
                        actions.push(AgricolaAction::Fences_11 as u32);
                        actions.push(AgricolaAction::Fences_12 as u32);
                        actions.push(AgricolaAction::Fences_13 as u32);
                        actions.push(AgricolaAction::Fences_14 as u32);
                        actions.push(AgricolaAction::Fences_15 as u32);
                    },
                    &AgricolaTile::MajorImprovement  => actions.push(AgricolaAction::MajorImprovement as u32),
                    &AgricolaTile::Sheep  => actions.push(AgricolaAction::Sheep as u32),
                    &AgricolaTile::FamilyGrowth  => actions.push(AgricolaAction::FamilyGrowth as u32),
                    &AgricolaTile::Stone_1  => actions.push(AgricolaAction::Stone_1 as u32),
                    &AgricolaTile::Renovation_MajorImprovement  => actions.push(AgricolaAction::Renovation_MajorImprovement as u32),
                    &AgricolaTile::Vegetable  => actions.push(AgricolaAction::Vegetable as u32),
                    &AgricolaTile::Boar  => actions.push(AgricolaAction::Boar as u32),
                    &AgricolaTile::Cattle  => actions.push(AgricolaAction::Cattle as u32),
                    &AgricolaTile::Stone_2  => actions.push(AgricolaAction::Stone_2 as u32),
                    &AgricolaTile::Plow_Sow  => {
                        actions.push(AgricolaAction::Plow_Sow as u32);
                        actions.push(AgricolaAction::Plow_NoSow as u32);
                        actions.push(AgricolaAction::Sow_NoPlow as u32);
                    },
                    &AgricolaTile::FamilyGrowth_NoSpace  => actions.push(AgricolaAction::FamilyGrowth_NoSpace as u32),
                    &AgricolaTile::Renovation_Fences  => actions.push(AgricolaAction::Renovation_Fences as u32),
                }
            }
        }

        actions
    }

    fn get_player_just_moved(&self) -> usize {
        self.player_just_moved
    }

    fn get_action_strings(&self) -> Vec<String> {
        let mut strings = Vec::new();
        for action in self.get_actions() {
            strings.push(format!("{:?}", AgricolaAction::from_u32(action).unwrap()));
        }
        strings
    }

    fn do_action(&mut self, action: u32) {
        if self.players[self.current_player].actions == 0 {
            panic!("Oh noes.. attempting to play a piece with no actions. :(");
        }
        println!("[R:{} P:{}] Action: {} {:?}", self.rounds, self.current_player, action, AgricolaAction::from_u32(action));
        let player_index = self.current_player;
        let num_players = self.players.len();
        let mut action_taken = String::from("");
        {
            let mut player = &mut self.players[player_index];
            let agricola_action = AgricolaAction::from_u32(action);
            let mut curr_tile;
            match agricola_action {
                Some(AgricolaAction::Grain) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Grain).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. Grain is already taken", player_index);
                    }
                    player.grains += 1;
                    action_taken = String::from("Grain +1");
                },
                Some(AgricolaAction::Wood) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Wood).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. Wood is already taken", player_index);
                    }
                    player.wood += curr_tile.items;
                    action_taken = format!("Wood +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Clay) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Clay).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. clay is already taken", player_index);
                    }
                    player.clay += curr_tile.items;
                    action_taken = format!("Clay +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Reed) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Reed).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. reed is already taken", player_index);
                    }
                    player.reed += curr_tile.items;
                    action_taken = format!("Reed +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Fishing) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Fishing).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. fishing is already taken", player_index);
                    }
                    player.food += curr_tile.items;
                    action_taken = format!("Food (Fishing) +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::DayLaborer_Food_Wood) |
                Some(AgricolaAction::DayLaborer_Food_Clay) |
                Some(AgricolaAction::DayLaborer_Food_Reed) |
                Some(AgricolaAction::DayLaborer_Food_Stone) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::DayLaborer).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. day_laborer is already taken", player_index);
                    }
                    player.food += 1;
                    match agricola_action {
                        Some(AgricolaAction::DayLaborer_Food_Wood) => {
                            action_taken = format!("Day Laborer Food +1 Wood +1").to_string();
                            player.wood += 1;
                        },
                        Some(AgricolaAction::DayLaborer_Food_Clay) => {
                            action_taken = format!("Day Laborer Food +1 Clay +1").to_string();
                            player.clay += 1;
                        },
                        Some(AgricolaAction::DayLaborer_Food_Reed) => {
                            action_taken = format!("Day Laborer Food +1 Reed +1").to_string();
                            player.reed += 1;
                        },
                        Some(AgricolaAction::DayLaborer_Food_Stone) => {
                            action_taken = format!("Day Laborer Food +1 Stone +1").to_string();
                            player.stone += 1;
                        },
                        _ => panic!("Should never get here.. Day Laborer only has 4 choices..")
                    }
                },
                Some(AgricolaAction::Sow) |
                Some(AgricolaAction::BakeBread_NotSow) |
                Some(AgricolaAction::Sow_BakeBread) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Sow_BakeBread).unwrap());
                    match agricola_action {
                        Some(AgricolaAction::Sow) => {
                            player.sow();
                            action_taken = format!("Sow").to_string();
                        },
                        Some(AgricolaAction::BakeBread_NotSow) => {
                            action_taken = format!("Bake Bread and not Sow").to_string();
                        },
                        Some(AgricolaAction::Sow_BakeBread) =>  {
                            player.sow();
                            action_taken = format!("Sow and Bake Bread").to_string();
                        },
                        _ => panic!("Should never get here.. Sow and Bake Bread only had 3 choices..")
                    }
                },
                Some(AgricolaAction::BuildRoom) |
                Some(AgricolaAction::BuildStables) |
                Some(AgricolaAction::BuildRoom_BuildStables) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::BuildRoom_BuildStables).unwrap());
                    match agricola_action {
                        Some(AgricolaAction::BuildRoom) => {
                            player.build_room();
                            action_taken = format!("Build Room").to_string();
                        },
                        Some(AgricolaAction::BuildStables) => {
                            player.build_stables();
                            action_taken = format!("Build Stables").to_string();
                        },
                        Some(AgricolaAction::BuildRoom_BuildStables) => {
                            player.build_room();
                            player.build_stables();
                            action_taken = format!("Build Room and Stables").to_string();
                        },
                        _ => panic!("[BuildRoom_BuildStables] Can never reach here..")
                    }

                },
                Some(AgricolaAction::StartingPlayer_Food) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::StartingPlayer_Food).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. starting player and food is already taken", player_index);
                    }
                    player.food += curr_tile.items;
                    action_taken = format!("Starting Player and Food +1").to_string();
                    self.starting_player_token = Some(self.current_player);
                },
                Some(AgricolaAction::Plow) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Plow).unwrap());
                    player.plow();
                    player.fields += 1;
                    action_taken = format!("Plow").to_string();
                },
                Some(AgricolaAction::BuildStable) |
                Some(AgricolaAction::BakeBread_NoStable) |
                Some(AgricolaAction::BuildStable_BakeBread) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::BuildStable_BakeBread).unwrap());
                    match agricola_action {
                        Some(AgricolaAction::BuildStable) => {
                            player.build_stable();
                            action_taken = format!("Build 1 stable").to_string();
                        }
                        Some(AgricolaAction::BakeBread_NoStable) => {
                            action_taken = format!("Bake Bread").to_string();
                        }
                        Some(AgricolaAction::BuildStable_BakeBread) => {
                            player.build_stable();
                            action_taken = format!("Build Stable and Bake Bread").to_string();
                        },
                        _ => panic!("[BuildStable_BakeBread] Can never reach here..")
                    }
                },
                Some(AgricolaAction::Sheep) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Sheep).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. sheep is already taken", player_index);
                    }
                    player.sheep += curr_tile.items;
                    action_taken = format!("Sheep +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::MajorImprovement) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::MajorImprovement).unwrap());
                    action_taken = format!("Major Improvement").to_string();
                },
                Some(AgricolaAction::Fences_1) |
                Some(AgricolaAction::Fences_2) |
                Some(AgricolaAction::Fences_3) |
                Some(AgricolaAction::Fences_4) |
                Some(AgricolaAction::Fences_5) |
                Some(AgricolaAction::Fences_6) |
                Some(AgricolaAction::Fences_7) |
                Some(AgricolaAction::Fences_8) |
                Some(AgricolaAction::Fences_9) |
                Some(AgricolaAction::Fences_10) |
                Some(AgricolaAction::Fences_11) |
                Some(AgricolaAction::Fences_12) |
                Some(AgricolaAction::Fences_13) |
                Some(AgricolaAction::Fences_14) |
                Some(AgricolaAction::Fences_15) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Fences).unwrap());
                    let mut orig_num_fences = 0;

                    match agricola_action {
                        Some(AgricolaAction::Fences_1) => { orig_num_fences = 1; },
                        Some(AgricolaAction::Fences_2) => { orig_num_fences = 2; },
                        Some(AgricolaAction::Fences_3) => { orig_num_fences = 3; },
                        Some(AgricolaAction::Fences_4) => { orig_num_fences = 4; },
                        Some(AgricolaAction::Fences_5) => { orig_num_fences = 5; },
                        Some(AgricolaAction::Fences_6) => { orig_num_fences = 6; },
                        Some(AgricolaAction::Fences_7) => { orig_num_fences = 7; },
                        Some(AgricolaAction::Fences_8) => { orig_num_fences = 8; },
                        Some(AgricolaAction::Fences_9) => { orig_num_fences = 9; },
                        Some(AgricolaAction::Fences_10) => { orig_num_fences = 10; },
                        Some(AgricolaAction::Fences_11) => { orig_num_fences = 11; },
                        Some(AgricolaAction::Fences_12) => { orig_num_fences = 12; },
                        Some(AgricolaAction::Fences_13) => { orig_num_fences = 13; },
                        Some(AgricolaAction::Fences_14) => { orig_num_fences = 14; },
                        Some(AgricolaAction::Fences_15) =>  { orig_num_fences = 15; },
                        _ => panic!("No other options for fences"),
                    }

                    if orig_num_fences > player.wood {
                        orig_num_fences = player.wood;
                    }

                    if orig_num_fences > (15 - player.fences) {
                        orig_num_fences = 15 - player.fences;
                    }
                    

                    println!("Start_Num fences: {}", orig_num_fences);
                    let mut i = 0;
                    let mut fences_worked = false;
                    for i in 0..3 {
                        let mut fence_places = Vec::new();
                        let mut temp_player_mat = player.player_mat.clone();
                        let mut num_fences = orig_num_fences;
                        println!("[{:?}] RESET... {}", agricola_action, i);

                        loop {
                            println!("[player.fences {}] Num fences: {}", player.fences, num_fences);

                            if num_fences == 0 {
                                break;
                            }
                            let mut fence_index = 0;
                            // No fences yet, random choice
                            if player.fences == 0 {
                                fence_index = rand::random::<usize>() % 60;
                            } else {
                                // Subsequent fences are placed connected to another fence
                                let current_fences = temp_player_mat.current_fences();

                                println!("current_fences: {:?}", current_fences);
                                println!("Player Mat:\n{}", temp_player_mat);

                                let mut curr_adjacent_fences = HashSet::new();

                                for curr_fence in current_fences {
                                    if let Some(&(ref endpoint_1, ref endpoint_2)) = FENCE_MAP.get(&curr_fence) {
                                        // TODO: Why do these loops work and not the iters :(
                                        for x in endpoint_1 {
                                            curr_adjacent_fences.insert(x);
                                        }

                                        for x in endpoint_2 {
                                            curr_adjacent_fences.insert(x);
                                        }
                                        
                                        // endpoint_1.iter().map(|x| curr_adjacent_fences.insert(x));
                                        // endpoint_2.iter().map(|x| curr_adjacent_fences.insert(x));
                                    }
                                }

                                let mut rng = thread_rng();
                                let sample = sample(&mut rng, curr_adjacent_fences, 1);
                                match sample.len() {
                                    1 => {
                                        let &(tile_index, direction) = sample[0];
                                        let tile_index_modifier = match direction {
                                            "north" => 0,
                                            "west" => 1,
                                            "south" => 2,
                                            "east" => 3,
                                            _ => panic!("Received a direction for fence that is unknown..")
                                        };
                                        fence_index = tile_index * 4 + tile_index_modifier;
                                        println!("Sample: {:?} fence_index: {:?}", sample, fence_index);
                                    },
                                    _ => panic!("Random fence sample failed..")
                                }
                            }

                            println!("Trying fence index: {}", fence_index);
                            if let Some(index) = temp_player_mat.place_fence(fence_index) {
                                num_fences -= 1;
                                fence_places.push(index.clone());
                            }
                        }

                    println!("{}", temp_player_mat.valid_fences());
                    println!("{}", temp_player_mat);
                    println!("Fence Places: {:?}", fence_places);

                    if fence_places.len() == 0 {
                        continue;
                    } 
                    if temp_player_mat.valid_fences() == true {
                        action_taken = format!("{:?} +{} {:?}", agricola_action, fence_places.len(), fence_places).to_string();

                        for fence_place in fence_places {
                            player.wood -= 1;
                            player.fences += 1;
                            player.player_mat.place_fence(fence_place);
                        }

                        println!("FOUND FOUND FOUND!!!");
                        println!("Player:\n{}", player);
                        fences_worked = true;
                        break;
                    }
                }
                },
                Some(AgricolaAction::FamilyGrowth) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::FamilyGrowth).unwrap());
                    let num_rooms = player.player_mat.tiles.iter()
                                                           .filter(|t| t.house.is_some())
                                                           .count();
                    if num_rooms > player.total_actions {
                        player.children = 1;
                    }
                    action_taken = format!("Family Growth").to_string();
                },
                Some(AgricolaAction::Stone_1) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Stone_1).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. first Stone is already taken", player_index);
                    }
                    player.stone += curr_tile.items;
                    action_taken = format!("(First) Stone +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Renovation_MajorImprovement) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Renovation_MajorImprovement).unwrap());

                    let num_rooms = player.player_mat.tiles.iter()
                                                           .filter(|t| t.house.is_some())
                                                           .count();


                    match player.house_type {
                        HouseType::Wood => {
                            if player.clay >= num_rooms && player.reed >= 1 {
                                player.clay -= num_rooms;
                                player.reed -= 1;
                                player.upgrade_house();
                            }
                        },
                        HouseType::Clay => {
                            if player.stone >= num_rooms && player.reed >= 1 {
                                player.stone -= num_rooms;
                                player.reed -= 1;
                                player.upgrade_house();
                            }
                        },
                        HouseType::Stone => {}
                    }

                    action_taken = format!("Renovation and MajorImprovement").to_string();
                },
                Some(AgricolaAction::Vegetable) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Vegetable).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. vegetable is already taken", player_index);
                    }
                    player.vegetables += 1;
                    action_taken = format!("Vegetable +1").to_string();
                },
                Some(AgricolaAction::Boar) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Boar).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. Boar is already taken", player_index);
                    }
                    player.boar += curr_tile.items;
                    action_taken = format!("Boar +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Cattle) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Cattle).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. Cattle is already taken", player_index);
                    }
                    player.cattle += curr_tile.items;
                    action_taken = format!("Cattle +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Stone_2) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Stone_2).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. second Stone is already taken", player_index);
                    }
                    player.stone += curr_tile.items;
                    action_taken = format!("(Second) Stone +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Plow_NoSow) |
                Some(AgricolaAction::Sow_NoPlow) |
                Some(AgricolaAction::Plow_Sow) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Plow_Sow).unwrap());
                    match agricola_action {
                        Some(AgricolaAction::Plow_NoSow) => {
                            player.plow();
                            player.fields += 1;
                            action_taken = format!("Plow but No Sow").to_string();
                        }
                        Some(AgricolaAction::Sow_NoPlow) => {
                            player.sow();
                            action_taken = format!("Sow but No Plow").to_string();
                        }
                        Some(AgricolaAction::Plow_Sow) => {
                            player.plow();
                            player.fields += 1;
                            player.sow();
                            action_taken = format!("Plow and Sow").to_string();
                        },
                        _ => panic!("[Plow_Sow] Can never reach here..")
                    }
                },
                Some(AgricolaAction::FamilyGrowth_NoSpace) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::FamilyGrowth_NoSpace).unwrap());
                    action_taken = format!("FamilyGrowth_NoSpace").to_string();
                },
                Some(AgricolaAction::Renovation_Fences) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Renovation_Fences).unwrap());
                    let num_rooms = player.player_mat.tiles.iter()
                                                           .filter(|t| t.house.is_some())
                                                           .count();


                    match player.house_type {
                        HouseType::Wood => {
                            if player.clay >= num_rooms && player.reed >= 1 {
                                player.clay -= num_rooms;
                                player.reed -= 1;
                                player.upgrade_house();
                            }
                        },
                        HouseType::Clay => {
                            if player.stone >= num_rooms && player.reed >= 1 {
                                player.stone -= num_rooms;
                                player.reed -= 1;
                                player.upgrade_house();
                            }
                        },
                        HouseType::Stone => {}
                    }
                    action_taken = format!("Renovation and Fences").to_string();
                },
                _ => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Plow).unwrap());
                    unimplemented!();
                },
            }

            // println!("Action: {:?} Curr_tile: {:?}", agricola_action, curr_tile);
            curr_tile.occupied = Some(self.current_player);

            player.actions -= 1;
            self.player_just_moved = self.current_player;
        }

        self.add_action(player_index, action_taken);

        /*
        * Since players can have different number of actions, we need to loop through
        * all players looking for the next player with actions.
        */ 
        let orig_player = self.current_player;
        for curr_player_index in player_index+1..player_index+1+num_players { 
            let player_index = curr_player_index % num_players;
            let curr_player = &self.players[player_index];
            if curr_player.actions > 0 {
                // Found a player with actions left
                self.current_player = player_index;
                break;
            }
        }

        // self.current_player didn't change, therefore no players have available actions
        if self.current_player == orig_player {
            self.end_round();
        }
    }

    fn get_result(&self, player: usize) -> f32 {
        let mut scores = Vec::new();
        for player in &self.players {
            scores.push(player.score());
        }

        // println!("P: {} {:?} [{}]", player, scores, scores.iter().max().unwrap());
        if scores[player] == *scores.iter().max().unwrap() {
            return 1.0
        } else {
            return 0.0
        }
    }
}


impl AgricolaState {
    pub fn new(num_players: usize) -> AgricolaState {
        let mut players = Vec::new();

        /// Player one gets 1 food while others get 2
        for i in 0..num_players {
            if i == 0 {
                players.push(Player::new(1));
            } else {
                players.push(Player::new(2));
            }
        }

        AgricolaState {
            players: players,
            player_just_moved: 0,
            current_player: 0,
            starting_player_token: None,
            board: Board::new(),
            rounds: 1,
            total_rounds: 14,
            actions_taken: Vec::new()
        }
    }

    fn end_round(&mut self) {
        // println!("Ending round");
        
        // Reset actions for all players
        for mut player in self.players.iter_mut() {
            player.actions = player.total_actions;
            // println!("After reset: {}", player);
        }

        // Set next player
        match self.starting_player_token {
            Some(player) => {
                // println!("Player {} chose starting player..", player);
                self.current_player = player;
            }
            None => {
                let num_players = self.players.len();
                self.current_player = (self.current_player + 1) % num_players;
                // println!("Player {} is the starting player in order", self.current_player);
            }
        }
        self.starting_player_token = None;

        // Reset the board
        self.board.reset();

        match self.rounds {
            4|7|9|11|13|14 => {
                // Field Phase
                for ref mut player in self.players.iter_mut() {
                    for ref mut curr_tile in player.player_mat.tiles.iter_mut() {
                        let mut empty = false;
                        if let Some(ref mut field) = curr_tile.field {
                            match field.crop {
                                Some(Crop::Grain) => {
                                    player.grains += 1;
                                },
                                Some(Crop::Vegetable) => {
                                    player.vegetables += 1;
                                },
                                None => { continue; },
                            }
                            field.count -= 1;
                            if field.count == 0 {
                                empty = true;
                            }
                        };
                        if empty {
                            curr_tile.field = None;
                        }
                    }
                }
                
                // Feeding Phase
                for mut player in self.players.iter_mut() {
                    if player.food >= (player.total_actions * 2) + (player.children) {
                        player.food -= player.total_actions * 2 + (player.children);
                    } else {
                        let mut food_needed = (player.total_actions * 2) + (player.children) - player.food;
                        player.food = 0;
                        loop {
                            if food_needed == 0 || player.grains == 0 {
                                break;
                            }
                            if player.grains > 0 {
                                player.grains -= 1;
                                food_needed -= 1;
                            }
                        }
                        player.beggers += food_needed;
                    }
                }

                // Breeding Phase
                for mut player in self.players.iter_mut() {
                    if player.sheep >= 2 {
                        player.sheep += 1;
                    }
                    if player.boar >= 2 {
                        player.boar += 1;
                    }
                    if player.cattle >= 2 {
                        player.cattle += 1;
                    }
                }
            },
            _ => {}
        }
        for mut player in self.players.iter_mut() {
            if player.children == 1 {
                player.total_actions += 1;
                player.children = 0;
            }
        }

        self.rounds += 1;
    }

    fn add_action(&mut self, player: usize, action: String) {
        let curr_player = &mut self.players[player];
        curr_player.actions_taken.push(format!("Round: {} [{}/{}] {}", self.rounds, curr_player.total_actions-curr_player.actions, curr_player.total_actions, action));
        self.actions_taken.push(format!("Round: {} Player: {} [{}/{}] {}", self.rounds, player, curr_player.total_actions-curr_player.actions, curr_player.total_actions, action));
    }

    pub fn print_ending(&self) {

        for (i, player) in self.players.iter().enumerate() {
            for action in &player.actions_taken {
                println!("{}", action);
            }
            println!("Player {}: {}\n{}", i, player.score(), player);
        }

        let mut scores = Vec::new();
        for player in &self.players {
            scores.push(player.score());
        }
    }
}

impl Display for AgricolaState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player Just Played: {}\n", self.player_just_moved + 1);
        for (i, player) in self.players.iter().enumerate() {
            write!(f, "P: {} {}\n", i+1, player);
        }
        write!(f, "{}", self.board);
        write!(f, "Next Player: {}\n", self.current_player + 1)
    }
}

#[derive(Debug, Clone)]
enum HouseType {
    Wood,
    Clay,
    Stone
}

impl Display for HouseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HouseType::Wood  => write!(f, "Wood "),
            &HouseType::Clay  => write!(f, "Clay "),
            &HouseType::Stone => write!(f, "Stone"),
        }
    }
}

#[derive(Debug, Clone)]
enum Animal {
    Sheep,
    Boar,
    Cattle
}

impl Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Animal::Sheep  => write!(f, "Sheep "),
            &Animal::Boar   => write!(f, "Boar  "),
            &Animal::Cattle => write!(f, "Cattle"),
        }
    }
}

#[derive(Debug, Clone)]
struct PlayerMat {
    tiles: Vec<FarmTile>
}

impl PlayerMat {
    pub fn new() -> PlayerMat {
        let mut player_mat = Vec::new();
        for i in 0..15 {
            let mut new_tile = FarmTile::new();
            match i {
                5|10 => {
                    new_tile.house = Some(HouseType::Wood);
                },
                _ => new_tile.house = None
            }

            match i {
                4|9|14 => {},
                _ => new_tile.surrounding_tiles.push(i+1)
            }
            match i {
                0|5|10 => {},
                _ => new_tile.surrounding_tiles.push(i-1)
            }
            match i {
                0..5 => {},
                _ => new_tile.surrounding_tiles.push(i-5)
            }
            match i {
                10..15 => {},
                _ => new_tile.surrounding_tiles.push(i+5)
            }

            player_mat.push(new_tile);
        }
        PlayerMat { tiles: player_mat }
    }

    /// Given a number, place a fence at that location for both tiles touching that fence location
    /// Returns: True, placed fence; False, already occupied
    pub fn place_fence(&mut self, x: usize) -> Option<usize> {
        let tile_index = x / 4;
        let position = x % 4;
        match position {
            0 => {
                {
                    let curr_tile = &self.tiles[tile_index];
                    if curr_tile.north_fence == true {
                        return None;
                    }
                }

                match tile_index {
                    0..5 => {
                        {
                            let curr_tile = &mut self.tiles[tile_index];
                            if curr_tile.house.is_none() && curr_tile.field.is_none() {
                                curr_tile.north_fence = true;
                            } else {
                                return None;
                            }
                        }
                    }, // Prevent underflow
                    _ => {
                        {
                            let curr_tile = self.tiles[tile_index].clone();
                            let next_tile = self.tiles[tile_index-5].clone();
                            // Only place a horizontal fence if one of the two adjacent tiles
                            // is not a house or field
                            if (curr_tile.house.is_none() && curr_tile.field.is_none()) ||
                               (next_tile.house.is_none() && next_tile.field.is_none()) {
                                {
                                    let curr_tile = &mut self.tiles[tile_index];
                                    curr_tile.north_fence = true;
                                }
                                {
                                    let next_tile = &mut self.tiles[tile_index-5];
                                    next_tile.south_fence = true;
                                }
                            } else {
                                return None;
                            }
                        }
                    }
                }
            },
            1 => {
                {
                    let curr_tile = &self.tiles[tile_index];
                    if curr_tile.west_fence == true {
                        return None;
                    }
                }
                match tile_index {
                    0|5|10 => {
                        let curr_tile = &mut self.tiles[tile_index];
                        if curr_tile.house.is_none() && curr_tile.field.is_none() {
                            curr_tile.west_fence = true;
                        } else {
                            return None;
                        }
                    }, // Prevent underflow
                    _ => {
                        {
                            let curr_tile = self.tiles[tile_index].clone();
                            let next_tile = self.tiles[tile_index-1].clone();
                            // Only place a vertical fence if one of the two adjacent tiles
                            // is not a house or field
                            if (curr_tile.house.is_none() && curr_tile.field.is_none()) ||
                                (next_tile.house.is_none() && next_tile.field.is_none()) {
                                {
                                    let curr_tile = &mut self.tiles[tile_index];
                                    curr_tile.west_fence = true;
                                }
                                {
                                    let next_tile = &mut self.tiles[tile_index-1];
                                    next_tile.east_fence = true;
                                }
                            } else {
                                return None;
                            }
                        }
                    }
                }
            },
            2 => {
                {
                    let curr_tile = &self.tiles[tile_index];
                    if curr_tile.south_fence == true {
                        return None;
                    }
                }
                match tile_index {
                    10..15 => { // Prevent out of bounds access
                        let curr_tile = &mut self.tiles[tile_index];
                        if curr_tile.house.is_none() && curr_tile.field.is_none() {
                            curr_tile.south_fence = true;
                        } else {
                            return None;
                        }
                    }, 
                    _ => {
                        {
                            let curr_tile = self.tiles[tile_index].clone();
                            let next_tile = self.tiles[tile_index+5].clone();
                            // Only place a vertical fence if one of the two adjacent tiles
                            // is not a house or field
                            if (curr_tile.house.is_none() && curr_tile.field.is_none()) ||
                                (next_tile.house.is_none() && next_tile.field.is_none()) {
                                {
                                    let curr_tile = &mut self.tiles[tile_index];
                                    curr_tile.south_fence = true;
                                }
                                {
                                    let next_tile = &mut self.tiles[tile_index+5];
                                    next_tile.north_fence = true;
                                }
                            } else {
                                return None;
                            }
                        }
                    }
                }
            },
            3 => {
                {
                    let curr_tile = &self.tiles[tile_index];
                    if curr_tile.east_fence == true {
                        return None;
                    }
                }
                match tile_index {
                    4|9|14 => {
                        let curr_tile = &mut self.tiles[tile_index];
                        if curr_tile.house.is_none() && curr_tile.field.is_none() {
                            curr_tile.east_fence = true;
                        } else {
                            return None;
                        }
                    }, // Prevent out of bounds access
                    _ => {
                        {
                                let curr_tile = self.tiles[tile_index].clone();
                                let next_tile = self.tiles[tile_index+1].clone();
                            // Only place a vertical fence if one of the two adjacent tiles
                            // is not a house or field
                            if (curr_tile.house.is_none() && curr_tile.field.is_none()) ||
                                (next_tile.house.is_none() && next_tile.field.is_none()) {
                                {
                                    let curr_tile = &mut self.tiles[tile_index];
                                    curr_tile.east_fence = true;
                                }
                                {
                                    let next_tile = &mut self.tiles[tile_index+1];
                                    next_tile.west_fence = true;
                                }
                            } else {
                                return None;
                            }
                        }
                    }
                }
            },
            _ => panic!("Should never reach here!")
        };

        /*
        let string_index = match position {
            0 => format!("{}-{}N", x, tile_index),
            1 => format!("{}-{}W", x, tile_index),
            2 => format!("{}-{}S", x, tile_index),
            3 => format!("{}-{}E", x, tile_index),
            _ => panic!("No other valid fence position")
        };
        */

        return Some(tile_index);
    }

    fn plow_random_field(&mut self) {
        // Get vector of indexes of current fields in the player mat
        let curr_fields: Vec<usize> = self.tiles
                                          .iter()
                                          .enumerate()
                                          .filter(|&(i, t)| !(t.field.is_none()))
                                          .map(|(i, t)| i)
                                          .collect();

        // Filter surrounding tiles if they are empty
        let possible_fields: Vec<usize> = curr_fields.iter()
                                                     .flat_map(|&i| self.tiles[i].surrounding_tiles.clone())
                                                     .filter(|&i| self.tiles[i].is_empty())
                                                     .collect();

        if possible_fields.len() == 0 {
            return;
        }
        let random_field = rand::thread_rng().choose(&possible_fields).unwrap();
        self.tiles[*random_field].plow();
    }

    fn current_fences(&self) -> Vec<(usize, &str)> {
        let mut current_fences = Vec::new(); 
        // for (i, tile) in player.player_mat.tiles.iter().enumerate() {
        for (i, tile) in self.tiles.iter().enumerate() {
            if tile.north_fence {
                current_fences.push((i, "north"));
            }
            if tile.west_fence {
                current_fences.push((i, "west"));
            }
            if tile.south_fence {
                current_fences.push((i, "south"));
            }
            if tile.east_fence {
                current_fences.push((i, "east"));
            }
        }
        current_fences
    }


    fn valid_fences(&self) -> bool {
        let current_fences = self.current_fences();

        for curr_fence in current_fences.iter() {
            if let Some(&(ref endpoint_1, ref endpoint_2)) = FENCE_MAP.get(&curr_fence) {
                let mut found_fences = Vec::new();
                for fence in endpoint_1 {
                    if current_fences.contains(&fence) {
                        found_fences.push(fence);
                    }
                }

                for fence in endpoint_2 {
                    if current_fences.contains(&fence) {
                        found_fences.push(fence);
                    }
                }
                if found_fences.len() == 2 {
                    return true;
                }
            }
        }
        false
    }
}

impl Display for PlayerMat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..3 {
            // Top line
            let mut line = String::from("+");
            for i in (row*5)..(row*5)+5 {
                let curr_tile = &self.tiles[i];
                match curr_tile.north_fence {
                    true  => line = format!("{} --------- +", line),
                    false => line = format!("{}           +", line),
                };
            }
            write!(f, "{}\n", line);

            // Row - Index
            match self.tiles[row*5].west_fence {
                true  => line = String::from("|"),
                false => line = String::from(" "),
            };
            for i in (row*5)..(row*5)+5 {
                let curr_tile = &self.tiles[i];
                line = format!("{}{number:>width$}        ", line, number=i, width=2);
                match curr_tile.east_fence {
                    true  => line = format!("{} |", line),
                    false => line = format!("{}  ", line),
                }
            }

            write!(f, "{}\n", line);
            

            // Row - Type of Tile (House or Field)
            match self.tiles[row*5].west_fence {
                true  => line = String::from("| "),
                false => line = String::from("  "),
            };
            for i in (row*5)..(row*5)+5 {
                let curr_tile = &self.tiles[i];
                match (&curr_tile.house, &curr_tile.field, curr_tile.stable) {
                    (&Some(ref house), &None, false) => line = format!("{} {}    ", line, house),
                    (&None, &Some(ref field), false) => line = format!("{} Field    ", line),
                    (&None, &None, true) => line = format!("{} Stable   ", line),
                    (&None, &None, false)            => line = format!("{}          ", line),
                    _ => panic!("Tile has multiple types!")
                };
                match curr_tile.east_fence {
                    true  => line = format!("{}| ", line),
                    false => line = format!("{}  ", line),
                }
            }

            write!(f, "{}\n", line);

            // Row - Count of resources on tile
            match self.tiles[row*5].west_fence {
                true  => line = String::from("| "),
                false => line = String::from("  "),
            };
            for i in (row*5)..(row*5)+5 {
                let curr_tile = &self.tiles[i];
                match (&curr_tile.animal_type, &curr_tile.field) {
                    (&Some(ref animal), &None) => line = format!("{} {}: {}  ", line, animal, curr_tile.animal_count),
                    (&None, &Some(ref field))  => {
                        match field.count() {
                            0 => line = format!("{}          ", line),
                            _ => line = format!("{} {}: {} ", line, field.crop(), field.count()),
                        }
                    }
                    (&None, &None)             => line = format!("{}          ", line),
                    _ => panic!("Tile has multiple types!")
                };
                match curr_tile.east_fence {
                    true  => line = format!("{}| ", line),
                    false => line = format!("{}  ", line),
                }
            }

            write!(f, "{}\n", line);

            /*
            // Row - Count of resources on tile
            match self.tiles[row*5].west_fence {
                true  => line = String::from("| "),
                false => line = String::from("  "),
            };
            for i in (row*5)..(row*5)+5 {
                let curr_tile = &self.tiles[i];
                match curr_tile.stable {
                    true  => line = format!("{} Stable   ", line),
                    false => line = format!("{}          ", line),
                };
                match curr_tile.east_fence {
                    true  => line = format!("{}| ", line),
                    false => line = format!("{}  ", line),
                }
            }

            write!(f, "{}\n", line);
            */
        }
        // Top line
        let mut line = String::from("+");
        for i in 10..15 {
            let curr_tile = &self.tiles[i];
            match curr_tile.south_fence {
                true  => line = format!("{} --------- +", line),
                false => line = format!("{}           +", line),
            };
        }
        write!(f, "{}\n", line)

        /*
        for (i, tile) in self.tiles.iter().enumerate() {
            write!(f, "N: {} W: {} S: {} E: {}|", tile.north_fence, tile.west_fence, tile.south_fence, tile.east_fence);
            match i {
                4|9 => write!(f, "\n"),
                _ => write!(f, "")
            };
        }
        write!(f, "\n")
        */
    }
}

#[derive(Debug, Clone)]
struct FarmTile {
    house: Option<HouseType>,
    stable: bool,
    animal_type: Option<Animal>,
    animal_count: usize,
    north_fence: bool,
    south_fence: bool,
    east_fence: bool,
    west_fence: bool,
    field: Option<FieldTile>,
    surrounding_tiles: Vec<usize>
}

impl FarmTile {
    fn new() -> FarmTile {
        FarmTile {
            house: None,
            stable: false,
            animal_type: None,
            animal_count: 0,
            north_fence: false,
            south_fence: false,
            east_fence: false,
            west_fence: false,
            field: None,
            surrounding_tiles: Vec::new()
        }
    }

    fn is_empty(&self) -> bool {
        self.house.is_none() && !self.stable && self.field.is_none()
    }

    fn plow(&mut self) {
        self.field = Some(FieldTile::new());
    }

    fn build_room(&mut self, house_type: HouseType) {
        self.house = Some(house_type);
    }

    fn stable(&mut self) {
        self.stable = true;
    }

    fn sow_veg(&mut self) {
        if let Some(ref mut field) = self.field {
            field.crop = Some(Crop::Vegetable);
            field.count = 2;
        }
    }

    fn sow_grain(&mut self) {
        if let Some(ref mut field) = self.field {
            field.crop = Some(Crop::Grain);
            field.count = 3;
        }
    }

    fn upgrade(&mut self) {
        match self.house {
            Some(HouseType::Wood) => self.house = Some(HouseType::Clay),
            Some(HouseType::Clay) => self.house = Some(HouseType::Stone),
            _ => {},
        }
    }
}

#[derive(Debug, Clone)]
struct FieldTile {
    crop: Option<Crop>,
    count: usize
}

impl FieldTile {
    fn new() -> FieldTile {
        FieldTile {
            crop: None,
            count: 0
        }
    }

    fn new_with_crop(crop: Crop, count: usize) -> FieldTile {
        FieldTile {
            crop: Some(crop),
            count: count
        }
    }

    fn crop(&self) -> String {
        match self.crop {
            Some(Crop::Grain)     => String::from("Grain"),
            Some(Crop::Vegetable) => String::from("Veg  "),
            _                     => String::from("     ")
        }
    }

    fn count(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        match self.crop {
            None => false,
            Some(ref field) => true
        }
    }
}

#[derive(Debug, Clone)]
enum Crop {
    Grain,
    Vegetable
}
