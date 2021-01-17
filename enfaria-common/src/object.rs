use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromVariant, ToVariant)]
pub struct Object {
    pub name: String,
    #[serde(default)]
    pub data: Vec<String>,
}

impl Display for Object {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "{}", self.name)
    }
}
