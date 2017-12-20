use ::std::fmt;
use ::std::fmt::Display;
use ::rand::{thread_rng, Rng, sample};
use std::collections::{HashMap, HashSet};

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
pub enum HouseType {
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
pub enum Animal {
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
pub enum Crop {
    Grain,
    Vegetable
}

