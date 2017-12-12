extern crate boardgameai_rs;
use boardgameai_rs::*;
use boardgameai_rs::state::State;
use boardgameai_rs::action::Action;

#[derive(Debug, Clone, Copy)]
pub struct NimState {
    player_just_moved: usize,
    chips: u32
}

#[derive(Debug)]
enum NimAction {
    One = 1,
    Two = 2,
    Three = 3
}

impl NimAction {
    fn from_u32(x: u32) -> Option<NimAction> {
        match x {
            1 => Some(NimAction::One),
            2 => Some(NimAction::Two),
            3 => Some(NimAction::Three),
            _ => None
        }
    }
}

impl State for NimState {
    fn get_player_just_moved(&self) -> usize {
        self.player_just_moved
    }

    fn get_actions(&self) -> Vec<u32> {
        let mut actions = Vec::new();
        if self.chips >= 1 {
            actions.push(NimAction::One as u32);
        } 
        if self.chips >= 2 {
            actions.push(NimAction::Two as u32);
        } 
        if self.chips >= 3 {
            actions.push(NimAction::Three as u32);
        } 
        actions
    }

    fn get_action_strings(&self) -> Vec<String> {
        let mut strings = Vec::new();
        for action in self.get_actions() {
            strings.push(format!("{:?}", NimAction::from_u32(action).unwrap()));
        }
        strings
    }

    fn do_action(&mut self, action: u32) {
        self.chips -= action;
        // Player 1: 3 - 1 -> 2
        // Player 2: 3 - 2 -> 1
        self.player_just_moved = 3 - self.player_just_moved
    }

    fn get_result(&self, player: usize) -> f32 {
        if self.player_just_moved == player {
            return 1.0;
        } else {
            return 0.0;
        }
    }
}

impl NimState {
    pub fn new(chips: u32) -> NimState {
        NimState {
            player_just_moved: 2,
            chips: chips
        }
    }
}
