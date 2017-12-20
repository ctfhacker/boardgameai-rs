use super::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum AgricolaTile {
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
    Fences = 21,
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
            21 => Some(AgricolaAction::Fences),
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