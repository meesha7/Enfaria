use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tile")]
pub struct Tile {
    pub name: String,
    #[serde(default)]
    pub data: HashMap<String, String>,
}

impl Display for Tile {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.name)
    }
}

impl From<String> for Tile {
    fn from(s: String) -> Self {
        s.into()
    }
}

impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        Tile {
            name: s.to_string(),
            data: HashMap::new(),
        }
    }
}
