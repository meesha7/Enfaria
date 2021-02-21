use crate::egui::handle_ui;
use crate::scenes::{menu::MenuScene, SceneInput, SceneStack};
use crate::world::GameWorld;
use egui::CtxRef;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{clear, present, Color};
use ggez::timer::check_update_time;
use ggez::{Context, GameResult};

// Semi-mutable global state, handles switching scenes and keeps information about keybindings.
pub struct MainState {
    scenes: SceneStack,
    egui: CtxRef,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> Self {
        let world = GameWorld::new();
        let mut scenes = SceneStack::new(world);
        let menu = Box::new(MenuScene::new());
        scenes.push(menu);

        let egui = CtxRef::default();

        MainState { scenes, egui }
    }
}

impl EventHandler for MainState {
    // Update game logic to target 60 FPS.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while check_update_time(ctx, DESIRED_FPS) {
            self.scenes.update(ctx);
        }

        Ok(())
    }

    // Draw the current scene.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::from((0.8, 0.8, 1.0, 0.0)));
        self.scenes.draw(ctx);
        handle_ui(ctx, &mut self.egui);
        present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        self.scenes.input(ctx, SceneInput::KeyDown(keycode, keymods, repeat))
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.scenes.input(ctx, SceneInput::KeyUp(keycode, keymods))
    }
}
