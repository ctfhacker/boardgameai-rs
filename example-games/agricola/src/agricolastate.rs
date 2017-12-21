use super::*;
use std::fmt::Display;
use std::fmt;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct AgricolaState {
    pub players: Vec<Player>,
    pub player_just_moved: usize,
    pub current_player: usize,
    pub starting_player_token: Option<usize>,
    pub board: Board,
    pub rounds: usize,
    pub total_rounds: usize,
    pub actions_taken: Vec<String>,
    pub available_improvements: Vec<MajorImprovement>,
    well_player: Option<usize>,
    well_food: usize
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
                        actions.push(AgricolaAction::Fences as u32);
                    },
                    &AgricolaTile::MajorImprovement  => {
                        if self.available_improvements.contains(&MajorImprovement::Fireplace_2) {
                            actions.push(AgricolaAction::MajorImprovement_Fireplace_2 as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::Fireplace_3) {
                            actions.push(AgricolaAction::MajorImprovement_Fireplace_3 as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::CookingHearth_4) {
                            actions.push(AgricolaAction::MajorImprovement_CookingHearth_4 as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::CookingHearth_5) {
                            actions.push(AgricolaAction::MajorImprovement_CookingHearth_5 as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::ClayOven) {
                            actions.push(AgricolaAction::MajorImprovement_ClayOven as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::StoneOven) {
                            actions.push(AgricolaAction::MajorImprovement_StoneOven as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::Pottery) {
                            actions.push(AgricolaAction::MajorImprovement_Pottery as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::Joinery) {
                            actions.push(AgricolaAction::MajorImprovement_Joinery as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::BasketmakersWorkshop) {
                            actions.push(AgricolaAction::MajorImprovement_BasketmakersWorkshop as u32);
                        }
                    },
                    &AgricolaTile::Sheep  => actions.push(AgricolaAction::Sheep as u32),
                    &AgricolaTile::FamilyGrowth  => actions.push(AgricolaAction::FamilyGrowth as u32),
                    &AgricolaTile::Stone_1  => actions.push(AgricolaAction::Stone_1 as u32),
                    &AgricolaTile::Renovation_MajorImprovement  => {
                        if self.available_improvements.contains(&MajorImprovement::Fireplace_2) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_Fireplace_2 as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::Fireplace_3) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_Fireplace_3 as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::CookingHearth_4) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_CookingHearth_4 as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::CookingHearth_5) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_CookingHearth_5 as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::ClayOven) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_ClayOven as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::StoneOven) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_StoneOven as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::Pottery) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_Pottery as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::Joinery) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_Joinery as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::BasketmakersWorkshop) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_BasketmakersWorkshop as u32);
                        }
                        if self.available_improvements.contains(&MajorImprovement::Well) {
                            actions.push(AgricolaAction::Renovation_MajorImprovement_Well as u32);
                        }
                    },
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
        // println!("[R:{} P:{}] Action: {} {:?}", self.rounds, self.current_player, action, AgricolaAction::from_u32(action));
        let self_clone = self.clone();
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
                    // TODO Bake Bread
                    match agricola_action {
                        Some(AgricolaAction::Sow) => {
                            player.sow();
                            action_taken = format!("Sow").to_string();
                        },
                        Some(AgricolaAction::BakeBread_NotSow) => {
                            let food = player.bake_bread();
                            action_taken = format!("Bake Bread (+{}) and not Sow", food).to_string();
                        },
                        Some(AgricolaAction::Sow_BakeBread) =>  {
                            player.sow();
                            let food = player.bake_bread();
                            action_taken = format!("Sow and Bake Bread (+{})", food).to_string();
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
                    action_taken = format!("Starting Player and Food +{}", curr_tile.items).to_string();
                    self.starting_player_token = Some(self.current_player);
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Plow) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Plow).unwrap());
                    player.plow();
                    action_taken = format!("Plow").to_string();
                },
                Some(AgricolaAction::BuildStable) |
                Some(AgricolaAction::BakeBread_NoStable) |
                Some(AgricolaAction::BuildStable_BakeBread) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::BuildStable_BakeBread).unwrap());
                    // TODO Bake Bread
                    match agricola_action {
                        Some(AgricolaAction::BuildStable) => {
                            player.build_stable();
                            action_taken = format!("Build 1 stable").to_string();
                        }
                        Some(AgricolaAction::BakeBread_NoStable) => {
                            let food = player.bake_bread();
                            action_taken = format!("Bake Bread (+{})", food).to_string();
                        }
                        Some(AgricolaAction::BuildStable_BakeBread) => {
                            player.build_stable();
                            let food = player.bake_bread();
                            action_taken = format!("Build Stable and Bake Bread (+{})", food).to_string();
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
                    player.place_animals();
                    action_taken = format!("Sheep +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::MajorImprovement_Fireplace_2) |
                Some(AgricolaAction::MajorImprovement_Fireplace_3) |
                Some(AgricolaAction::MajorImprovement_CookingHearth_4) |
                Some(AgricolaAction::MajorImprovement_CookingHearth_5) | 
                Some(AgricolaAction::MajorImprovement_ClayOven) | 
                Some(AgricolaAction::MajorImprovement_StoneOven) |
                Some(AgricolaAction::MajorImprovement_Pottery) |
                Some(AgricolaAction::MajorImprovement_Joinery) |
                Some(AgricolaAction::MajorImprovement_BasketmakersWorkshop) => {
                    // println!("{}", self_clone);
                    // println!("{:?}", agricola_action);
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::MajorImprovement).unwrap());

                    match agricola_action {
                        Some(AgricolaAction::MajorImprovement_Fireplace_2) => {
                            if player.clay >= 2 && self.available_improvements.contains(&MajorImprovement::Fireplace_2) {
                                player.clay -= 2;
                                player.improvements.push(MajorImprovement::Fireplace_2);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Fireplace_2) {
                                    self.available_improvements.remove(index);
                                    action_taken = format!("Major Improvement +Fireplace (2)").to_string();
                                } else {
                                    // panic!("No idea why this Fireplace_2 isn't in the major improvement list");
                                    action_taken = format!("Tried to buy Fireplace (2), but it wasn't available").to_string();
                                }
                            }

                            if player.clay < 2 {
                                action_taken = format!("Tried to buy Fireplace (2), but not enough clay").to_string();
                            } else {
                                action_taken = format!("Tried to buy Fireplace (2), but it wasn't available..").to_string();
                            }
                        },
                        Some(AgricolaAction::MajorImprovement_Fireplace_3) => {
                            if player.clay >= 3 && self.available_improvements.contains(&MajorImprovement::Fireplace_3) {
                                player.clay -= 3;
                                player.improvements.push(MajorImprovement::Fireplace_3);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Fireplace_3) {
                                    self.available_improvements.remove(index);
                                    action_taken = format!("Major Improvement +Fireplace (3)").to_string();
                                } else {
                                    action_taken = format!("Tried to buy Fireplace (3), but it wasn't available").to_string();
                                }
                                action_taken = format!("Major Improvement +Fireplace (3)").to_string();
                            }

                            if player.clay < 3 {
                                action_taken = format!("Tried to buy Fireplace (3), but not enough clay").to_string();
                            } else {
                                action_taken = format!("Tried to buy Fireplace (3), but it wasn't available..").to_string();
                            }
                        },
                        Some(AgricolaAction::MajorImprovement_CookingHearth_4) => {
                            let mut action = None;

                            if player.clay >= 4 && (self.available_improvements.contains(&MajorImprovement::Fireplace_2) || 
                                                    self.available_improvements.contains(&MajorImprovement::Fireplace_3)) {
                                let coin_toss = ::rand::random::<usize>() % 2; 
                                match coin_toss {
                                    0 => { action = Some("pay")},
                                    1 => { action = Some("exchange")},
                                    _ => panic!("No other coin toss outcome..?!")
                                }
                            } else if player.clay >= 4 {
                                action = Some("pay");
                            } else if (self.available_improvements.contains(&MajorImprovement::Fireplace_2) || 
                                       self.available_improvements.contains(&MajorImprovement::Fireplace_3)) {
                                action = Some("exchange");
                            }

                            match action {
                                Some("pay") => {
                                    player.improvements.push(MajorImprovement::CookingHearth_4);
                                    if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_4) {
                                        self.available_improvements.remove(index);
                                        player.clay -= 4; 
                                        action_taken = format!("Major Improvement +Cooking Hearth (4) (Pay)").to_string();
                                    } else {
                                        action_taken = format!("Tried to buy Cooking Hearth (4), but it wasn't available").to_string();
                                    }
                                },
                                Some("exchange") => {
                                    if let Some(fireplace_index) = player.improvements.iter().position(|x| *x == MajorImprovement::Fireplace_3) {
                                        if let Some(cookinghearth_index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_4) {
                                            self.available_improvements.remove(cookinghearth_index);
                                            self.available_improvements.push(MajorImprovement::Fireplace_3);
                                            player.improvements.remove(fireplace_index);
                                            player.improvements.push(MajorImprovement::CookingHearth_4);
                                        } else {
                                            action_taken = format!("Tried to exchange for Cooking Hearth (4), but it wasn't available").to_string();
                                        }
                                    } else if let Some(fireplace_index) = player.improvements.iter().position(|x| *x == MajorImprovement::Fireplace_2) {
                                        if let Some(cookinghearth_index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_4) {
                                            self.available_improvements.remove(cookinghearth_index);
                                            self.available_improvements.push(MajorImprovement::Fireplace_2);
                                            player.improvements.remove(fireplace_index);
                                            player.improvements.push(MajorImprovement::CookingHearth_4);
                                            action_taken = format!("Exchange Fireplace (2) -> Cooking Hearth (4)").to_string();
                                        } else {
                                            action_taken = format!("Tried to exchange for Cooking Hearth (4), but it wasn't available").to_string();
                                        }
                                    }
                                },
                                _ => {} // Cannot pay or exchange for Cooking Hearth 4
                            }
                        },
                        Some(AgricolaAction::MajorImprovement_CookingHearth_5) => {
                            let mut action = None;

                            if player.clay >= 5 && (self.available_improvements.contains(&MajorImprovement::Fireplace_2) || 
                                                    self.available_improvements.contains(&MajorImprovement::Fireplace_3)) {
                                let coin_toss = ::rand::random::<usize>() % 2; 
                                match coin_toss {
                                    0 => { action = Some("pay")},
                                    1 => { action = Some("exchange")},
                                    _ => panic!("No other coin toss outcome..?!")
                                }
                            } else if player.clay >= 5 {
                                action = Some("pay");
                            } else if (self.available_improvements.contains(&MajorImprovement::Fireplace_2) || 
                                       self.available_improvements.contains(&MajorImprovement::Fireplace_3)) {
                                action = Some("exchange");
                            }

                            match action {
                                Some("pay") => {
                                    player.improvements.push(MajorImprovement::CookingHearth_5);
                                    if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_5) {
                                        player.clay -= 5; 
                                        self.available_improvements.remove(index);
                                        action_taken = format!("Major Improvement +Cooking Hearth (5) (Pay)").to_string();
                                    } else {
                                        // Cooking Hearth 5 is already taken
                                        action_taken = format!("Tried to buy Cooking Hearth (5), but it wasn't available").to_string();
                                    }
                                },
                                Some("exchange") => {
                                    if let Some(fireplace_index) = player.improvements.iter().position(|x| *x == MajorImprovement::Fireplace_3) {
                                        if let Some(cookinghearth_index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_5) {
                                            self.available_improvements.remove(cookinghearth_index);
                                            self.available_improvements.push(MajorImprovement::Fireplace_3);
                                            player.improvements.remove(fireplace_index);
                                            player.improvements.push(MajorImprovement::CookingHearth_5);
                                            action_taken = format!("Major Improvement Fireplace (3) -> Cooking Hearth (5)").to_string();
                                        } else {
                                            // Cooking Hearth 5 is already taken
                                            action_taken = format!("Tried to buy Cooking Hearth (5), but it wasn't available").to_string();
                                        }
                                    } else if let Some(fireplace_index) = player.improvements.iter().position(|x| *x == MajorImprovement::Fireplace_2) {
                                        if let Some(cookinghearth_index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_5) {
                                            self.available_improvements.remove(cookinghearth_index);
                                            self.available_improvements.push(MajorImprovement::Fireplace_2);
                                            player.improvements.remove(fireplace_index);
                                            player.improvements.push(MajorImprovement::CookingHearth_5);
                                            action_taken = format!("Major Improvement Fireplace (2) -> Cooking Hearth (5)").to_string();
                                        } else {
                                            // Cooking Hearth 5 is already taken
                                            action_taken = format!("Tried to buy Cooking Hearth (5), but it wasn't available").to_string();
                                        }
                                    }
                                },
                                _ => {} // Cannot pay or exchange for Cooking Hearth 5
                            }
                        },
                        Some(AgricolaAction::MajorImprovement_ClayOven) => {
                            if player.clay >= 3 && player.stone >= 1 && self.available_improvements.contains(&MajorImprovement::ClayOven) {
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::ClayOven) {
                                    self.available_improvements.remove(index);
                                    player.clay -= 3;
                                    player.stone -= 1;
                                    player.improvements.push(MajorImprovement::ClayOven);
                                    let food = player.bake_bread();
                                    action_taken = format!("Major Improvement - Clay Oven (BakeBread +{})", food).to_string();
                                } else {
                                    // Clay Oven taken
                                    // panic!("No idea why this ClayOven isn't in the major improvement list");
                                    action_taken = format!("Tried to buy Clay Oven, but it wasn't available").to_string();
                                }
                            }

                            if player.clay < 3 {
                                action_taken = format!("Tried to buy Clay Oven, but not enough clay").to_string();
                            } else if player.stone < 1 {
                                action_taken = format!("Tried to buy Clay Oven, but not enough stone").to_string();
                            } else {
                                action_taken = format!("Tried to buy Clay Oven, but it wasn't available..").to_string();
                            }
                        },
                        Some(AgricolaAction::MajorImprovement_StoneOven) => {
                            if player.clay >= 1 && player.stone >= 3 && self.available_improvements.contains(&MajorImprovement::StoneOven) {
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::StoneOven) {
                                    self.available_improvements.remove(index);
                                    player.clay -= 1;
                                    player.stone -= 3;
                                    player.improvements.push(MajorImprovement::StoneOven);
                                    let food = player.bake_bread();
                                    action_taken = format!("Major Improvement - Stone Oven (BakeBread +{})", food).to_string();
                                } else {
                                    // Stone Oven taken
                                    action_taken = format!("Tried to buy Stone Oven, but it wasn't available").to_string();
                                }
                            }
                            if player.clay == 0 {
                                action_taken = format!("Tried to buy Stone Oven, but not enough clay").to_string();
                            } else if player.stone < 3 {
                                action_taken = format!("Tried to buy Stone Oven, but not enough stone").to_string();
                            } else {
                                action_taken = format!("Tried to buy Stone Oven, but it wasn't available..").to_string();
                            }
                        },
                        Some(AgricolaAction::MajorImprovement_Pottery) => {
                            if player.clay >= 2 && player.stone >= 2 && self.available_improvements.contains(&MajorImprovement::Pottery) {
                                player.clay -= 2;
                                player.stone -= 2;
                                player.improvements.push(MajorImprovement::Pottery);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Pottery) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this Pottery isn't in the major improvement list");
                                }
                                action_taken = format!("Major Improvement - Pottery").to_string();
                            }
                        },
                        Some(AgricolaAction::MajorImprovement_Joinery) => {
                            if player.wood >= 2 && player.stone >= 2 && self.available_improvements.contains(&MajorImprovement::Joinery) {
                                player.wood -= 2;
                                player.stone -= 2;
                                player.improvements.push(MajorImprovement::Joinery);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Joinery) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this Joinery isn't in the major improvement list");
                                }
                                action_taken = format!("Major Improvement - Joinery").to_string();
                            }
                        },
                        Some(AgricolaAction::MajorImprovement_BasketmakersWorkshop) => {
                            if player.reed >= 2 && player.stone >= 2 && self.available_improvements.contains(&MajorImprovement::BasketmakersWorkshop) {
                                player.reed -= 2;
                                player.stone -= 2;
                                player.improvements.push(MajorImprovement::BasketmakersWorkshop);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::BasketmakersWorkshop) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this BasketmakersWorkshop isn't in the major improvement list");
                                }
                                action_taken = format!("Major Improvement - BasketmakersWorkshop").to_string();
                            }
                        },
                        Some(AgricolaAction::MajorImprovement_Well) => {
                            if player.wood >= 1 && player.stone >= 3 && self.available_improvements.contains(&MajorImprovement::Well) {
                                player.wood -= 1;
                                player.stone -= 3;
                                player.improvements.push(MajorImprovement::Well);
                                self.well_player = Some(self.current_player);
                                self.well_food = 5;
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Well) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this Well isn't in the major improvement list");
                                }
                                action_taken = format!("Major Improvement - Well").to_string();
                            }
                        },
                        _ => panic!("No other actions available for major improvements..")
                    }
                    action_taken = format!("Major Improvement +{:?}", agricola_action).to_string();

                },
                Some(AgricolaAction::Fences) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Fences).unwrap());
                    let fences_built = player.make_pastures();
                    // player.fences += fences_built;
                    action_taken = format!("Fences +{}", fences_built).to_string();
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
                Some(AgricolaAction::Renovation_MajorImprovement_Fireplace_2) |
                Some(AgricolaAction::Renovation_MajorImprovement_Fireplace_3) |
                Some(AgricolaAction::Renovation_MajorImprovement_CookingHearth_4) |
                Some(AgricolaAction::Renovation_MajorImprovement_CookingHearth_5) |
                Some(AgricolaAction::Renovation_MajorImprovement_ClayOven) |
                Some(AgricolaAction::Renovation_MajorImprovement_StoneOven) |
                Some(AgricolaAction::Renovation_MajorImprovement_Pottery) |
                Some(AgricolaAction::Renovation_MajorImprovement_Joinery) |
                Some(AgricolaAction::Renovation_MajorImprovement_BasketmakersWorkshop) |
                Some(AgricolaAction::Renovation_MajorImprovement_Well) => {
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

                    match agricola_action {
                        Some(AgricolaAction::Renovation_MajorImprovement_Fireplace_2) => {
                            if player.clay >= 2 && self.available_improvements.contains(&MajorImprovement::Fireplace_2) {
                                player.clay -= 2;
                                player.improvements.push(MajorImprovement::Fireplace_2);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Fireplace_2) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this Fireplace_2 isn't in the major improvement list");
                                }
                                action_taken = format!("Renovation and Major Improvement +Fireplace (2)").to_string();
                            }
                        },
                        Some(AgricolaAction::Renovation_MajorImprovement_Fireplace_3) => {
                            if player.clay >= 3 && self.available_improvements.contains(&MajorImprovement::Fireplace_3) {
                                player.clay -= 3;
                                player.improvements.push(MajorImprovement::Fireplace_3);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Fireplace_3) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this Fireplace_3 isn't in the major improvement list");
                                }
                                action_taken = format!("Renovation and Major Improvement +Fireplace (3)").to_string();
                            }
                        },
                        Some(AgricolaAction::Renovation_MajorImprovement_CookingHearth_4) => {
                            let mut action = None;

                            if player.clay >= 4 && (self.available_improvements.contains(&MajorImprovement::Fireplace_2) || 
                                                    self.available_improvements.contains(&MajorImprovement::Fireplace_3)) {
                                let coin_toss = ::rand::random::<usize>() % 2; 
                                match coin_toss {
                                    0 => { action = Some("pay")},
                                    1 => { action = Some("exchange")},
                                    _ => panic!("No other coin toss outcome..?!")
                                }
                            } else if player.clay >= 4 {
                                action = Some("pay");
                            } else if (self.available_improvements.contains(&MajorImprovement::Fireplace_2) || 
                                       self.available_improvements.contains(&MajorImprovement::Fireplace_3)) {
                                action = Some("exchange");
                            }

                            match action {
                                Some("pay") => {
                                    player.improvements.push(MajorImprovement::CookingHearth_4);
                                    if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_4) {
                                        player.clay -= 4; 
                                        self.available_improvements.remove(index);
                                    } else {
                                        // Cooking Hearth 4 already taken
                                    }
                                    action_taken = format!("Renovation and Major Improvement +Cooking Hearth (4) (Pay)").to_string();
                                },
                                Some("exchange") => {
                                    if let Some(fireplace_index) = player.improvements.iter().position(|x| *x == MajorImprovement::Fireplace_3) {
                                        if let Some(cookinghearth_index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_4) {
                                            self.available_improvements.remove(cookinghearth_index);
                                            self.available_improvements.push(MajorImprovement::Fireplace_3);
                                            player.improvements.remove(fireplace_index);
                                            player.improvements.push(MajorImprovement::CookingHearth_4);
                                        } else {
                                            // Cooking Hearth 4 is already taken
                                        }
                                    } else if let Some(fireplace_index) = player.improvements.iter().position(|x| *x == MajorImprovement::Fireplace_2) {
                                        if let Some(cookinghearth_index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_4) {
                                            self.available_improvements.remove(cookinghearth_index);
                                            self.available_improvements.push(MajorImprovement::Fireplace_2);
                                            player.improvements.remove(fireplace_index);
                                            player.improvements.push(MajorImprovement::CookingHearth_4);
                                        } else {
                                            // Cooking Hearth 4 is already taken
                                        }
                                    }
                                    action_taken = format!("Renovation and Major Improvement +Cooking Hearth (4) (Exchange)").to_string();
                                },
                                _ => {} // Cannot pay or exchange for Cooking Hearth 4
                            }
                        },
                        Some(AgricolaAction::Renovation_MajorImprovement_CookingHearth_5) => {
                            let mut action = None;

                            if player.clay >= 5 && (self.available_improvements.contains(&MajorImprovement::Fireplace_2) || 
                                                    self.available_improvements.contains(&MajorImprovement::Fireplace_3)) {
                                let coin_toss = ::rand::random::<usize>() % 2; 
                                match coin_toss {
                                    0 => { action = Some("pay")},
                                    1 => { action = Some("exchange")},
                                    _ => panic!("No other coin toss outcome..?!")
                                }
                            } else if player.clay >= 5 {
                                action = Some("pay");
                            } else if (self.available_improvements.contains(&MajorImprovement::Fireplace_2) || 
                                       self.available_improvements.contains(&MajorImprovement::Fireplace_3)) {
                                action = Some("exchange");
                            }

                            match action {
                                Some("pay") => {
                                    player.clay -= 5; 
                                    player.improvements.push(MajorImprovement::CookingHearth_5);
                                    if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_5) {
                                        self.available_improvements.remove(index);
                                    } else {
                                        // Cooking Hearth 5 already taken
                                    }
                                    action_taken = format!("Renovation and Major Improvement +Cooking Hearth (5) (Pay)").to_string();
                                },
                                Some("exchange") => {
                                    if let Some(fireplace_index) = player.improvements.iter().position(|x| *x == MajorImprovement::Fireplace_3) {
                                        if let Some(cookinghearth_index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_5) {
                                            self.available_improvements.remove(cookinghearth_index);
                                            self.available_improvements.push(MajorImprovement::Fireplace_3);
                                            player.improvements.remove(fireplace_index);
                                            player.improvements.push(MajorImprovement::CookingHearth_5);
                                        } else {
                                            // Cooking Hearth 5 is already taken
                                        }
                                    } else if let Some(fireplace_index) = player.improvements.iter().position(|x| *x == MajorImprovement::Fireplace_2) {
                                        if let Some(cookinghearth_index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::CookingHearth_5) {
                                            self.available_improvements.remove(cookinghearth_index);
                                            self.available_improvements.push(MajorImprovement::Fireplace_2);
                                            player.improvements.remove(fireplace_index);
                                            player.improvements.push(MajorImprovement::CookingHearth_5);
                                        } else {
                                            // Cooking Hearth 5 is already taken
                                        }
                                    }
                                    action_taken = format!("Renovation and Major Improvement +Cooking Hearth (5) (Exchange)").to_string();
                                },
                                _ => {} // Cannot pay or exchange for Cooking Hearth 5
                            }
                        },
                        Some(AgricolaAction::Renovation_MajorImprovement_ClayOven) => {
                            if player.clay >= 3 && player.stone >= 1 && self.available_improvements.contains(&MajorImprovement::ClayOven) {
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::ClayOven) {
                                    self.available_improvements.remove(index);
                                    player.clay -= 3;
                                    player.stone -= 1;
                                    player.improvements.push(MajorImprovement::ClayOven);
                                    player.bake_bread();
                                } else {
                                    // Clay Oven taken
                                    // panic!("No idea why this ClayOven isn't in the major improvement list");
                                }
                                action_taken = format!("Renovation and Major Improvement - Clay Oven").to_string();
                            }
                        },
                        Some(AgricolaAction::Renovation_MajorImprovement_StoneOven) => {
                            if player.clay >= 1 && player.stone >= 3 && self.available_improvements.contains(&MajorImprovement::StoneOven) {
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::StoneOven) {
                                    self.available_improvements.remove(index);
                                    player.clay -= 1;
                                    player.stone -= 3;
                                    player.improvements.push(MajorImprovement::StoneOven);
                                    player.bake_bread();
                                } else {
                                    // Stone Oven taken
                                    // panic!("No idea why this StoneOven isn't in the major improvement list");
                                }
                                action_taken = format!("Renovation and Major Improvement - Stone Oven").to_string();
                            }
                        },
                        Some(AgricolaAction::Renovation_MajorImprovement_Pottery) => {
                            if player.clay >= 2 && player.stone >= 2 && self.available_improvements.contains(&MajorImprovement::Pottery) {
                                player.clay -= 2;
                                player.stone -= 2;
                                player.improvements.push(MajorImprovement::Pottery);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Pottery) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this Pottery isn't in the major improvement list");
                                }
                                action_taken = format!("Renovation and Major Improvement - Pottery").to_string();
                            }
                        },
                        Some(AgricolaAction::Renovation_MajorImprovement_Joinery) => {
                            if player.wood >= 2 && player.stone >= 2 && self.available_improvements.contains(&MajorImprovement::Joinery) {
                                player.wood -= 2;
                                player.stone -= 2;
                                player.improvements.push(MajorImprovement::Joinery);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Joinery) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this Joinery isn't in the major improvement list");
                                }
                                action_taken = format!("Renovation and Major Improvement - Joinery").to_string();
                            }
                        },
                        Some(AgricolaAction::Renovation_MajorImprovement_BasketmakersWorkshop) => {
                            if player.reed >= 2 && player.stone >= 2 && self.available_improvements.contains(&MajorImprovement::BasketmakersWorkshop) {
                                player.reed -= 2;
                                player.stone -= 2;
                                player.improvements.push(MajorImprovement::BasketmakersWorkshop);
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::BasketmakersWorkshop) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this BasketmakersWorkshop isn't in the major improvement list");
                                }
                                action_taken = format!("Renovation and Major Improvement - BasketmakersWorkshop").to_string();
                            }
                        },
                        Some(AgricolaAction::Renovation_MajorImprovement_Well) => {
                            if player.wood >= 1 && player.stone >= 3 && self.available_improvements.contains(&MajorImprovement::Well) {
                                player.wood -= 1;
                                player.stone -= 3;
                                player.improvements.push(MajorImprovement::Well);
                                self.well_player = Some(self.current_player);
                                self.well_food = 5;
                                if let Some(index) = self.available_improvements.iter().position(|x| *x == MajorImprovement::Well) {
                                    self.available_improvements.remove(index);
                                } else {
                                    panic!("No idea why this Well isn't in the major improvement list");
                                }
                                action_taken = format!("Renovation and Major Improvement - Well").to_string();
                            }
                        },
                        _ => panic!("No other actions available for major improvements..")
                    }

                    action_taken = format!("{:?}", agricola_action).to_string();
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
                    player.place_animals();
                    action_taken = format!("Boar +{}", curr_tile.items).to_string();
                    curr_tile.items = 0;
                },
                Some(AgricolaAction::Cattle) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::Cattle).unwrap());
                    if !curr_tile.occupied.is_none() {
                        panic!("Player {} is bad.. Cattle is already taken", player_index);
                    }
                    player.cattle += curr_tile.items;
                    player.place_animals();
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
                            action_taken = format!("Plow but No Sow").to_string();
                        }
                        Some(AgricolaAction::Sow_NoPlow) => {
                            player.sow();
                            action_taken = format!("Sow but No Plow").to_string();
                        }
                        Some(AgricolaAction::Plow_Sow) => {
                            player.plow();
                            player.sow();
                            action_taken = format!("Plow and Sow").to_string();
                        },
                        _ => panic!("[Plow_Sow] Can never reach here..")
                    }
                },
                Some(AgricolaAction::FamilyGrowth_NoSpace) => {
                    curr_tile = &mut *(self.board.tiles.get_mut(&AgricolaTile::FamilyGrowth_NoSpace).unwrap());
                    player.children = 1;
                    action_taken = format!("Family Growth with no space").to_string();
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

                    let fences_built = player.make_pastures();
                    action_taken = format!("Renovation and Fences +{}", fences_built).to_string();
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
            scores.push(player.score(false));
        }

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

        /// Player one gets 2 food while others get 3
        for i in 0..num_players {
            if i == 0 {
                players.push(Player::new(2));
            } else {
                players.push(Player::new(3));
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
            actions_taken: Vec::new(),
            available_improvements: vec!(MajorImprovement::Fireplace_2, MajorImprovement::Fireplace_3, MajorImprovement::CookingHearth_4, 
                                         MajorImprovement::CookingHearth_5, MajorImprovement::ClayOven, MajorImprovement::StoneOven, 
                                         MajorImprovement::Joinery, MajorImprovement::Pottery, MajorImprovement::BasketmakersWorkshop, 
                                         MajorImprovement::Well),
            well_player: None,
            well_food: 0
        }
    }

    pub fn end_round(&mut self) {
        // println!("Ending round");
        

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
                // Check Pottery, Joinery, Basketmakers Workshop
                for ref mut player in self.players.iter_mut() {
                    if player.improvements.contains(&MajorImprovement::Pottery) && player.clay > 0 {
                        player.clay -= 1;
                        player.food += 2;
                    }
                    if player.improvements.contains(&MajorImprovement::Joinery) && player.wood > 0 {
                        player.wood -= 1;
                        player.food += 2;
                    }
                    if player.improvements.contains(&MajorImprovement::BasketmakersWorkshop) && player.reed > 0 {
                        player.reed -= 1;
                        player.food += 3;
                    }
                }

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
                            if let Some(ref mut field) = curr_tile.field {
                                field.crop = None
                            }
                        }
                    }
                    let player_display = format!("{}", player);
                    player.actions_taken.push(format!("Field Phase:\n{}", player_display));
                }
                
                // Feeding Phase
                for mut player in self.players.iter_mut() {
                    if player.food >= (player.total_actions * 2) + (player.children) {
                        player.food -= player.total_actions * 2 + (player.children);
                    } else {
                        let mut food_needed = (player.total_actions * 2) + player.children - player.food;
                        player.food = 0;

                        // Cannot kill animals, so only leverage grain and vegetables in raw form
                        if !(player.improvements.contains(&MajorImprovement::Fireplace_2) || 
                             player.improvements.contains(&MajorImprovement::Fireplace_3)) {
                            loop {
                                if food_needed == 0 || (player.grains == 0 && player.vegetables == 0) {
                                    break;
                                }
                                if player.grains > 0 {
                                    player.grains -= 1;
                                    food_needed -= 1;
                                    continue;
                                }
                                if player.vegetables > 0 {
                                    player.vegetables -= 1;
                                    food_needed -= 1;
                                }
                            }
                        }

                        if food_needed > 0 && (player.improvements.contains(&MajorImprovement::Fireplace_2) || player.improvements.contains(&MajorImprovement::Fireplace_3)) {
                            let mut best_score = player.score(false) - (food_needed * 3) as i32; // Simulate taking beggers
                            let mut best_state = player.clone();
                            let mut best_food_needed = food_needed;
                            let orig_food_needed = food_needed;
                            for a in 0..100 {
                                let mut temp_state = player.clone();
                                let mut food_needed = orig_food_needed;
                                while food_needed > 0 && (temp_state.sheep > 0 || temp_state.boar > 0 || temp_state.cattle > 0 || temp_state.vegetables > 0) {
                                    let possibles = vec!("s", "b", "c", "v", "g");
                                    let animal = rand::thread_rng().choose(&possibles);
                                    match animal {
                                        Some(&"s") => { 
                                            if temp_state.sheep == 0 { continue; }
                                            temp_state.sheep -= 1;
                                            if food_needed == 1 {
                                                food_needed = 0;
                                                temp_state.food = 1;
                                            } else {
                                                food_needed -= 2;
                                            }

                                        },
                                        Some(&"b") => { 
                                            if temp_state.boar == 0 { continue; }
                                            temp_state.boar -= 1;
                                            if food_needed == 1 {
                                                food_needed = 0;
                                                temp_state.food = 1;
                                            } else {
                                                food_needed -= 2;
                                            }
                                        },
                                        Some(&"c") => { 
                                            if temp_state.cattle == 0 { continue; }
                                            temp_state.cattle -= 1;
                                            if food_needed >= 3 {
                                                food_needed -= 3
                                            } else {
                                                food_needed = 0;
                                                temp_state.food = 3 - food_needed;
                                            }
                                        },
                                        Some(&"v") => { 
                                            if temp_state.vegetables == 0 { continue; }
                                            temp_state.vegetables -= 1;
                                            if food_needed == 1 {
                                                food_needed = 0;
                                                temp_state.food = 1;
                                            } else {
                                                food_needed -= 2;
                                            }
                                        },
                                        Some(&"g") => { 
                                            if temp_state.grains == 0 { continue; }
                                            temp_state.grains -= 1;
                                            food_needed -= 1;
                                        },
                                        _ => panic!("No idea what other animal/grain/vegetable can be triggered for feeding..")
                                    }
                                }

                                if temp_state.score(false) > best_score {
                                    // println!("Better score: {} -> {}", best_score, temp_state.score(false));
                                    best_score = temp_state.score(false);
                                    best_state = temp_state;
                                    best_food_needed = food_needed;
                                }
                            }
                            player.sheep = best_state.sheep;
                            player.boar = best_state.boar;
                            player.cattle = best_state.cattle;
                            player.food = best_state.food;
                            player.grains = best_state.grains;
                           }

                        player.beggers += food_needed;
                    }

                    let player_display = format!("{}", player);
                    player.actions_taken.push(format!("Feeding Phase:\n{}", player_display));
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

                    player.place_animals();
                    let player_display = format!("{}", player);
                    player.actions_taken.push(format!("Breeding Phase:\n{}", player_display));
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

        // Reset actions for all players
        for mut player in self.players.iter_mut() {
            player.actions = player.total_actions;
        }

        self.rounds += 1;

        if self.well_food > 0 {
            if let Some(food_player) = self.well_player {
                self.players[food_player].food += 1;
                self.well_food -= 1;
                self.players[food_player].actions_taken.push(format!("Well: +1 Food"));
            }
        }
    }

    pub fn add_action(&mut self, player: usize, action: String) {
        let curr_player = &mut self.players[player];
        let curr_player_display = format!("{}", curr_player);
        self.actions_taken.push(format!("Round: {} Player: {} [{}/{}] {}", self.rounds, player, curr_player.total_actions-curr_player.actions, curr_player.total_actions, action));
        curr_player.actions_taken.push(format!("Round: {} [{}/{}] {}\n{}", self.rounds, curr_player.total_actions-curr_player.actions, curr_player.total_actions, action, curr_player_display));
    }

    pub fn print_ending(&self) {

        for (i, player) in self.players.iter().enumerate() {
            for action in &player.actions_taken {
                println!("{}", action);
            }
            println!("Player {}: {}\n{}", i, player.score(true), player);
        }

        let mut scores = Vec::new();
        for (i, player) in self.players.iter().enumerate() {

            println!("-- Player {}\n{}", i, player);
            scores.push(player.score(true));
        }
    }
}

impl Display for AgricolaState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player Just Played: {}\n", self.player_just_moved + 1);
        for (i, player) in self.players.iter().enumerate() {
            let num_actions_taken = player.actions_taken.len();
            write!(f, "P: {} {}\n", i+1, player.actions_taken.iter().nth(num_actions_taken-1).unwrap());
        }
        write!(f, "{}", self.board);
        write!(f, "Next Player: {}\n", self.current_player + 1)
    }
}
