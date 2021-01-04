use crate::{Position, item::Item};
use serde::{Serialize, Deserialize};
use std::fs::{self, read_to_string};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
	position: Position,
	inventory: HashMap<u16, Item>,
}

pub fn get_player(path: &str) -> Player {
	let s = read_to_string(path).unwrap();
	toml::from_str(&s).unwrap()
}

pub fn save_player(path: &str, map: &Player) {
	fs::write(path, toml::to_string(map).unwrap()).unwrap();
}
