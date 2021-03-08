use crate::state::MainState;
use tetra::ContextBuilder;

mod components;
mod data;
mod egui;
mod input;
mod scenes;
mod state;
mod systems;
mod world;

pub fn get_assets_folder() -> String {
    "assets/".into()
}

fn main() -> tetra::Result {
    dotenv::dotenv().expect("Failed to setup dotenv.");
    env_logger::init();

    ContextBuilder::new("Enfaria", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(MainState::new)
}
