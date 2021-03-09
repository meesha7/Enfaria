use crate::scenes::{Scene, SceneSwitch};
use crate::world::GameWorld;
use egui::*;
use log::info;
use serde::ser::{Serialize, Serializer};
use serde_derive::Serialize as SerializeMacro;
use tetra::{Context, Event};

// Resolution options
#[derive(Debug, PartialEq, Clone, Copy)]
enum Resolution {
    Res1080,
    Res768,
    Res600,
}

// Choose default resolution unless it's provided by config
impl Default for Resolution {
    fn default() -> Self {
        Resolution::Res1080
    }
}

// Serializing Resolution into the config file
impl Serialize for Resolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Resolution::Res1080 => serializer.serialize_unit_variant("resolution", 0, "1920x1080"),
            Resolution::Res768 => serializer.serialize_unit_variant("resolution", 1, "1024x768"),
            Resolution::Res600 => serializer.serialize_unit_variant("resolution", 2, "800x600"),
        }
    }
}

// Our options scene
#[derive(Debug, Default)]
pub struct OptionsScene {
    back_clicked: bool,
    resolution_selected: Resolution,
    fullscreen: bool,
}

impl OptionsScene {
    pub fn new(_world: &mut GameWorld, _ctx: &mut Context) -> Self {
        OptionsScene::default()
    }
}

// Config struct to pass scene options into the config saving function
#[derive(Debug, SerializeMacro)]
struct Config {
    resolution: Resolution,
    fullscreen: bool,
}

// Saves config into file
fn save_config(config: Config) {
    info!("Saving config");

    let toml = toml::to_string(&config).unwrap();
    info!("{:?}", toml);

    // TODO: Implement saving
}

impl Scene for OptionsScene {
    fn update(&mut self, _world: &mut GameWorld, _ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.back_clicked {
            return Ok(SceneSwitch::Pop);
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, _world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        let size = tetra::window::get_size(ctx);
        let rect = Rect::from_center_size(pos2((size.0 / 2) as f32, (size.1 / 2) as f32), vec2(200.0, 200.0));
        Window::new("Options")
            .resize(|r| r.with_stroke(true))
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .fixed_rect(rect)
            .show(ectx, |ui| {
                // TODO: Label is on the right side of the combobox, we want it on the left
                egui::combo_box_with_label(ui, "Resolution:", format!("{:?}", self.resolution_selected), |ui| {
                    ui.selectable_value(&mut self.resolution_selected, Resolution::Res1080, "1920x1080");
                    ui.selectable_value(&mut self.resolution_selected, Resolution::Res768, "1024x768");
                    ui.selectable_value(&mut self.resolution_selected, Resolution::Res600, "800x600");
                });

                ui.checkbox(&mut self.fullscreen, "Fullscreen");

                ui.vertical_centered_justified(|ui| {
                    let back = ui.add(Button::new("Back"));

                    if back.clicked() {
                        info!("Clicked back");

                        let config = Config {
                            resolution: self.resolution_selected,
                            fullscreen: self.fullscreen,
                        };

                        save_config(config);
                        self.back_clicked = true;
                    }
                });
            });

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _event: Event) -> tetra::Result {
        Ok(())
    }
}
