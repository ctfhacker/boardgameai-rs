extern crate boardgameai_rs;
extern crate nim;
extern crate agricola;
extern crate rand;
extern crate colored;

use boardgameai_rs::state::State;
use boardgameai_rs::node::{NodeArena, NodeId};
use nim::NimState;
use agricola::AgricolaState;
use agricola::AgricolaAction;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use std::io::{self, BufRead};
use std::collections::HashSet;

use colored::*;

fn UCT<S: State+Clone+Debug>(arena: &mut NodeArena, rootstate: S, seconds: u64) -> u32 {
    let rootnode = arena.new_node(rootstate.clone());

    let begin_time = Instant::now();
    let time_to_think = Duration::from_secs(seconds);

    let mut counter = 0;
    while begin_time.elapsed() < time_to_think {
        if counter % 100 == 0 {
            println!("Counter: {}", counter);
        }
        let mut curr_node = rootnode;

        //let mut node = &arena[rootnode].clone();
        let node: NodeId;

        let mut state = rootstate.clone();

        // println!("Before select: {:?}", curr_node);
        // Select
        // Find a full expanded, non-terminal node to start this state
        loop {
            if(arena[curr_node].untried_actions.len() != 0 || 
               arena[curr_node].children.len() == 0){
                //let rootnode_tmp = arena.new_node(rootstate.clone());
                //arena[curr_node].untried_actions = arena[rootnode_tmp].clone().untried_actions;
                //arena[curr_node].untried_action_strings = arena[rootnode_tmp].clone().untried_action_strings;

                //println!("num_actions: {:?}", num_actions);
                //println!("untried curr: {:?}", arena[curr_node].untried_action_strings);
                //println!("untried root: {:?}", arena[rootnode_tmp].untried_action_strings);

                break;
            }

            let UCTK: f64 = 1.4;
            let mut best_value = (-1 as f64);

            for child in &arena[curr_node].children {
                let curr_child = &arena[*child];
                let curr_value = (curr_child.wins as f64)/(curr_child.visits as f64) + UCTK * ((((arena[curr_node].visits as f64).ln() * 2.0) / (curr_child.visits as f64)).sqrt());

                // if curr_value > best_value && arena[rootnode].untried_actions.contains(&arena[*child].action.expect("can't get action for curr_child")){
                if curr_value > best_value {
                    //let mut tmp_state= state.clone();
                    //tmp_state.do_action(arena[*child].action.unwrap());
                    //let mut a = tmp_state.get_actions().clone();
                    //a.sort();
                    //let mut b = arena[*child].untried_actions.clone();
                    //b.sort();
                    //if arena[*child].untried_actions.iter().all(|a| tmp_state.get_actions().contains(&a)) {
                    best_value = curr_value;
                    curr_node = *child;
                    //}
                }
            }

            let best_action = arena[curr_node].action.unwrap();

            // println!("\\__ Select {:?}", best_action);
            state.do_action(best_action);
        }

        // println!("After select: {:?}", arena[curr_node]);
        // println!("{}", arena.as_tree());

        let num_actions = arena[curr_node].untried_actions.len();
        // Expand
        if num_actions > 0 {

            let action = loop {
                let num_actions = arena[curr_node].untried_actions.len();

                let action = arena[curr_node].untried_actions[rand::random::<usize>() % num_actions]; 

                // Possible state change did not come to fruition, delete it from untried (since it
                // is not possible and continue looking
                /*
                if !arena[rootnode_tmp].untried_actions.contains(&action) {
                    // println!("Action is not possible from this game state.. removing..");
                    arena[curr_node].untried_actions.iter()
                        .position(|&n| n == action)
                        .map(|e| {
                            arena[curr_node].untried_actions.remove(e);
                            arena[curr_node].untried_action_strings.remove(e);
                        });
                }

                // Found a node that can be done in this game state
                else {
                    break action;
                }
                */
                // break action;

                // if arena[rootnode].untried_actions.contains(&action) {
                if state.get_actions().contains(&action) {
                    break action;
                }
            };

            // let new_nodeid = node.add_child(arena, Some(action), state.clone());
            //println!("EXPAND untried actions: {:?}", arena[curr_node].untried_actions);
            //println!("EXPAND untried action string: {:?}", arena[curr_node].untried_action_strings);
            state.do_action(action);

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
            let curr_move = state.get_actions()[rand::random::<usize>() % num_actions];

            // println!("\\__ Rollout {:?}", curr_move);
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
        counter += 1
    }

    println!("{}", format!("{} iterations in {} seconds", counter, seconds).blue());

    // println!("{}", arena.as_tree());
    // let mut actions_cant_perform = vec!();

    let mut most_visits = 0 ;
    let mut best_action = None;

    for child in &arena[rootnode].children {
        println!("{:?}: {:?}", child, arena[*child].visits);
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

fn human_action<S: State+Clone+Display>(state: S) -> u32 {

    println!("Current State: {}", state);
    let possible_actions = state.get_actions();
    for (i, action) in possible_actions.iter().enumerate() {
        println!("[{}] {:?}", i, AgricolaAction::from_u32(*action).unwrap());
    }
    println!("Enter your action: ");

    let stdin = io::stdin();
    
    let choice = loop {
        let mut choice = String::new();
        stdin.lock().read_line(&mut choice).expect("Failed to read stdin choice..");
        match choice.trim().parse::<usize>() {
            Ok(choice) => { break choice },
            _ => { println!("Error reading input.. Try again.."); continue; }
        }
    };

    let their_choice = possible_actions.iter().nth(choice).unwrap();
    println!("Your choice: {:?} -> {:?}", their_choice, AgricolaAction::from_u32(*their_choice));
    *their_choice
}


fn main() {
    let arena = &mut NodeArena::new();
    let AI_PLAYER = 0; // 0 - first, 1 - second
    // let mut state = NimState::new(10);
    //
    let mut state = AgricolaState::new(2);

    while state.clone().get_actions().len() > 0 {

        let now = Instant::now();
        let best_action;
        let iterations = 0;
        if state.current_player == AI_PLAYER {
            // First player is "dumb" with less iterations
            let seconds = 10;
            best_action = UCT(arena, state.clone(), seconds);
            println!("{}", format!("AI chose.. {:?}", AgricolaAction::from_u32(best_action).unwrap()).red().bold());
        } else {
            // "smart" players
            /*
            let seconds = 1;
            // println!("UCT BEGIN tiles: {:?}", state.board.tiles);
            best_action = UCT(arena_2, state.clone(), seconds);
            */

            // let num_actions_taken = state.players[AI_PLAYER].actions_taken.len();
            best_action = human_action(state.clone());
        }

        /*
        println!("[{:?}] Best action [P: {}] {:?}", now.elapsed().as_secs(), state.current_player, AgricolaAction::from_u32(best_action));
        */
        let old_tiles = state.clone().board.tiles;

        state.do_action(best_action);

        let new_tiles = state.clone().board.tiles;
        if new_tiles.len() > old_tiles.len() {
            let new_set: HashSet<_> = new_tiles.iter().collect();
            let old_set: HashSet<_> = old_tiles.iter().collect();
            println!("{}", format!("{:?}", new_set.difference(&old_set)).green());
        }
    }

    state.print_ending();
}
