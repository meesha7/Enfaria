use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, read_to_string};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    position: Position,
    inventory: HashMap<u16, Item>,
}

pub fn get_player(path: &str) -> Player {
    let s = read_to_string(path).unwrap();
    toml::from_str(&s).unwrap()
}

pub fn save_player(path: &str, player: &Player) {
    fs::write(path, toml::to_string(player).unwrap()).unwrap();
}
