use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
	Hoe(Option<ItemData>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemData(HashMap<String, String>);
