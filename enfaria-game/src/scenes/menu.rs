use crate::scenes::{Scene, SceneSwitch};
use crate::world::GameWorld;
use egui::*;
use tetra::{Context, Event};

pub struct MenuScene {
    checkbox_one: bool,
    checkbox_two: bool,
    checkbox_three: bool,
}

impl MenuScene {
    #[allow(clippy::clippy::new_without_default)]
    pub fn new() -> Self {
        MenuScene {
            checkbox_one: false,
            checkbox_two: false,
            checkbox_three: false,
        }
    }
}

impl Scene for MenuScene {
    fn update(&mut self, _world: &mut GameWorld, _ctx: &mut Context) -> SceneSwitch {
        SceneSwitch::None
    }

    fn draw(&mut self, _world: &mut GameWorld, _ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        Window::new("Title!").show(&ectx, |ui| {
            ui.add(egui::Label::new("Hello World!"));
            ui.label("A shorter and more convenient way to add a label.");
            ui.horizontal(|ui| {
                ui.label("Add widgets");
                if ui.button("on the same row!").clicked() {
                    println!("You clicked me!");
                }
            });
        });

        Area::new("Area").fixed_pos([250.0, 250.0]).show(&ectx, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Circle:");
                        ui.add(
                            egui::Slider::f32(&mut 5.0, 1e-4..=1e4)
                                .logarithmic(true)
                                .smallest_positive(1e-2)
                                .text("radius"),
                        );
                        ui.add(
                            egui::Slider::f32(&mut 2.0, -1e4..=1e4)
                                .logarithmic(true)
                                .smallest_positive(1e-2)
                                .text("center x"),
                        );
                        ui.add(
                            egui::Slider::f32(&mut 4.0, -1e4..=1e4)
                                .logarithmic(true)
                                .smallest_positive(1e-2)
                                .text("center y"),
                        );
                    });
                });

                ui.vertical(|ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.checkbox(&mut self.checkbox_one, "animate");
                    ui.advance_cursor(8.0);
                    ui.checkbox(&mut self.checkbox_two, "square view");
                    ui.checkbox(&mut self.checkbox_three, "proportional data axes");
                });
            });
        });

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _event: Event) {}
}
