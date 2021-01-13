use crate::prelude::*;
use enfaria_common::map::save_map;

pub fn handle_quits(server: &mut ServerData) {
    let mut quitters = vec![];

    {
        for user in server.users.iter() {
            let mut removed = false;
            for packet in user.receive_queue.iter() {
                if packet.command == Command::Quit {
                    quitters.push(user.id);
                    removed = true;
                    break;
                }
            }
            if removed {
                continue;
            }
            let now = get_timestamp();
            if now > user.time + 10_000 {
                quitters.push(user.id);
            }
        }
    }

    for quitter in quitters.iter() {
        let user = match server.user_by_id(*quitter) {
            Some(u) => u,
            None => {
                info!("Tried to remove non-existent player {:?}", quitter);
                continue;
            }
        };
        match std::fs::create_dir_all(&format!("data/{}", &user.username)) {
            Ok(_) => {}
            Err(e) => {
                info!("Failed to create save directory: {:?}", e);
                continue;
            }
        };
        save_map(&format!("data/{}/map", user.username), &user.map);
        info!("Player quit: {:?}", &user.username);
    }

    server.users.retain(|u| !quitters.contains(&u.id))
}
