use crate::components::{Player, Position, Texture};
use crate::data::Map;
use crate::get_assets_folder;
use crate::input::key_to_dir;
use crate::scenes::{Scene, SceneSwitch};
use crate::systems::{hover_system, movement_system, render_system};
use crate::world::GameWorld;
use egui::CtxRef;
use hecs::EntityBuilder;
use std::fs::read_to_string;
use tetra::input::*;
use tetra::{Context, Event};

#[derive(Debug)]
pub struct GameScene {
    pub map: Map,
}

impl GameScene {
    pub fn new(world: &mut GameWorld, _ctx: &mut Context) -> Self {
        let assets_folder = get_assets_folder();

        let mut map: Map =
            ron::from_str(&read_to_string(&format!("{}/map.ron", &assets_folder)).expect("Failed to read map file."))
                .expect("Failed to deserialize map file.");

        let width = map.tiles[0].len() * 32;
        let height = map.tiles.len() * 32;

        let mut x = 0;
        let mut y = 0;
        for row in map.tiles.iter_mut() {
            for mut tile in row.iter_mut() {
                let mut builder = EntityBuilder::new();
                builder.add(Position { x, y, z: 0 }).add(Texture {
                    texture: tile.get_texture(),
                });
                if tile.name.is_empty() {
                    tile.name = tile.kind.get_name()
                }
                builder.add(tile.name.clone());
                world.ecs.spawn(builder.build());
                x += map.tile_width;
            }
            x = 0;
            y += map.tile_height;
        }

        world.ecs.spawn((
            Position {
                x: (width / 2) as u32 - 16,
                y: (height / 2) as u32 - 16,
                z: 1,
            },
            Player,
            Texture {
                texture: format!("{}/player.png", &assets_folder),
            },
        ));

        GameScene { map }
    }
}

impl Scene for GameScene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        for key in get_keys_down(ctx) {
            if let Some(md) = key_to_dir(key) {
                movement_system(world, &self.map, md)
            }
        }
        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        render_system(ctx, world);
        hover_system(ctx, ectx, world);
        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _event: Event) -> tetra::Result {
        Ok(())
    }
}
