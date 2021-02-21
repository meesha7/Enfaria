use egui::{Area, ClippedMesh, CtxRef, Output, RawInput, Texture as ETexture, Window};
use ggez::graphics::{draw, window, DrawParam, Image, Mesh, Vertex};
use ggez::Context;

pub fn handle_ui(ctx: &mut Context, ectx: &mut CtxRef) {
    let raw_input = gather_input(ctx);
    ectx.begin_frame(raw_input);
    let texture = ectx.texture();

    Window::new("Title!").show(&ectx, |ui| {
        ui.add(egui::Label::new("Hello World!"));
        ui.label("A shorter and more convenient way to add a label.");
        ui.horizontal(|ui| {
            ui.label("Add widgets");
            if ui.button("on the same row!").clicked() { /* … */ }
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
                ui.checkbox(&mut false, "animate");
                ui.advance_cursor(8.0);
                ui.checkbox(&mut true, "square view");
                ui.checkbox(&mut true, "proportional data axes");
            });
        });
    });

    let (output, shapes) = ectx.end_frame();
    let clipped_meshes = ectx.tessellate(shapes);
    handle_output(output);
    paint(ctx, clipped_meshes, texture.as_ref());
}

pub fn gather_input(ctx: &mut Context) -> RawInput {
    let scale_factor = window(ctx).scale_factor();

    RawInput {
        pixels_per_point: Some(scale_factor as f32),
        ..Default::default()
    }
}

pub fn handle_output(_output: Output) {}

pub fn paint(ctx: &mut Context, meshes: Vec<ClippedMesh>, texture: &ETexture) {
    for cm in meshes.into_iter() {
        let _clip = cm.0;
        let m = cm.1;

        let mut verts = vec![];
        for v in m.vertices.into_iter() {
            let c = v.color.to_tuple();
            let vert = Vertex {
                pos: [v.pos.x, v.pos.y],
                uv: [v.uv.x, v.uv.y],
                color: [c.0 as f32, c.1 as f32, c.2 as f32, c.3 as f32],
            };
            verts.push(vert);
        }

        let mut fixed = vec![];
        for x in texture.pixels.iter() {
            fixed.push(255);
            fixed.push(255);
            fixed.push(255);
            fixed.push(*x);
        }

        let tex = Image::from_rgba8(ctx, texture.width as u16, texture.height as u16, &fixed).unwrap();
        let mesh = Mesh::from_raw(ctx, &verts, &m.indices, Some(tex)).unwrap();
        draw(ctx, &mesh, DrawParam::default()).unwrap()
    }
}
