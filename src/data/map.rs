use crate::components::Position;
use crate::data::Tile;
use egui::{pos2, vec2, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub tile_width: u32,
    pub tile_height: u32,
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn get_tile(&self, position: Position) -> Option<&Tile> {
        let mut x = 0;
        let mut y = 0;

        for row in self.tiles.iter() {
            for tile in row.iter() {
                let rect = Rect::from_min_size(pos2(x as f32, y as f32), vec2(32.0, 32.0));
                if rect.contains(pos2(position.x as f32, position.y as f32)) {
                    return Some(&tile);
                }
                x += self.tile_width;
            }
            x = 0;
            y += self.tile_height;
        }

        None
    }
}
