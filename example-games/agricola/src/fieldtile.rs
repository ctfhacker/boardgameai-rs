use super::*;

#[derive(Debug, Clone)]
pub struct FieldTile {
    pub crop: Option<Crop>,
    pub count: usize
}

impl FieldTile {
    pub fn new() -> FieldTile {
        FieldTile {
            crop: None,
            count: 0
        }
    }

    pub fn new_with_crop(crop: Crop, count: usize) -> FieldTile {
        FieldTile {
            crop: Some(crop),
            count: count
        }
    }

    pub fn crop(&self) -> String {
        match self.crop {
            Some(Crop::Grain)     => String::from("Grain"),
            Some(Crop::Vegetable) => String::from("Veg  "),
            _                     => String::from("     ")
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        match self.crop {
            None => false,
            Some(ref field) => true
        }
    }

    pub fn is_vegetable(&self) -> bool {
        if let Some(ref crop) = self.crop {
            return crop == &Crop::Vegetable;
        }
        false
    }

    pub fn is_grain(&self) -> bool {
        if let Some(ref crop) = self.crop {
            return crop == &Crop::Grain;
        }
        false
    }
}
