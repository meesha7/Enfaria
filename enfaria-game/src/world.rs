use enfaria_common::{Mode, State};

// Mutable global state, used in every scene.
#[derive(Default)]
pub struct GameWorld {
    pub state: State,
}

impl GameWorld {
    pub fn new() -> Self {
        let state = State::new(Mode::Client);

        GameWorld { state }
    }
}
