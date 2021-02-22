use crate::egui::{end_ui_frame, prepare_ui, render_ui};
use crate::scenes::{menu::MenuScene, SceneStack, Scenes};
use crate::world::GameWorld;
use egui::CtxRef;
use tetra::graphics::{clear, Color};
use tetra::window::set_mouse_visible;
use tetra::{Context, Event, State};

// Semi-mutable global state, handles switching scenes.
pub struct MainState {
    scenes: SceneStack,
    egui: CtxRef,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        set_mouse_visible(ctx, true)?;

        let scenes = SceneStack::new(GameWorld::new(), Scenes::Menu(MenuScene::new()));
        let egui = CtxRef::default();

        Ok(MainState { scenes, egui })
    }
}

impl State for MainState {
    // Draw the current scene.
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        clear(ctx, Color::rgb(0.094, 0.11, 0.16));
        prepare_ui(ctx, &mut self.egui);
        self.scenes.draw(ctx, &mut self.egui);
        let (output, shapes) = end_ui_frame(&mut self.egui);
        if output.needs_repaint {
            render_ui(ctx, &mut self.egui, shapes);
        };
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.scenes.update(ctx);
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        self.scenes.event(ctx, event);
        Ok(())
    }
}
