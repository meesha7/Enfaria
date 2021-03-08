use crate::components::{Position, Texture};
use crate::world::GameWorld;
use tetra::graphics::{DrawParams, Texture as TetraTexture};
use tetra::Context;

pub fn render_system(ctx: &mut Context, world: &mut GameWorld) {
    for (_id, (position, texture)) in world.ecs.query::<(&Position, &Texture)>().iter() {
        if world.textures.get(&texture.texture).is_none() {
            let tex = TetraTexture::new(ctx, &texture.texture).expect("Failed to create texture.");
            world.textures.insert(texture.texture.clone(), tex);
        };

        let tex = world.textures.get(&texture.texture).unwrap().clone();
        let pos = (position.x as f32, position.y as f32).into();
        tex.draw(ctx, DrawParams::default().position(pos))
    }
}
