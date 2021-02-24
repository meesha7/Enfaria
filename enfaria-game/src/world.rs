use crossbeam_channel::*;
use enfaria_common::*;

// Mutable global state, used in every scene.
#[derive(Default)]
pub struct GameWorld {
    pub state: State,
    // Gets filled after user logs in, is then available until logout.
    pub receiver: Option<Receiver<Packet>>,
    pub sender: Option<Sender<Packet>>,
    pub session_id: String,
}

impl GameWorld {
    pub fn new() -> Self {
        let state = State::new(Mode::Client);

        GameWorld {
            state,
            ..Default::default()
        }
    }

    pub fn send_packet(&self, message: Message) {
        let packet = Packet {
            destination: "127.0.0.1:8888".parse().unwrap(),
            message,
            session_id: self.session_id.clone(),
        };
        self.sender.as_ref().unwrap().try_send(packet).unwrap();
    }
}
