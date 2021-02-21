use config::Config;
use enfaria_common::{Mode, State};
use hecs::World;

// Mutable global state, used in every scene.
#[derive(Default)]
pub struct GameWorld {
    pub input: State,
    pub config: Config,
    pub session_id: String,
    pub ecs: World,
}

impl GameWorld {
    pub fn new() -> Self {
        let input = State::new(Mode::Client);

        let mut config = Config::new();
        config
            .merge(config::File::with_name("config"))
            .expect("Failed to read config file.");

        let session_id = String::from("");

        let ecs = World::new();

        GameWorld {
            input,
            config,
            session_id,
            ecs,
        }
    }
}
