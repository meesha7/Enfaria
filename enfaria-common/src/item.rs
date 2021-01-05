use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Hoe(Option<ItemData>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemData(HashMap<String, String>);
