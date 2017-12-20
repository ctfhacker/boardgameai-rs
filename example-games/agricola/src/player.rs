use super::*;
use rand::{Rng, thread_rng};
use std::fmt;
use std::fmt::Display;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Player {
    pub food: usize,
    pub fields: usize,
    pub grains: usize,
    pub vegetables: usize,
    pub wood: usize,
    pub clay: usize,
    pub reed: usize,
    pub stone: usize,
    pub sheep: usize,
    pub cattle: usize,
    pub boar: usize,
    pub actions: usize,
    pub total_actions: usize,
    pub actions_taken: Vec<String>,
    pub player_mat: PlayerMat,
    pub house_type: HouseType,
    pub beggers: usize,
    pub children: usize,
    pub stables: usize,
    pub fences: usize,
    pub pastures: Vec<Pasture>
}

impl Player {
    pub fn new(food: usize) -> Player {
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
            fences: 0,
            pastures: Vec::new(),
            stables: 0
        }
    }

    pub fn score(&self) -> i32 {
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
        result += self.pastures.len() as i32;
        for pasture in &self.pastures {
            result += pasture.stables as i32;
        }
        result += (self.total_actions * 3) as i32;
        result
    }

    /// Randomly plow a field if none exists. If a field already exists, plow a random field
    /// connected to an existing field
    pub fn plow(&mut self) {
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

    pub fn can_build_room(&self) -> bool {
        if self.reed < 2 { return false; }
        match self.house_type {
            HouseType::Wood => if self.wood < 5 { return false; },
            HouseType::Clay => if self.clay < 5 { return false; },
            HouseType::Stone => if self.stone < 5 { return false; },
        }
        return true;
    }

    pub fn build_room(&mut self) {
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

        let random_room = ::rand::thread_rng().choose(&possible_rooms).unwrap();
        self.player_mat.tiles[*random_room].build_room(self.house_type.clone());
    }

    pub fn build_stables(&mut self) {
        let available_stables = 4 - self.stables;
        let max_stables = ::std::cmp::min(available_stables, self.wood / 2);
        let num_stables = ::rand::random::<usize>() % (max_stables+1);
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

            let random_tile = ::rand::thread_rng().choose(&possibles).unwrap();
            self.player_mat.tiles[*random_tile].stable();
            self.wood -= 2;
            self.stables += 1;
        }
    }

    pub fn build_stable(&mut self) {
        if self.wood == 0 || self.stables == 4{
            // Not enough wood to buy one stable or no available stables
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

        let random_tile = ::rand::thread_rng().choose(&possibles).unwrap();
        self.player_mat.tiles[*random_tile].stable();
        self.wood -= 1;
        self.stables += 1;
    }

    pub fn sow(&mut self) {
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

    pub fn sow_veg(&mut self, index: usize) {
        self.player_mat.tiles[index].sow_veg();
    }

    pub fn sow_grain(&mut self, index: usize) {
        self.player_mat.tiles[index].sow_grain();
    }

    pub fn upgrade_house(&mut self) {
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

    pub fn make_pastures(&mut self) -> usize {
        let mut curr_pasture = HashSet::new();
        let mut fences_built = 0;

        {
            let empty_spaces: Vec<&FarmTile> = self.player_mat.tiles.iter()
                                                                    .filter(|&t| t.can_be_fenced())
                                                                    .collect(); 
                

            if let Some(temp_space) = ::rand::thread_rng().choose(&empty_spaces) {
                let mut curr_space = *temp_space;
                curr_pasture.insert(curr_space.index);
                loop {
                    if ::rand::random::<usize>() % 100 < 30 {
                        break;
                    }
                    
                    let surrounding_tiles: Vec<&usize> = curr_space.surrounding_tiles.iter()
                                                                                    .filter(|&t| self.player_mat.tiles[*t].can_be_fenced())
                                                                                    .collect();

                    if let Some(surrounding_tile) = ::rand::thread_rng().choose(&surrounding_tiles) {
                        curr_space = &self.player_mat.tiles[**surrounding_tile];
                        curr_pasture.insert(**surrounding_tile);
                    } else {
                        break;
                    }
                }
            } else {
                println!("[Make Pastures] No more empty spaces.. cannot make pasture");
            }
        }

        if curr_pasture.len() > 0 {
            if let Some(wood_used) = self.player_mat.make_pasture(curr_pasture.clone().into_iter().collect(), self.wood) {
                let stables = curr_pasture.iter().filter(|&&t| self.player_mat.tiles[t].stable == true).sum();
                self.wood -= wood_used;
                self.pastures.push(
                    Pasture::new(curr_pasture.into_iter().collect(), stables)
                );
                fences_built += wood_used;
            }
        }
        fences_built
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

/*
#[derive(Debug, Clone, Copy)]
struct PlayerId {
    index: usize
}
*/
