use crate::components::*;
use hecs::*;
use tetra::graphics::DrawParams;
use tetra::Context;

pub fn render(ctx: &mut Context, world: &World) {
    for (_id, (drawable, position)) in world.query::<(&Drawable, &Position)>().iter() {
        drawable.texture.draw(
            ctx,
            DrawParams::default().position([position.x as f32, position.y as f32].into()),
        )
    }
}
