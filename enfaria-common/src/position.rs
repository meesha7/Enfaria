use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize, FromVariant, ToVariant)]
pub struct Position {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl From<String> for Position {
    fn from(string: String) -> Self {
        string.into()
    }
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let split: Vec<u64> = s.split(' ').map(|x| gresult!(x.parse())).collect();
        Position {
            x: split[0],
            y: split[1],
            z: split[2],
        }
    }
}
