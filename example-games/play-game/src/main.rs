extern crate boardgameai_rs;
extern crate nim;
extern crate agricola;

use boardgameai_rs::state::State;
use boardgameai_rs::node::{NodeArena, NodeId};
use nim::NimState;
use agricola::AgricolaState;
use agricola::AgricolaAction;
use std::fmt::Debug;
use std::ops::Deref;
use std::fs::File;
use std::io::Read;

fn random() -> usize {
    let mut urandom = File::open("/dev/urandom").ok().unwrap();
    let mut buf = [0, 1];
    urandom.read_exact(&mut buf);
    buf[0] as usize
}

fn UCT<S: State+Clone+Debug>(arena: &mut NodeArena, mut rootstate: S, iterations: u32) -> u32 {
    let mut rootnode = arena.new_node(rootstate.clone());

    for i in 1..iterations {
        // println!("i: {}", i);
        let mut curr_node = rootnode;

        //let mut node = &arena[rootnode].clone();
        let mut node: NodeId;
        let mut state = rootstate.clone();

        // println!("Before select: {:?}", curr_node);
        // Select
        // Find a full expanded, non-terminal node to start this state
        loop {
            if(arena[curr_node].untried_actions.len() != 0 || 
               arena[curr_node].children.len() == 0){
                break
            }

            let UCTK: f64 = 1.4;
            let mut best_value = (-1 as f64);

            for child in &arena[curr_node].children {
                let curr_child = &arena[*child];
                let curr_value = (curr_child.wins as f64)/(curr_child.visits as f64) + UCTK * ((((arena[curr_node].visits as f64).ln() * 2.0) / (curr_child.visits as f64)).sqrt());
                /*
                println!("{}/{} + 1.4 * sqrt(2*log({})/{})", (curr_child.wins as f64), (curr_child.visits as f64), (arena[curr_node].visits as f64), curr_child.visits as f64);
                println!("{}/{} + 1.4 * sqrt({}/{})", 
                    (curr_child.wins as f64), 
                    (curr_child.visits as f64), 
                    (arena[curr_node].visits as f64).ln() * 2.0, 
                    curr_child.visits as f64);
                */
                // println!("V: {:?} {}", curr_value, curr_child);

                if curr_value > best_value {
                    best_value = curr_value;
                    curr_node = *child;
                }
            }


            let best_action = arena[curr_node].action.unwrap();

            // println!("\\__ {:?}", best_action);
            state.do_action(best_action);
        }
        // println!("After select: {:?}", curr_node);

        // Expand
        if arena[curr_node].untried_actions.len() > 0 {
            let num_actions = arena[curr_node].untried_actions.len();
            let action = arena[curr_node].untried_actions[random() % num_actions]; 
            state.do_action(action);

            // let new_nodeid = node.add_child(arena, Some(action), state.clone());

            // Create new child node for the current node
            let action_str = arena[curr_node].untried_actions.iter()
                            .position(|&n| n == action)
                            .map(|e| arena[curr_node].untried_action_strings[e].clone());

            let new_node = arena.new_child_node(Some(curr_node), Some(action), action_str, &state);

            {
                let parent_node = &mut arena[curr_node];

                // Remove action from current node list
                parent_node.untried_actions.iter()
                    .position(|&n| n == action)
                    .map(|e| {
                        parent_node.untried_actions.remove(e);
                        parent_node.untried_action_strings.remove(e);
                    });


                parent_node.children.push(new_node);
            }

            // println!("{}", arena.as_tree());
            // node = &mut arena[new_node];
            curr_node = new_node;
        }

        // Rollout
        loop {
            if state.get_actions().len() == 0 {
                break;
            }
            let num_actions = state.get_actions().len();
            let curr_move = state.get_actions()[random() % num_actions];
            state.do_action(curr_move); 
            // println!("Move: {:?} State: {:?}", curr_move, state);
        }

        // Backpropogate
        loop {
            // println!("curr_node: {:?} player: {:?}", curr_node, arena[curr_node].player_just_moved);
            let result = state.get_result(arena[curr_node].player_just_moved);
            {
                let node = &mut arena[curr_node];
                node.update(result);
                match node.parent {
                    Some(parent) => curr_node = parent,
                    None => break,
                }
            }
        }
    }

    let mut most_visits = 0 ;
    let mut best_action = None;
    // println!("{}", arena.as_tree());
    for child in &arena[rootnode].children {
        if arena[*child].visits > most_visits {
            best_action = Some(child);
            most_visits = arena[*child].visits;
        }
    }
    match best_action {
        Some(node) => arena[*node].action.unwrap(),
        None => panic!("No valid best action")
    }
}

fn main() {
    let arena = &mut NodeArena::new();
    // let mut state = NimState::new(10);
    let mut state = AgricolaState::new(2);
    println!("{}", state.board);

    /*
    println!("Before\n{}", state);
    // Round 1
    state.do_action(AgricolaAction::StartingPlayer_Food as u32);
    println!("{:?}", state.get_action_strings());
    state.do_action(AgricolaAction::Grain as u32);
    state.do_action(AgricolaAction::Wood as u32);
    state.do_action(AgricolaAction::Grain as u32);
    println!("{}", state);
    // Round 2
    state.do_action(AgricolaAction::Grain as u32);
    state.do_action(AgricolaAction::Grain as u32);
    state.do_action(AgricolaAction::Grain as u32);
    state.do_action(AgricolaAction::Grain as u32);
    println!("{}", state);
    */

    while state.clone().get_actions().len() > 0 {
        let best_action = UCT(arena, state.clone(), 1000);
        // println!("{}", arena.simple_display());
        // println!("Best action {:?}", best_action);
        state.do_action(best_action);
    }

    loop {
        let actions = state.get_actions();
        if (actions.len() == 0) {
            break;
        }

        state.do_action(actions[0]);
        println!("{:?}", actions);
    }
    // println!("{:?}", state);
    state.print_ending();
}
