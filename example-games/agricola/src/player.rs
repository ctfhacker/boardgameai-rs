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
    pub pastures: Vec<Pasture>,
    pub pet: Option<Animal>,
    pub improvements: Vec<MajorImprovement>
}

impl Player {
    pub fn new(food: usize) -> Player {
        
        let mut new_player = Player {
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
            stables: 0,
            pet: None,
            improvements: Vec::new()
        };
        let new_player_display = format!("{}", new_player);
        new_player.actions_taken.push(format!("Round: 0 [2/2] Init\n{}", new_player_display));
        new_player
    }

    pub fn score(&self, verbose: bool) -> i32 {
        let mut result: i32 = 0;
        let mut score = -1;
        match self.fields {
            0|1 => score = -1,
            2   => score = 1,
            3   => score = 2,
            4   => score = 3,
            _   => score = 4
        }
        result += score;
        if verbose { println!("Fields: {} -> {} pts", self.fields, score); }

        result += (self.pastures.len() * 1) as i32;
        if verbose { println!("Pastures: {} -> {} pts", self.pastures.len(), self.pastures.len()); }

        let grain_in_fields: usize = self.player_mat.tiles.iter()
                                                          .filter_map(|t| t.field.clone())
                                                          .filter(|t| t.is_grain())
                                                          .map(|t| t.count)
                                                          .sum();

        match (self.grains + grain_in_fields) {
            0     => score = -1,
            1|2|3 => score = 1,
            4|5   => score = 2,
            6|7   => score = 3,
            _     => score = 4,
        };
        result += score;
        if verbose { println!("Grains: {} (in hand) + {} (in fields) -> {} pts", self.grains, grain_in_fields, score); }

        let veg_in_fields: usize = self.player_mat.tiles.iter()
                                                        .filter_map(|t| t.field.clone())
                                                        .filter(|t| t.is_vegetable())
                                                        .map(|t| t.count)
                                                        .sum();

        match (self.vegetables + veg_in_fields) {
            0 => score = -1,
            1 => score = 1,
            2 => score = 2,
            3 => score = 3,
            _ => score = 4,
        };
        result += score;
        if verbose { println!("Vegetables: {} (in hand) + {} (in fields) -> {} pts", self.vegetables, veg_in_fields, score); }

        match self.sheep {
            0     => score = -1,
            1|2|3 => score = 1,
            4|5   => score = 2,
            6|7   => score = 3,
            _     => score = 4,
        };
        result += score;
        if verbose { println!("Sheep: {} -> {} pts", self.sheep, score); }

        match self.boar {
            0   => score = -1,
            1|2 => score = 1,
            3|4 => score = 2,
            5|6 => score = 3,
            _   => score = 4,
        };
        result += score;
        if verbose { println!("Boar: {} -> {} pts", self.boar, score); }

        match self.cattle {
            0   => score = -1,
            1   => score = 1,
            2|3 => score = 2,
            4|5 => score = 3,
            _   => score = 4,
        };
        if verbose { println!("Cattle: {} -> {} pts", self.cattle, score); }

        let empty_spaces: Vec<&FarmTile> = self.player_mat.tiles.iter()
                                                                .filter(|&t| t.is_empty())
                                                                .collect(); 
        if verbose { println!("Empty Spaces: {} -> -{} pts", empty_spaces.len(), empty_spaces.len()); }

        result -= empty_spaces.len() as i32;

        let num_rooms = self.player_mat.tiles.iter()
                                             .filter(|t| t.house.is_some())
                                             .count();
        
        match self.house_type {
            HouseType::Wood => score = 0 as i32,
            HouseType::Clay => score = (num_rooms * 1) as i32,
            HouseType::Stone => score = (num_rooms * 2) as i32,
        }
        result += score;
        if verbose { println!("Rooms [{:?}]: {} -> {} pts", self.house_type, num_rooms, score); }

        result -= (self.beggers * 3) as i32;
        if verbose { println!("Beggers: {} -> -{} pts", self.beggers, 3 * self.beggers); }

        let mut fenced_stables = 0;
        for pasture in &self.pastures {
            result += pasture.stables as i32;
            fenced_stables += pasture.stables as i32;
        }

        if verbose { println!("Fenced stables: {} pts", fenced_stables); }
        result += (self.total_actions * 3) as i32;

        if verbose { println!("Total actions: {} -> {} pts", self.total_actions, self.total_actions*3); }

        if self.improvements.contains(&MajorImprovement::Fireplace_2) { 
            result += 1;
            if verbose { println!("Fireplace_2 -> 1 pt") }
        };
        if self.improvements.contains(&MajorImprovement::Fireplace_3) { 
            result += 1;
            if verbose { println!("Fireplace_3 -> 1 pt") }
        };
        if self.improvements.contains(&MajorImprovement::CookingHearth_4) { 
            result += 1;
            if verbose { println!("CookingHearth_4 -> 1 pt") }
        };
        if self.improvements.contains(&MajorImprovement::CookingHearth_5) { 
            result += 1;
            if verbose { println!("CookingHearth_5 -> 1 pt") }
        };
        if self.improvements.contains(&MajorImprovement::ClayOven) { 
            result += 2;
            if verbose { println!("ClayOven -> 2 pt") }
        };
        if self.improvements.contains(&MajorImprovement::StoneOven) { 
            result += 3;
            if verbose { println!("StoneOven -> 3 pt") }
        };
        if self.improvements.contains(&MajorImprovement::Pottery) { 
            result += 2;
            match self.clay {
                0|1|2 => { if verbose { println!("Pottery -> 2 pt + 0 pt") } },
                3|4 => { result += 1; if verbose { println!("Pottery -> 2 pt + 1 pt (3-4 clay)") } },
                5|6 => { result += 2; if verbose { println!("Pottery -> 2 pt + 2 pt (5-6 clay)") } },
                _   => { result += 3; if verbose { println!("Pottery -> 2 pt + 3 pt (7+ clay)") } },
            }
        };
        if self.improvements.contains(&MajorImprovement::Joinery) { 
            result += 2;
            match self.wood {
                0|1|2 => { if verbose { println!("Joinery -> 2 pt + 0 pt") } },
                3|4 => { result += 1; if verbose { println!("Joinery -> 2 pt + 1 pt (3-4 wood)") } },
                5|6 => { result += 2; if verbose { println!("Joinery -> 2 pt + 2 pt (5-6 wood)") } },
                _   => { result += 3; if verbose { println!("Joinery -> 2 pt + 3 pt (7+ wood)") } },
            }
        };
        if self.improvements.contains(&MajorImprovement::BasketmakersWorkshop) { 
            result += 2;
            match self.reed {
                0|1 => { if verbose { println!("Basketmaker's Workshop -> 2 pt + 0 pt") } },
                2|3 => { result += 1; if verbose { println!("Basketmaker's Workshop -> 2 pt + 1 pt (2-3 reed)") } },
                4   => { result += 2; if verbose { println!("Basketmaker's Workshop -> 2 pt + 2 pt (4 reed)") } },
                _   => { result += 3; if verbose { println!("Basketmaker's Workshop -> 2 pt + 3 pt (5+ reed)") } },
            }
        };
        if self.improvements.contains(&MajorImprovement::Well) { 
            result += 4;
            if verbose { println!("Well -> 4 pt") }
        };

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
            HouseType::Wood => {
                if self.wood < 5 { return false; }
            }
            HouseType::Clay => {
                if self.clay < 5 { return false; }
            }
            HouseType::Stone => {
                if self.stone < 5 { return false; }
            }
        }
        return true;
    }

    pub fn pay_for_room(&mut self) {
        match self.house_type {
            HouseType::Wood =>  { self.wood -= 5; }, 
            HouseType::Clay =>  { self.clay -= 5; }, 
            HouseType::Stone => { self.stone -= 5; }
        }
    }

    pub fn build_room(&mut self) {
        if !self.can_build_room() {
            return;
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
        self.pay_for_room();
    }

    pub fn build_stables(&mut self) {
        let available_stables = 4 - self.stables;
        let max_stables = ::std::cmp::min(available_stables, self.wood / 2);
        if max_stables == 0 { return; }
        let num_stables = ::rand::random::<usize>() % (max_stables) + 1; // Always guarentee at least one stable
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
        let mut fences_built = 0;

        for i in 0..20 {
            let mut curr_pasture = HashSet::new();

            {
                let empty_spaces: Vec<&FarmTile> = self.player_mat.tiles.iter()
                                                                        .filter(|&t| t.can_be_fenced())
                                                                        .collect(); 
                    

                if let Some(temp_space) = ::rand::thread_rng().choose(&empty_spaces) {
                    let mut curr_space = *temp_space;
                    curr_pasture.insert(curr_space.index);
                    loop {
                        if ::rand::random::<usize>() % 100 < 20 {
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
                    // println!("[Make Pastures] No more empty spaces.. cannot make pasture");
                }
            }

            // println!("Fences: {} Wood: {}", self.fences, self.wood);
            // println!("Max fences: min({}, {})", 15-self.fences, self.wood);
            let max_fences = std::cmp::min(15-self.fences, self.wood);

            if curr_pasture.len() > 0 {
                if let Some(wood_used) = self.player_mat.make_pasture(curr_pasture.clone().into_iter().collect(), max_fences) {
                    // if wood_used+self.fences > 15 {
                        // println!("Max fences: min({}, {})", 15-self.fences, self.wood);
                        // println!("Wood used: {}", wood_used);
                    // }

                    let stables = curr_pasture.iter().filter(|&&t| self.player_mat.tiles[t].stable == true).collect::<Vec<&usize>>().len();
                    self.wood -= wood_used;
                    self.fences += wood_used;
                    self.pastures.push(
                        Pasture::new(curr_pasture.into_iter().collect(), stables)
                    );
                    fences_built += wood_used;
                }
            }
        }

        fences_built
    }

    pub fn place_animals(&mut self) {
        let orig_state = self.clone();
        let mut best_score = -999;
        let mut best_state = orig_state;

        // TODO: We try 10 times to find the best animal placement.. more/less?
        for i in 0..20 {
            // println!("i: {}", i);
            let mut temp_state = self.clone();

            temp_state.animal_reset();
            let mut animals_to_place = Vec::new();
            let mut sheep_count = temp_state.sheep;
            let mut boar_count = temp_state.boar;
            let mut cattle_count = temp_state.cattle;

            if sheep_count > 0 { animals_to_place.push("sheep"); }
            if boar_count > 0 { animals_to_place.push("boar"); }
            if cattle_count > 0 { animals_to_place.push("cattle"); }

            temp_state.sheep = 0;
            temp_state.boar = 0;
            temp_state.cattle = 0;

            // println!("[{:?}] -- Placing {} sheep, {} boar, {} cattle", animals_to_place, sheep_count, boar_count, cattle_count);

            while animals_to_place.len() > 0 {
                let available_pastures: Vec<&Pasture> = temp_state.pastures.iter()
                                                                           .filter(|p| {
                                                                                // Check if one of the tiles in the pasture is occupied
                                                                                temp_state.player_mat.tiles[p.tiles[0]].animal_type.is_none()
                                                                           }).collect();
                if let Some(ref mut pasture) = rand::thread_rng().choose(&available_pastures) {
                    let mut index = None;
                    if let Some(random_animal) = rand::thread_rng().choose(&animals_to_place) {
                        match random_animal {
                            &"sheep" => {
                                for tile_index in &pasture.tiles {
                                    if temp_state.player_mat.tiles[*tile_index].animal_count > 0 { continue; }
                                    if sheep_count >= pasture.capacity {
                                        // println!("[{}] {:?}", pasture.capacity, tile_index);
                                        let mut curr_tile = &mut temp_state.player_mat.tiles[*tile_index];
                                        curr_tile.animal_type = Some(Animal::Sheep);
                                        curr_tile.animal_count = pasture.capacity;
                                        sheep_count -= pasture.capacity;
                                        temp_state.sheep += pasture.capacity;
                                        if sheep_count == 0 {
                                            index = animals_to_place.iter().position(|x| *x == "sheep");
                                        }
                                    } else if sheep_count >= 0 {
                                        let mut curr_tile = &mut temp_state.player_mat.tiles[*tile_index];
                                        curr_tile.animal_type = Some(Animal::Sheep);
                                        curr_tile.animal_count = sheep_count;
                                        temp_state.sheep += sheep_count;
                                        sheep_count = 0;
                                        index = animals_to_place.iter().position(|x| *x == "sheep");
                                    }
                                }
                            },
                            &"boar" => {
                                for tile_index in &pasture.tiles {
                                    if boar_count >= pasture.capacity {
                                        let mut curr_tile = &mut temp_state.player_mat.tiles[*tile_index];
                                        curr_tile.animal_type = Some(Animal::Boar);
                                        curr_tile.animal_count = pasture.capacity;
                                        boar_count -= pasture.capacity;
                                        temp_state.boar += pasture.capacity;
                                        if boar_count == 0 {
                                            index = animals_to_place.iter().position(|x| *x == "boar");
                                        }
                                    } else if boar_count >= 0 {
                                        let mut curr_tile = &mut temp_state.player_mat.tiles[*tile_index];
                                        curr_tile.animal_type = Some(Animal::Boar);
                                        curr_tile.animal_count = boar_count;
                                        temp_state.boar += boar_count;
                                        boar_count = 0;
                                        index = animals_to_place.iter().position(|x| *x == "boar");
                                    }
                                }
                            },
                            &"cattle" => {
                                for tile_index in &pasture.tiles {
                                    if cattle_count >= pasture.capacity {
                                        let mut curr_tile = &mut temp_state.player_mat.tiles[*tile_index];
                                        curr_tile.animal_type = Some(Animal::Cattle);
                                        curr_tile.animal_count = pasture.capacity;
                                        cattle_count -= pasture.capacity;
                                        temp_state.cattle += pasture.capacity;
                                        if cattle_count == 0 {
                                            index = animals_to_place.iter().position(|x| *x == "cattle");
                                        }
                                    } else if cattle_count >= 0 {
                                        let mut curr_tile = &mut temp_state.player_mat.tiles[*tile_index];
                                        curr_tile.animal_type = Some(Animal::Cattle);
                                        curr_tile.animal_count = cattle_count;
                                        temp_state.cattle += cattle_count;
                                        cattle_count = 0;
                                        index = animals_to_place.iter().position(|x| *x == "cattle");
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                    if let Some(i) = index { animals_to_place.remove(i); }
                }
                // println!("[{:?}] -- No pastures.. trying single stables..", animals_to_place);
                for single_stable in temp_state.player_mat.tiles.iter_mut().filter(|t| t.stable == true && t.pasture == false && t.animal_type.is_none()) {
                    let mut index = None;
                    if let Some(random_animal) = rand::thread_rng().choose(&animals_to_place) {
                        match random_animal {
                            &"sheep" => { 
                                single_stable.animal_type = Some(Animal::Sheep); 
                                single_stable.animal_count = 1; 
                                sheep_count -= 1; 
                                temp_state.sheep += 1;
                                if sheep_count == 0 {
                                    index = animals_to_place.iter().position(|x| *x == "sheep");
                                }
                            },
                            &"boar" => {
                                single_stable.animal_type = Some(Animal::Boar); 
                                single_stable.animal_count = 1; 
                                boar_count -= 1; 
                                temp_state.boar += 1;
                                if boar_count == 0 {
                                    index = animals_to_place.iter().position(|x| *x == "boar");
                                }
                            },
                            &"cattle" => {
                                single_stable.animal_type = Some(Animal::Cattle); 
                                single_stable.animal_count = 1; 
                                cattle_count -= 1; 
                                temp_state.cattle += 1;
                                if cattle_count == 0 {
                                    index = animals_to_place.iter().position(|x| *x == "cattle");
                                }
                            },
                            _ => {}
                        }
                    }
                    if let Some(i) = index { 
                        animals_to_place.remove(i); 
                    }
                }

                let mut index = None;
                if temp_state.pet.is_none() && animals_to_place.len() > 0 {
                    // println!("[{:?}] -- No pastures left, no single stables, trying pet..", animals_to_place);
                    if let Some(random_animal) = rand::thread_rng().choose(&animals_to_place) {
                        match random_animal {
                            &"sheep" => { 
                                temp_state.pet = Some(Animal::Sheep); 
                                temp_state.player_mat.tiles[10].animal_type = Some(Animal::Sheep);  
                                temp_state.player_mat.tiles[10].animal_count = 1;  
                                temp_state.sheep += 1;
                                sheep_count -= 1;
                                if sheep_count == 0 {
                                    index = animals_to_place.iter().position(|x| *x == "sheep");
                                }
                            },
                            &"boar" => {
                                temp_state.pet = Some(Animal::Boar); 
                                temp_state.player_mat.tiles[10].animal_type = Some(Animal::Boar);  
                                temp_state.player_mat.tiles[10].animal_count = 1;  
                                temp_state.boar += 1;
                                boar_count -=1 ;
                                if boar_count == 0 {
                                    index = animals_to_place.iter().position(|x| *x == "boar");
                                }
                            },
                            &"cattle" => {
                                temp_state.pet = Some(Animal::Cattle); 
                                temp_state.player_mat.tiles[10].animal_type = Some(Animal::Cattle);  
                                temp_state.player_mat.tiles[10].animal_count = 1;  
                                temp_state.cattle += 1;
                                cattle_count -= 1;
                                if cattle_count == 0 {
                                    index = animals_to_place.iter().position(|x| *x == "cattle");
                                }
                            },
                            _ => {}
                        }
                    }
                }
                if let Some(i) = index { 
                    animals_to_place.remove(i); 
                }

                break; // nothing else is available.. break
            }

            if self.improvements.contains(&MajorImprovement::CookingHearth_4) || self.improvements.contains(&MajorImprovement::CookingHearth_5) && 
                (sheep_count > 0 || boar_count > 0 || cattle_count > 0) {
                // println!("Killing animals in cooking hearth: sheep {} ({}) boar {} ({}) cattle {} ({})", sheep_count, 2*sheep_count, boar_count, 2*boar_count, cattle_count, 3*cattle_count);
                temp_state.food += 2 * sheep_count;
                temp_state.food += 3 * boar_count;
                temp_state.food += 4 * cattle_count;
            } else if self.improvements.contains(&MajorImprovement::Fireplace_2) || self.improvements.contains(&MajorImprovement::Fireplace_3) && 
                (sheep_count > 0 || boar_count > 0 || cattle_count > 0) {
                // println!("Killing animals in fireplace: sheep {} ({}) boar {} ({}) cattle {} ({})", sheep_count, 2*sheep_count, boar_count, 2*boar_count, cattle_count, 3*cattle_count);
                temp_state.food += 2 * sheep_count;
                temp_state.food += 2 * boar_count;
                temp_state.food += 3 * cattle_count;
            }

            if temp_state.score(false) > best_score {
                best_score = temp_state.score(false);
                best_state = temp_state;
            }
        }

        self.sheep = best_state.sheep;
        self.boar = best_state.boar;
        self.cattle = best_state.cattle;
        self.player_mat = best_state.player_mat;
        self.food = best_state.food;
    }

    fn animal_reset(&mut self) {
        self.pet = None;
        for pasture in &self.pastures {
            for tile_index in &pasture.tiles {
                self.player_mat.tiles[*tile_index].animal_type = None;
                self.player_mat.tiles[*tile_index].animal_count = 0;
            }
        }

        for single_stable in self.player_mat.tiles.iter_mut().filter(|t| t.stable == true && t.pasture == false) {
            single_stable.animal_type = None;
        }

        self.player_mat.tiles[10].animal_type = None;
        self.player_mat.tiles[10].animal_count = 0;
    }

    pub fn bake_bread(&mut self) -> usize {
        if self.grains == 0 {
            return 0;
        }

        let mut food_gained = 0;
        if self.improvements.contains(&MajorImprovement::ClayOven) {
            self.grains -= 1;
            self.food += 5;
            food_gained += 5;
        } else if self.improvements.contains(&MajorImprovement::StoneOven) {
            match self.grains {
                2..100 => { self.grains -= 2; self.food += 8; food_gained += 8 },
                1   => { self.grains -= 1; self.food += 4; food_gained += 4 },
               _ => {},
            }
        } else if self.improvements.contains(&MajorImprovement::CookingHearth_4) || self.improvements.contains(&MajorImprovement::CookingHearth_5) {
            food_gained = self.grains * 3;
            self.food += self.grains * 3;
            self.grains = 0;
        } else if self.improvements.contains(&MajorImprovement::Fireplace_2) || self.improvements.contains(&MajorImprovement::Fireplace_3) {
            food_gained = self.grains * 2;
            self.food += food_gained;
            self.grains = 0;
        }
        food_gained
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Score: {}\n", self.score(false));
        write!(f, "[Food: {} Wood: {} Clay: {} Reed: {} Stone: {} Actions: {}/{} Fields: {} Beggers: {}]\n",
                self.food, self.wood, self.clay, self.reed, self.stone, self.actions, self.total_actions, self.fields, self.beggers);
        write!(f, "[Grain: {} Veg: {}]\n", self.grains, self.vegetables);
        write!(f, "[Sheep: {} Boar: {} Cattle: {}]\n", self.sheep, self.boar, self.cattle);
        write!(f, "[Fences: {}]\n", self.fences);
        write!(f, "[Pastures: {:?}]\n", self.pastures);
        write!(f, "{}", self.player_mat);
        write!(f, "Improvements: {:?}", self.improvements)
    }
}
