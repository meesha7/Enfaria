use hecs::*;

// Marker struct that an entity should be synced between the client and the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sync;

// Mode that dictates what defaults get loaded into the state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Client,
    Server,
}

#[derive(Default)]
pub struct State {
    ecs: World,
}

impl State {
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::Client => State::new_client(),
            Mode::Server => State::new_server(),
        }
    }

    fn new_client() -> Self {
        State { ecs: World::new() }
    }

    fn new_server() -> Self {
        State { ecs: World::new() }
    }

    pub fn ecs(&self) -> &World {
        &self.ecs
    }

    pub fn ecs_mut(&mut self) -> &mut World {
        &mut self.ecs
    }

    pub fn spawn_synced(&mut self) -> EntityBuilder {
        let mut builder = EntityBuilder::new();
        builder.add(Sync);
        builder
    }
}
