use super::*;

#[derive(Debug, Clone)]
pub struct FarmTile {
    pub house: Option<HouseType>,
    pub stable: bool,
    pub pasture: bool,
    pub animal_type: Option<Animal>,
    pub animal_count: usize,
    pub north_fence: bool,
    pub south_fence: bool,
    pub east_fence: bool,
    pub west_fence: bool,
    pub field: Option<FieldTile>,
    pub surrounding_tiles: Vec<usize>,
    pub index: usize
}

impl FarmTile {
    pub fn new(index: usize) -> FarmTile {
        FarmTile {
            house: None,
            stable: false,
            pasture: false,
            animal_type: None,
            animal_count: 0,
            north_fence: false,
            south_fence: false,
            east_fence: false,
            west_fence: false,
            field: None,
            surrounding_tiles: Vec::new(),
            index: index
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.house.is_none() && self.field.is_none() && !self.pasture && !self.stable 
    }

    pub fn can_be_fenced(&self) -> bool {
        self.house.is_none() && self.field.is_none() && !self.pasture
    }

    pub fn plow(&mut self) {
        self.field = Some(FieldTile::new());
    }

    pub fn build_room(&mut self, house_type: HouseType) {
        self.house = Some(house_type);
    }

    pub fn stable(&mut self) {
        self.stable = true;
    }

    pub fn sow_veg(&mut self) {
        if let Some(ref mut field) = self.field {
            field.crop = Some(Crop::Vegetable);
            field.count = 2;
        }
    }

    pub fn sow_grain(&mut self) {
        if let Some(ref mut field) = self.field {
            field.crop = Some(Crop::Grain);
            field.count = 3;
        }
    }

    pub fn upgrade(&mut self) {
        match self.house {
            Some(HouseType::Wood) => self.house = Some(HouseType::Clay),
            Some(HouseType::Clay) => self.house = Some(HouseType::Stone),
            _ => {},
        }
    }
}

