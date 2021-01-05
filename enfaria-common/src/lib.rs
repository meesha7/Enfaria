use gdnative::prelude::*;

pub use command::Command;
pub use item::{Item, ItemData};
pub use map::{get_map, save_map, Map, Tile, TileData};
pub use packet::Packet;
pub use player::{get_player, save_player, Player};
pub use position::Position;

pub mod command;
pub mod item;
pub mod map;
pub mod packet;
pub mod player;
pub mod position;

fn init(handle: InitHandle) {
    handle.add_class::<Packet>();
}

godot_init!(init);
