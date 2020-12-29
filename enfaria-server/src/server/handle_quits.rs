use crate::prelude::*;
use enfaria_common::{Command, map::save_map};

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
        let username = server.usernames.get(&quitter).unwrap();
        let map = server.maps.get(&quitter).unwrap();
        save_map(&format!("data/{}", username), map);

        server.players.retain(|_, v| v != &mut quitter);
        server.send_queue.remove(&quitter);
        server.receive_queue.remove(&quitter);
        server.maps.remove(&quitter);
        server.tokens.remove(&quitter);
        server.usernames.remove(&quitter);
        server.positions.remove(&quitter);
    }
}
