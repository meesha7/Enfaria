use tetra::graphics::Texture;

#[derive(Debug, Clone, PartialEq)]
pub struct Drawable {
    pub texture: Texture,
}

// SAFETY: You promise to not use this component in a multi-threaded context.
unsafe impl Send for Drawable {}
unsafe impl Sync for Drawable {}
