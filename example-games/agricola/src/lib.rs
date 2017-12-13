#![feature(exclusive_range_pattern)]
extern crate boardgameai_rs;
extern crate rand;

use boardgameai_rs::*;
use boardgameai_rs::state::State;
use boardgameai_rs::action::Action;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use rand::Rng;

#[derive(Debug, Clone)]
struct Player {
    food: usize,
    fields: usize,
    grain: usize,
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
    player_mat: PlayerMat
}

impl Player {
    fn new(food: usize) -> Player {
        Player {
            food: food,
            fields: 0,
            grain: 0,
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
            player_mat: PlayerMat::new()
        }
    }

    fn score(&self) -> i32 {
        let mut result: i32 = 0;
        match self.fields {
            0|1 => result -= 1,
            2|3 => result += 1,
            4|5 => result += 2,
            6   => result += 3,
            _   => result += 4
        }
        match self.grain {
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
        // TODO Spaces in farmyard
        // TODO fenced in stables
        // TODO Room types
        result += (self.total_actions * 3) as i32;
        result
    }

    /// Randomly plow a field if none exists. If a field already exists, plow a random field
    /// connected to an existing field
    fn plow(&mut self) {
        if self.fields == 0 {
            loop {
                let num = rand::thread_rng().gen_range(0, 15);
                if self.player_mat.tiles[num].is_empty() {
                    self.player_mat.tiles[num].plow();
                    break;
                }
            }
        } else {
            self.player_mat.plow_random_field();
        }
    }

}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Food: {} Grain: {} Wood: {} Clay: {} Reed: {} Stone: {} Actions: {}/{} Fields: {}]\n{}", 
                    self.food, self.grain, self.wood, self.clay, self.reed, 
                    self.stone, self.actions, self.total_actions, self.fields, self.player_mat)
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
    tiles: HashMap<AgricolaTile, Box<BoardTile>>
}

impl Board {
    fn new() -> Board {
        let mut board = HashMap::new();
        board.insert(AgricolaTile::BuildRoom_BuildStable, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::StartingPlayer_Food, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1}));
        board.insert(AgricolaTile::Grain, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::Plow, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::BuildStable_BakeBread, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::DayLaborer, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::Sow_BakeBread, Box::new(BoardTile { occupied: None, items: 0, reset_amount: 0}));
        board.insert(AgricolaTile::Wood, Box::new(BoardTile { occupied: None, items: 3, reset_amount: 3}));
        board.insert(AgricolaTile::Clay, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1}));
        board.insert(AgricolaTile::Reed, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1}));
        board.insert(AgricolaTile::Fishing, Box::new(BoardTile { occupied: None, items: 1, reset_amount: 1}));
        Board {
            tiles: board,
        }
    }

    fn reset(&mut self) {
        for (name, mut tile) in &mut self.tiles {
            // println!("{:?}: {:?} -> {:?}", name, tile.items, tile.items+tile.reset_amount);
            tile.items += tile.reset_amount;
            tile.occupied = None;
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
    BuildRoom_BuildStable = 1,
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
}

#[derive(Debug)]
pub enum AgricolaAction {
    BuildRoom_BuildStable = 1,
    StartingPlayer_Food = 2,
    Grain = 3,
    Plow = 4,
    BuildStable_BakeBread = 5,
    DayLaborer_Food_Wood = 6,
    DayLaborer_Food_Clay = 7,
    DayLaborer_Food_Reed = 8,
    DayLaborer_Food_Stone = 9,
    Sow_BakeBread = 10,
    Wood = 11,
    Clay = 12,
    Reed = 13,
    Fishing = 14,
}

impl AgricolaAction {
    fn from_u32(x: u32) -> Option<AgricolaAction> {
        match x {
            1 => Some(AgricolaAction::BuildRoom_BuildStable),
            2 => Some(AgricolaAction::StartingPlayer_Food),
            3 => Some(AgricolaAction::Grain),
            4 => Some(AgricolaAction::Plow),
            5 => Some(AgricolaAction::BuildStable_BakeBread),
            6 => Some(AgricolaAction::DayLaborer_Food_Wood),
            7 => Some(AgricolaAction::DayLaborer_Food_Clay),
            8 => Some(AgricolaAction::DayLaborer_Food_Reed),
            9 => Some(AgricolaAction::DayLaborer_Food_Stone),
            10 => Some(AgricolaAction::Sow_BakeBread),
            11 => Some(AgricolaAction::Wood),
            12 => Some(AgricolaAction::Clay),
            13 => Some(AgricolaAction::Reed),
            14 => Some(AgricolaAction::Fishing),
            _ => None
        }
    }
}


#[derive(Debug, Clone)]
pub struct AgricolaState {
    players: Vec<Player>,
    player_just_moved: usize,
    current_player: usize,
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
                    &AgricolaTile::BuildRoom_BuildStable => actions.push(AgricolaAction::BuildRoom_BuildStable as u32),
                    &AgricolaTile::StartingPlayer_Food => actions.push(AgricolaAction::StartingPlayer_Food as u32),
                    &AgricolaTile::Grain => actions.push(AgricolaAction::Grain as u32),
                    &AgricolaTile::Plow  => actions.push(AgricolaAction::Plow as u32),
                    &AgricolaTile::BuildStable_BakeBread  => actions.push(AgricolaAction::BuildStable_BakeBread as u32),
                    &AgricolaTile::Sow_BakeBread  => actions.push(AgricolaAction::Sow_BakeBread as u32),
                    &AgricolaTile::Wood  => actions.push(AgricolaAction::Wood as u32),
                    &AgricolaTile::Clay  => actions.push(AgricolaAction::Clay as u32),
                    &AgricolaTile::Reed  => actions.push(AgricolaAction::Reed as u32),
                    &AgricolaTile::Fishing  => actions.push(AgricolaAction::Fishing as u32),
                }
            }
        }
        // println!("Available actions: {:?}", actions);

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
        // println!("[R:{} P:{}] Action: {:?}", self.rounds, self.current_player, AgricolaAction::from_u32(action));
        let player_index = self.current_player;
        let num_players = self.players.len();
        let mut action_taken = String::from("");
        {
            let mut player = &mut self.players[player_index];
            let agricola_action = AgricolaAction::from_u32(action);
            let mut curr_tile;
            match agricola_action {
                Some(AgricolaAction::Grain) => {
                    // println!("In Grain");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Grain).unwrap());
                    /*
                    if curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. Grain is already taken", player_index);
                    }
                    */
                    player.grain += 1;
                    action_taken = String::from("Grain +1");
                },
                Some(AgricolaAction::Wood) => {
                    // println!("In Wood");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Wood).unwrap());
                    // println!("Wood tile: {}", curr_tile.items);
                    /*
                    if curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. Wood is already taken", player_index);
                    }
                    */
                    player.wood += curr_tile.items;
                    action_taken = format!("Wood +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Clay) => {
                    // println!("In Clay");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Clay).unwrap());
                    // println!("clay tile: {}", curr_tile.items);
                    /*
                    if curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. clay is already taken", player_index);
                    }
                    */
                    player.clay += curr_tile.items;
                    action_taken = format!("Clay +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Reed) => {
                    // println!("In Reed");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Reed).unwrap());
                    // println!("reed tile: {}", curr_tile.items);
                    /*
                    if reed_tile.occupied.is_none() {
                        panic!("Player {} is bad.. reed is already taken", player_index);
                    }
                    */
                    player.reed += curr_tile.items;
                    action_taken = format!("Reed +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Fishing) => {
                    // println!("In Fishing");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Fishing).unwrap());
                    // println!("fishing tile: {}", curr_tile.items);
                    /*
                    if curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. fishing is already taken", player_index);
                    }
                    */
                    player.food += curr_tile.items;
                    action_taken = format!("Food (Fishing) +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::DayLaborer_Food_Wood) |
                Some(AgricolaAction::DayLaborer_Food_Clay) |
                Some(AgricolaAction::DayLaborer_Food_Reed) |
                Some(AgricolaAction::DayLaborer_Food_Stone) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::DayLaborer).unwrap());
                    // println!("day_laborer tile: {}", curr_tile.items);
                    /*
                    if curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. day_laborer is already taken", player_index);
                    }
                    */
                    player.food += 1;
                    match agricola_action {
                        Some(AgricolaAction::DayLaborer_Food_Wood) => {
                            // println!("In DayLaborer: Food + Wood");
                            action_taken = format!("Day Laborer Food +1 Wood +1").to_string();
                            player.wood += 1;
                        },
                        Some(AgricolaAction::DayLaborer_Food_Clay) => {
                            // println!("In DayLaborer: Food + Clay");
                            action_taken = format!("Day Laborer Food +1 Clay +1").to_string();
                            player.clay += 1;
                        },
                        Some(AgricolaAction::DayLaborer_Food_Reed) => {
                            // println!("In DayLaborer: Food + Reed");
                            action_taken = format!("Day Laborer Food +1 Reed +1").to_string();
                            player.reed += 1;
                        },
                        Some(AgricolaAction::DayLaborer_Food_Stone) => {
                            // println!("In DayLaborer: Food + Stone");
                            action_taken = format!("Day Laborer Food +1 Stone +1").to_string();
                            player.stone += 1;
                        },
                        _ => panic!("Should never get here.. Day Laborer only has 4 choices..")
                    }
                },
                Some(AgricolaAction::Sow_BakeBread) => {
                    // println!("In Sow + BakeBread.. doing nothing");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Sow_BakeBread).unwrap());
                    action_taken = format!("Sow and Bake Bread").to_string();
                },
                Some(AgricolaAction::BuildRoom_BuildStable) => {
                    // println!("In Build Room  + Build Stable.. doing nothing");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::BuildRoom_BuildStable).unwrap());
                    action_taken = format!("Build Room and Build Stable").to_string();
                },
                Some(AgricolaAction::StartingPlayer_Food) => {
                    // println!("In Starting Player + Food");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::StartingPlayer_Food).unwrap());
                    /*
                    if curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. reed is already taken", player_index);
                    }
                    */
                    player.food += 1;
                    action_taken = format!("Starting Player and Food +1").to_string();
                    self.starting_player_token = Some(self.current_player);
                },
                Some(AgricolaAction::Plow) => {
                    // println!("In Plow.. doing nothing");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Plow).unwrap());
                    action_taken = format!("Plow").to_string();
                    player.plow();
                    player.fields += 1;
                },
                Some(AgricolaAction::BuildStable_BakeBread) => {
                    // println!("In Build Stable + Bake Bread.. doing nothing");
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::BuildStable_BakeBread).unwrap());
                    action_taken = format!("Build Stable and Bake Bread").to_string();
                },
                _ => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Plow).unwrap());
                    unimplemented!();
                }
            }
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
            total_rounds: 6,
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

        self.rounds += 1;
    }

    fn add_action(&mut self, player: usize, action: String) {
        let curr_player = &mut self.players[player];
        curr_player.actions_taken.push(format!("Round: {} [{}/{}] {}", self.rounds, curr_player.total_actions-curr_player.actions, curr_player.total_actions, action));
        self.actions_taken.push(format!("Round: {} Player: {} [{}/{}] {}", self.rounds, player, curr_player.total_actions-curr_player.actions, curr_player.total_actions, action));
    }

    pub fn print_ending(&self) {
        for (i, player) in self.players.iter().enumerate() {
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
    pub fn place_fence(&mut self, x: usize) {
        let tile_index = x / 4;
        let position = x % 4;
        match position {
            0 => {
                {
                    let curr_tile = &mut self.tiles[tile_index];
                    curr_tile.north_fence = true;
                }
                match tile_index {
                    0..5 => {}, // Prevent underflow
                    _ => {
                        let next_tile = &mut self.tiles[tile_index-5];
                        next_tile.south_fence = true;
                    }
                }
            },
            1 => {
                {
                    let curr_tile = &mut self.tiles[tile_index];
                    curr_tile.west_fence  = true;
                }
                match tile_index {
                    0|5|10 => {}, // Prevent underflow
                    _ => {
                        let next_tile = &mut self.tiles[tile_index-1];
                        next_tile.east_fence = true;
                    }
                }
            },
            2 => {
                {
                    let curr_tile = &mut self.tiles[tile_index];
                    curr_tile.south_fence = true;
                }
                match tile_index {
                    10..15 => {}, // Prevent out of bounds access
                    _ => {
                        let next_tile = &mut self.tiles[tile_index+5];
                        next_tile.north_fence = true;
                    }
                }
            },
            3 => {
                {
                    let curr_tile = &mut self.tiles[tile_index];
                    curr_tile.east_fence  = true;
                }
                match tile_index {
                    4|9|14 => {}, // Prevent out of bounds access
                    _ => {
                        let next_tile = &mut self.tiles[tile_index+1];
                        next_tile.west_fence = true;
                    }
                }
            },
            _ => panic!("Should never reach here!")
        };

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

        let random_field = rand::thread_rng().choose(&possible_fields).unwrap();
        self.tiles[*random_field].plow();
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
                    true  => line = format!("{}| ", line),
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
                match (&curr_tile.house, &curr_tile.field) {
                    (&Some(ref house), &None) => line = format!("{} {}    ", line, house),
                    (&None, &Some(ref field)) => line = format!("{} Field    ", line),
                    (&None, &None)            => line = format!("{}          ", line),
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

}

#[derive(Debug, Clone)]
enum Crop {
    Grain,
    Vegetable
}
