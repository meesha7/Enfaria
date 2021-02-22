use crate::state::MainState;
use tetra::ContextBuilder;

pub mod egui;
pub mod scenes;
pub mod state;
pub mod world;

fn main() -> tetra::Result {
    dotenv::dotenv().expect("Failed to setup dotenv.");
    env_logger::init();

    ContextBuilder::new("Enfaria", 1280, 720).build()?.run(MainState::new)
}
