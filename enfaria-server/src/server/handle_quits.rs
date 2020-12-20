use crate::prelude::*;
use enfaria_common::Command;

pub fn handle_quits(server: &mut ServerData) {
    let mut quitters = vec![];

    {
        for (userid, packets) in server.receive_queue.iter() {
            for packet in packets {
                if packet.command == Command::Quit {
                    quitters.push(userid.clone());
                }
            }
        }
    }

    for mut quitter in quitters {
        server.players.retain(|_, v| v != &mut quitter);
        server.send_queue.remove(&quitter);
        server.receive_queue.remove(&quitter);
    }
}
