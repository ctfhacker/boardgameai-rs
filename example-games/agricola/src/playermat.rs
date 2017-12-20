use super::*;
use std::fmt;
use std::fmt::Display;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct PlayerMat {
    pub tiles: Vec<FarmTile>
}

impl PlayerMat {
    pub fn new() -> PlayerMat {
        let mut player_mat = Vec::new();
        for i in 0..15 {
            let mut new_tile = FarmTile::new(i);
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

    /*
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
    */

    pub fn plow_random_field(&mut self) {
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
        let random_field = ::rand::thread_rng().choose(&possible_fields).unwrap();
        self.tiles[*random_field].plow();
    }

    pub fn current_fences(&self) -> Vec<(usize, &str)> {
        let mut current_fences = Vec::new(); 
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


    /*
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
    */

    pub fn make_pasture(&mut self, curr_pasture: Vec<usize>, available_wood: usize) -> Option<usize> {
        let test_pasture = curr_pasture.clone();

        // Calculate how much wood is necessary to build the pasture before actually setting it
        let mut wood_needed = 0;
        for tile_index in &test_pasture {
            if wood_needed > available_wood {
                // println!("Not enough wood for pasture..");
                return None;
            }
            {
                let curr_tile = &mut self.tiles[*tile_index];
                if !curr_tile.can_be_fenced() {
                    panic!("Make pasture tile is currently occupied");
                    return None;
                }
            }

            match tile_index {
                &4|&9|&14 => { 
                    if !self.tiles[*tile_index].west_fence {
                        wood_needed += 1; 
                    }
                },
                _ => { 
                    if !curr_pasture.contains(&(tile_index+1)) && !self.tiles[*tile_index].west_fence {
                        wood_needed += 1;
                    }
                }
            }
            match tile_index {
                &0|&5|&10 => { 
                    if !self.tiles[*tile_index].east_fence {
                        wood_needed += 1; 
                    }
                },
                _ => { 
                    if !curr_pasture.contains(&(tile_index-1)) && !self.tiles[*tile_index].east_fence {
                        wood_needed += 1;
                    }
                }
            }
            match tile_index {
                &0|&1|&2|&3|&4 => { 
                    if !self.tiles[*tile_index].north_fence {
                        wood_needed += 1; 
                    }
                },
                _ => { 
                    if !curr_pasture.contains(&(tile_index-5)) && !self.tiles[*tile_index].north_fence {
                        wood_needed += 1;
                    }
                }
            }
            match tile_index {
                &10|&11|&12|&13|&14 => { 
                    if !self.tiles[*tile_index].south_fence {
                        wood_needed += 1; 
                    }
                },
                _ => { 
                    if !curr_pasture.contains(&(tile_index+5)) && !self.tiles[*tile_index].south_fence {
                        wood_needed += 1;
                    }
                }
            }
        }

        if wood_needed > available_wood {
            // println!("Not enough wood for pasture..");
            return None;
        }

        // println!("Before pasture: {:?}", curr_pasture);
        // println!("{}", self);
        
        // We have enough wood to make the pasture work.. actually set the pasture now.
        for tile_index in test_pasture {
            {
                let curr_tile = &mut self.tiles[tile_index];
                if !curr_tile.can_be_fenced() {
                    panic!("Make pasture tile is currently occupied");
                }
                curr_tile.pasture = true;
            }

            // let curr_tile = &mut self.tiles[tile_index];
            match tile_index {
                4|9|14 => { self.tiles[tile_index].east_fence = true },
                _ => { 
                    if !curr_pasture.contains(&(tile_index+1)) {
                        self.tiles[tile_index].east_fence = true;
                        self.tiles[tile_index+1].west_fence = true;
                    }
                }
            }
            match tile_index {
                0|5|10 => { self.tiles[tile_index].west_fence = true },
                _ => { 
                    if !curr_pasture.contains(&(tile_index-1)) {
                        self.tiles[tile_index].west_fence = true;
                        self.tiles[tile_index-1].east_fence = true;
                    }
                }
            }
            match tile_index {
                0..5 => { self.tiles[tile_index].north_fence = true },
                _ => { 
                    if !curr_pasture.contains(&(tile_index-5)) {
                        self.tiles[tile_index].north_fence = true;
                        self.tiles[tile_index-5].south_fence = true;
                    }
                }
            }
            match tile_index {
                10..15 => { self.tiles[tile_index].south_fence = true },
                _ => { 
                    if !curr_pasture.contains(&(tile_index+5)) {
                        self.tiles[tile_index].south_fence = true;
                        self.tiles[tile_index+5].north_fence = true;
                    }
                }
            }
        }
        // println!("[Wood: {}] After pasture: {:?}", wood_needed, curr_pasture);
        // println!("{}", self);
        Some(wood_needed)
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
