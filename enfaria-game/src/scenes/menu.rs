use crate::networking::start_networking;
use crate::scenes::{GameScene, Scene, SceneSwitch, Scenes};
use crate::world::GameWorld;
use crossbeam_channel::*;
use egui::*;
use enfaria_common::*;
use std::net::TcpStream;
use std::time::Duration;
use tetra::{Context, Event};

#[derive(Debug, Default)]
pub struct MenuScene {
    pub username: String,
    pub password: String,
    pub logged_in: bool,
}

impl MenuScene {
    pub fn new(_world: &mut GameWorld, _ctx: &mut Context) -> Self {
        MenuScene {
            username: String::new(),
            password: String::new(),
            logged_in: false,
        }
    }
}

impl Scene for MenuScene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.logged_in {
            let scene = GameScene::new(world, ctx);
            return Ok(SceneSwitch::Push(Scenes::Game(scene)));
        }
        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        let size = tetra::window::get_size(ctx);
        Window::new("Login")
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .fixed_pos([(size.0 / 2 - 100) as f32, (size.1 / 2 - 100) as f32])
            .fixed_size([200.0, 200.0])
            .show(ectx, |ui| {
                ui.add(Label::new("Username"));
                ui.add(TextEdit::singleline(&mut self.username));

                ui.add(Label::new("Password"));
                ui.add(TextEdit::singleline(&mut self.password));

                ui.vertical_centered(|ui| {
                    let login = ui.add(Button::new("Login"));
                    if login.clicked() {
                        let url = format!("{}/api/login", std::env::var("DOMAIN").unwrap());
                        let response = ureq::post(&url)
                            .send_form(&[("username", &self.username), ("password", &self.password)])
                            .unwrap();

                        let token = response.into_string().unwrap();
                        world.session_id = token;

                        let stream = TcpStream::connect("127.0.0.1:8888").unwrap();
                        stream.set_nodelay(true).unwrap();
                        stream.set_nonblocking(true).unwrap();
                        stream.set_read_timeout(Some(Duration::from_millis(50))).unwrap();
                        stream.set_write_timeout(Some(Duration::from_millis(50))).unwrap();

                        let (s1, r1) = unbounded::<Packet>();
                        let (s2, r2) = unbounded::<Packet>();
                        world.sender = Some(s1);
                        world.receiver = Some(r2);

                        std::thread::spawn(move || {
                            start_networking(stream, r1, s2);
                        });

                        world.send_packet(Message::Connect);
                        self.logged_in = true;
                    }
                });
            });

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _event: Event) -> tetra::Result {
        Ok(())
    }
}
