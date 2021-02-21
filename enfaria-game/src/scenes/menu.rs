use crate::scenes::{Scene, SceneInput, SceneSwitch};
use crate::world::GameWorld;
use ggez::Context;

pub struct MenuScene;

impl MenuScene {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        MenuScene
    }
}

impl Scene for MenuScene {
    fn update(&mut self, _world: &mut GameWorld, _ctx: &mut Context) -> SceneSwitch {
        SceneSwitch::None
    }

    fn draw(&mut self, _world: &mut GameWorld, _ctx: &mut Context) -> ggez::GameResult<()> {
        Ok(())
    }

    fn input(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _input: SceneInput) {}
}
