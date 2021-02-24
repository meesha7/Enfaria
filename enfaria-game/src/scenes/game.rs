use crate::scenes::{Scene, SceneSwitch};
use crate::world::GameWorld;
use egui::CtxRef;
use enfaria_common::*;
use tetra::{Context, Event};

#[derive(Debug, Default)]
pub struct GameScene;

impl GameScene {
    pub fn new(_world: &mut GameWorld, _ctx: &mut Context) -> Self {
        GameScene
    }
}

impl Scene for GameScene {
    fn update(&mut self, world: &mut GameWorld, _ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        world.send_packet(Message::Ping);
        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _ectx: &mut CtxRef) -> tetra::Result {
        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _event: Event) -> tetra::Result {
        Ok(())
    }
}
