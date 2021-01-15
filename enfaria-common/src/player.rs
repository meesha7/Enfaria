use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, read_to_string};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub position: Position,
    pub inventory: BTreeMap<u16, Item>,
}

impl Player {
    pub fn _in_hotbar(&self, item: &Item) -> bool {
        for (key, value) in self.inventory.iter() {
            if *key <= 4 && value == item {
                return true;
            }
        }
        false
    }
}

pub fn get_player(path: &str) -> Result<Player, Box<dyn Error>> {
    let s = read_to_string(path)?;
    serde_json::from_str(&s).map_err(|e| e.into())
}

pub fn save_player(path: &str, player: &Player) -> Result<(), Box<dyn Error>> {
    fs::write(path, serde_json::to_string_pretty(player)?)?;
    Ok(())
}
