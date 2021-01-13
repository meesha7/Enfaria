use crate::{position::Position, tile::Tile};
use serde::{Deserialize, Serialize};
use std::fs::{self, read_to_string};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: u64,
    pub height: u64,
}

impl Map {
    pub fn can_visit(&self, pos: Position) -> bool {
        let tile = self.get_tile(pos);
        if tile.name == "Blocker" {
            return false;
        };

        if tile.data.get("blocker").is_some() {
            return true;
        };

        true
    }

    pub fn get_tile(&self, pos: Position) -> &Tile {
        let x = (pos.x / 32) as usize;
        let y = (pos.y / 32) as usize;
        &self.tiles[x][y]
    }
}

pub fn get_map(path: &str) -> Map {
    let s = read_to_string(path).unwrap();
    serde_json::from_str(&s).unwrap()
}

pub fn save_map(path: &str, map: &Map) {
    fs::write(path, serde_json::to_string_pretty(map).unwrap()).unwrap();
}
