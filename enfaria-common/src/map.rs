use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    fmt::{Formatter, Display},
    fs::{self, read_to_string}
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: u64,
    pub height: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    #[serde(rename = "B")]
    Blocker(Option<TileData>),
    #[serde(rename = "G")]
    Grass(Option<TileData>),
    #[serde(rename = "F")]
    Field(Option<TileData>),
}

impl Display for Tile {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Blocker(_) => write!(fmt, "Blocker"),
            Tile::Grass(_) => write!(fmt, "Grass"),
            Tile::Field(_) => write!(fmt, "Field"),
        }
    }
}

impl From<String> for Tile {
    fn from(s: String) -> Self {
        s.into()
    }
}

impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        match s {
            "Blocker" => Tile::Blocker(None),
            "Grass" => Tile::Grass(None),
            "Field" => Tile::Field(None),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TileData(HashMap<String, String>);

pub fn get_map(path: &str) -> Map {
    let s = read_to_string(path).unwrap();
    toml::from_str(&s).unwrap()
}

pub fn save_map(path: &str, map: &Map) {
    fs::write(path, toml::to_string(map).unwrap()).unwrap();
}
