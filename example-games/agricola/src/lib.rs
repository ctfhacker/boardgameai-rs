#![feature(exclusive_range_pattern)]

#[macro_use]
extern crate lazy_static;
extern crate boardgameai_rs;
extern crate rand;

use boardgameai_rs::*;
use boardgameai_rs::state::State;
use boardgameai_rs::action::Action;
// use std::collections::{HashMap, HashSet};
use std::fmt::Display;
// use std::rand::{Rng, thread_rng};

pub mod player;
pub mod playermat;
pub mod farmtile;
pub mod fieldtile;
pub mod pasture;
pub mod agricolaaction;
pub mod agricolastate;
pub mod board;
pub mod misc;
pub mod majorimprovement;

pub use player::*;
pub use playermat::*;
pub use farmtile::*;
pub use fieldtile::*;
pub use pasture::*;
pub use agricolaaction::*;
pub use agricolastate::*;
pub use board::*;
pub use misc::*;
pub use pasture::*;
pub use majorimprovement::*;
