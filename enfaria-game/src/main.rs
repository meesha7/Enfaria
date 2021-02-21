use crate::state::MainState;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::run;
use ggez::ContextBuilder;
use std::path::PathBuf;

pub mod egui;
pub mod scenes;
pub mod state;
pub mod world;

fn main() {
    dotenv::dotenv().expect("Failed to setup dotenv.");
    env_logger::init();

    let resource_dir = PathBuf::from("./assets");

    let builder = ContextBuilder::new("Enfaria", "WitchyCat")
        .window_setup(WindowSetup::default().title("Enfaria"))
        .window_mode(WindowMode::default())
        .add_resource_path(&resource_dir);

    let (mut context, event) = builder.build().unwrap();

    // Run the game.
    let state = MainState::new(&mut context);
    run(context, event, state);
}
