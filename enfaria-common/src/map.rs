use serde::{Serialize, Deserialize};
use std::{
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
	Blocker,
	#[serde(rename = "G")]
	Grass,
	#[serde(rename = "F")]
	Field,
}

impl Display for Tile {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
        	Tile::Blocker => write!(fmt, "Blocker"),
        	Tile::Grass => write!(fmt, "Grass"),
        	Tile::Field => write!(fmt, "Field"),
        }
    }
}

impl From<String> for Tile {
    fn from(s: String) -> Self {
        match &s[..] {
        	"Blocker" => Tile::Blocker,
        	"Grass" => Tile::Grass,
        	"Field" => Tile::Field,
        	_ => unreachable!(),
        }
    }
}

impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        match s {
        	"Blocker" => Tile::Blocker,
        	"Grass" => Tile::Grass,
        	"Field" => Tile::Field,
        	_ => unreachable!(),
        }
    }
}

pub fn get_map(path: &str) -> Map {
	let s = read_to_string(path).unwrap();
	toml::from_str(&s).unwrap()
}

pub fn save_map(path: &str, map: &Map) {
	fs::write(path, toml::to_string(map).unwrap()).unwrap();
}
