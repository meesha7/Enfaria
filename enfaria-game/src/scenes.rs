use crate::world::GameWorld;
use ggez::event::{KeyCode, KeyMods};
use ggez::{Context, GameResult};

pub mod game;
pub mod menu;

pub struct SceneStack {
    pub world: GameWorld,
    scenes: Vec<Box<dyn Scene>>,
}

impl SceneStack {
    pub fn new(world: GameWorld) -> Self {
        Self {
            world,
            scenes: Vec::new(),
        }
    }

    pub fn push(&mut self, scene: Box<dyn Scene>) {
        self.scenes.push(scene)
    }

    pub fn pop(&mut self) -> Box<dyn Scene> {
        self.scenes.pop().expect("Tried to pop an empty scene stack.")
    }

    pub fn switch(&mut self, next_scene: SceneSwitch) -> Option<Box<dyn Scene>> {
        match next_scene {
            SceneSwitch::None => None,
            SceneSwitch::Pop => {
                let s = self.pop();
                Some(s)
            }
            SceneSwitch::Push(s) => {
                self.push(s);
                None
            }
            SceneSwitch::Replace(s) => {
                let old_scene = self.pop();
                self.push(s);
                Some(old_scene)
            }
        }
    }

    fn draw_scenes(scenes: &mut [Box<dyn Scene>], world: &mut GameWorld, ctx: &mut Context) {
        assert!(!scenes.is_empty());
        if let Some((current, rest)) = scenes.split_last_mut() {
            if current.draw_previous() {
                SceneStack::draw_scenes(rest, world, ctx);
            }
            current.draw(world, ctx).expect("Failed to draw a scene.");
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) {
        SceneStack::draw_scenes(&mut self.scenes, &mut self.world, ctx)
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let next_scene = {
            let current_scene = &mut **self.scenes.last_mut().expect("Tried to update empty scene stack.");
            current_scene.update(&mut self.world, ctx)
        };
        self.switch(next_scene);
    }

    pub fn input(&mut self, ctx: &mut Context, input: SceneInput) {
        let current_scene = &mut **self.scenes.last_mut().expect("Tried to do input for empty scene stack");
        current_scene.input(&mut self.world, ctx, input);
    }
}

pub trait Scene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> SceneSwitch;
    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context) -> GameResult<()>;
    fn input(&mut self, world: &mut GameWorld, ctx: &mut Context, input: SceneInput);
    fn draw_previous(&self) -> bool {
        false
    }
}

pub enum SceneSwitch {
    None,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
}

pub enum SceneInput {
    KeyDown(KeyCode, KeyMods, bool),
    KeyUp(KeyCode, KeyMods),
}
