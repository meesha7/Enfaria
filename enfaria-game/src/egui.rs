use egui::paint::ClippedShape;
use egui::{ClippedMesh, CtxRef, Event, Modifiers, Output, Pos2, RawInput};
use egui::{PointerButton, Texture as ETexture, Vec2 as EVec2};
use tetra::graphics::mesh::{IndexBuffer, Vertex, VertexBuffer, VertexWinding};
use tetra::graphics::{Color, DrawParams, Texture};
use tetra::input::*;
use tetra::math::Vec2;
use tetra::Context;

pub fn prepare_ui(ctx: &mut Context, ectx: &mut CtxRef) {
    let raw_input = gather_input(ctx);
    ectx.begin_frame(raw_input);
}

pub fn end_ui_frame(ectx: &mut CtxRef) -> (Output, Vec<ClippedShape>) {
    ectx.end_frame()
}

pub fn render_ui(ctx: &mut Context, ectx: &mut CtxRef, shapes: Vec<ClippedShape>) {
    let texture = ectx.texture();
    let clipped_meshes = ectx.tessellate(shapes);
    paint(ctx, clipped_meshes, texture.as_ref());
}

pub fn gather_input(ctx: &mut Context) -> RawInput {
    let mut ri = RawInput::default();

    let mouse_wheel = {
        let v = get_mouse_wheel_movement(ctx);
        EVec2 {
            x: v[0] as f32,
            y: v[1] as f32,
        }
    };
    ri.scroll_delta = mouse_wheel;

    let mouse_pos: Pos2 = get_mouse_position(ctx).into_array().into();
    ri.events.push(Event::PointerMoved(mouse_pos));

    let mut modifiers = Modifiers::default();
    if is_key_down(ctx, Key::LeftCtrl) || is_key_down(ctx, Key::RightCtrl) {
        modifiers.ctrl = true;
        modifiers.command = true;
    };
    if is_key_down(ctx, Key::LeftShift) || is_key_down(ctx, Key::RightShift) {
        modifiers.shift = true;
    };
    if is_key_down(ctx, Key::LeftAlt) || is_key_down(ctx, Key::RightAlt) {
        modifiers.alt = true;
    };

    if modifiers.ctrl && is_key_down(ctx, Key::C) {
        ri.events.push(Event::Copy)
    }

    if modifiers.ctrl && is_key_down(ctx, Key::X) {
        ri.events.push(Event::Cut)
    }

    ri.modifiers = modifiers;

    if is_mouse_button_down(ctx, MouseButton::Left) {
        ri.events.push(Event::PointerButton {
            pos: mouse_pos,
            button: PointerButton::Primary,
            pressed: true,
            modifiers,
        })
    }

    if is_mouse_button_down(ctx, MouseButton::Right) {
        ri.events.push(Event::PointerButton {
            pos: mouse_pos,
            button: PointerButton::Secondary,
            pressed: true,
            modifiers,
        })
    }

    if is_mouse_button_down(ctx, MouseButton::Middle) {
        ri.events.push(Event::PointerButton {
            pos: mouse_pos,
            button: PointerButton::Middle,
            pressed: true,
            modifiers,
        })
    }

    if is_mouse_button_up(ctx, MouseButton::Left) {
        ri.events.push(Event::PointerButton {
            pos: mouse_pos,
            button: PointerButton::Primary,
            pressed: false,
            modifiers,
        })
    }

    if is_mouse_button_up(ctx, MouseButton::Right) {
        ri.events.push(Event::PointerButton {
            pos: mouse_pos,
            button: PointerButton::Secondary,
            pressed: false,
            modifiers,
        })
    }

    if is_mouse_button_up(ctx, MouseButton::Middle) {
        ri.events.push(Event::PointerButton {
            pos: mouse_pos,
            button: PointerButton::Middle,
            pressed: false,
            modifiers,
        })
    }

    for key in get_keys_pressed(ctx) {
        let key = match convert_key(key) {
            Some(k) => k,
            None => continue,
        };
        ri.events.push(Event::Key {
            key,
            pressed: true,
            modifiers,
        });
    }

    for key in get_keys_released(ctx) {
        let key = match convert_key(key) {
            Some(k) => k,
            None => continue,
        };
        ri.events.push(Event::Key {
            key,
            pressed: false,
            modifiers,
        });
    }

    if let Some(i) = get_text_input(ctx) {
        ri.events.push(Event::Text(i.to_owned()))
    }

    ri
}

pub fn paint(ctx: &mut Context, meshes: Vec<ClippedMesh>, texture: &ETexture) {
    for cm in meshes.into_iter() {
        let mut verts = vec![];

        // Convert egui::Vertex into tetra::Vertex
        for v in cm.1.vertices.into_iter() {
            let c = v.color.to_tuple();
            let vert = Vertex {
                position: Vec2::new(v.pos.x, v.pos.y),
                uv: Vec2::new(v.uv.x, v.uv.y),
                color: Color::rgba8(c.0, c.1, c.2, c.3),
            };
            verts.push(vert);
        }

        // Indices
        let index = IndexBuffer::new(ctx, &cm.1.indices).unwrap();
        // Vertices
        let buffer = VertexBuffer::new(ctx, &verts).unwrap();

        // Egui uses premultiplied alpha with white pixels.
        let alphas = &texture.pixels;
        let mut fixed = vec![];
        for x in alphas {
            fixed.push(255);
            fixed.push(255);
            fixed.push(255);
            fixed.push(*x);
        }

        let tex = Texture::from_rgba(ctx, texture.width as i32, texture.height as i32, &fixed).unwrap();
        let mut mesh = buffer.into_mesh();
        mesh.set_index_buffer(index);
        // This should most likely stay disabled.
        mesh.set_backface_culling(false);
        mesh.set_front_face_winding(VertexWinding::Clockwise);
        mesh.set_texture(tex);
        mesh.draw(ctx, DrawParams::default());
    }
}

// This is neccesary since egui has less Key types than tetra.
fn convert_key(key: &Key) -> Option<egui::Key> {
    match key {
        Key::A => Some(egui::Key::A),
        Key::B => Some(egui::Key::B),
        Key::C => Some(egui::Key::C),
        Key::D => Some(egui::Key::D),
        Key::E => Some(egui::Key::E),
        Key::F => Some(egui::Key::F),
        Key::G => Some(egui::Key::G),
        Key::H => Some(egui::Key::H),
        Key::I => Some(egui::Key::I),
        Key::J => Some(egui::Key::J),
        Key::K => Some(egui::Key::K),
        Key::L => Some(egui::Key::L),
        Key::M => Some(egui::Key::M),
        Key::N => Some(egui::Key::N),
        Key::O => Some(egui::Key::O),
        Key::P => Some(egui::Key::P),
        Key::Q => Some(egui::Key::Q),
        Key::R => Some(egui::Key::R),
        Key::S => Some(egui::Key::S),
        Key::T => Some(egui::Key::T),
        Key::U => Some(egui::Key::U),
        Key::V => Some(egui::Key::V),
        Key::W => Some(egui::Key::W),
        Key::X => Some(egui::Key::X),
        Key::Y => Some(egui::Key::Y),
        Key::Z => Some(egui::Key::Z),
        Key::Num0 => Some(egui::Key::Num0),
        Key::Num1 => Some(egui::Key::Num1),
        Key::Num2 => Some(egui::Key::Num2),
        Key::Num3 => Some(egui::Key::Num3),
        Key::Num4 => Some(egui::Key::Num4),
        Key::Num5 => Some(egui::Key::Num5),
        Key::Num6 => Some(egui::Key::Num6),
        Key::Num7 => Some(egui::Key::Num7),
        Key::Num8 => Some(egui::Key::Num8),
        Key::Num9 => Some(egui::Key::Num9),
        Key::Escape => Some(egui::Key::Escape),
        Key::Tab => Some(egui::Key::Tab),
        Key::Backspace => Some(egui::Key::Backspace),
        Key::Enter => Some(egui::Key::Enter),
        Key::Space => Some(egui::Key::Space),
        Key::Insert => Some(egui::Key::Insert),
        Key::Delete => Some(egui::Key::Delete),
        Key::Home => Some(egui::Key::Home),
        Key::End => Some(egui::Key::End),
        Key::PageDown => Some(egui::Key::PageDown),
        Key::PageUp => Some(egui::Key::PageUp),
        Key::Up => Some(egui::Key::ArrowUp),
        Key::Down => Some(egui::Key::ArrowDown),
        Key::Left => Some(egui::Key::ArrowLeft),
        Key::Right => Some(egui::Key::ArrowRight),
        _ => None,
    }
}
