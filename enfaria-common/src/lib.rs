use gdnative::prelude::*;

pub use command::*;
pub use item::Item;
pub use map::{get_map, save_map, Map};
pub use packet::Packet;
pub use player::{get_player, save_player, Player};
pub use position::Position;
pub use tile::Tile;

pub mod command;
pub mod item;
pub mod map;
pub mod packet;
pub mod player;
pub mod position;
pub mod tile;

#[macro_export]
macro_rules! gresult {
    ($e:expr) => {
        match $e {
            Ok(k) => k,
            Err(e) => {
                godot_error!("Rust Error: {:?}", e);
                panic!();
            }
        }
    };
}

#[macro_export]
macro_rules! goption {
    ($e:expr) => {
        match $e {
            Some(s) => s,
            None => {
                godot_error!("Rust None");
                panic!();
            }
        }
    };
}

fn init(handle: InitHandle) {
    handle.add_class::<Packet>();
}

godot_init!(init);
