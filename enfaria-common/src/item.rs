use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    #[serde(default)]
    pub data: HashMap<String, String>,
}

impl Display for Item {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "{}", self.name)
    }
}

impl From<String> for Item {
    fn from(s: String) -> Self {
        s.into()
    }
}

impl From<&str> for Item {
    fn from(s: &str) -> Self {
        Item {
            name: s.to_string(),
            data: HashMap::new(),
        }
    }
}
