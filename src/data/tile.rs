use crate::components::Name;
use crate::get_assets_folder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pub kind: TileKind,
    #[serde(default)]
    pub name: Name,
}

impl Default for Tile {
    fn default() -> Self {
        let kind = TileKind::Blocker;
        let name = kind.get_name();
        Tile { kind, name }
    }
}

impl Tile {
    pub fn get_texture(&self) -> String {
        self.kind.get_texture()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileKind {
    Blocker,
    Grass,
}

impl TileKind {
    pub fn get_texture(&self) -> String {
        let mut path = get_assets_folder();

        use TileKind::*;
        match self {
            Blocker => path += "blocker.png",
            Grass => path += "grass.png",
        };

        path
    }

    pub fn get_name(&self) -> Name {
        let name;
        let description;

        use TileKind::*;
        match self {
            Blocker => {
                name = "Blocker".into();
                description = "You shouldn't go past this.".into();
            }
            Grass => {
                name = "Grass".into();
                description = "Plants grow here.".into();
            }
        }

        Name { name, description }
    }
}
