use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    pub description: String,
}

impl Name {
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.description.is_empty()
    }
}
