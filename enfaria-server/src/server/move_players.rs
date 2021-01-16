use crate::prelude::*;

pub fn move_players(server: &mut ServerData) {
    for user in server.users.iter_mut() {
        let mut final_pos = None;
        for packet in user.receive_queue.iter() {
            if let Command::Move(mov) = packet.command {
                let Position { x, y, z } = mov;
                if x % 4 != 0 || y % 4 != 0 || z % 4 != 0 {
                    return;
                }
                if !user.map.can_visit(mov) {
                    continue;
                }
                let mut check = mov;
                check.x += 28;
                check.y += 28;
                if !user.map.can_visit(check) {
                    continue;
                }
                final_pos = Some(mov)
            }
        }

        let position = match final_pos {
            Some(p) => p,
            None => continue,
        };

        let packet = Packet {
            beat: server.beat,
            command: Command::Move(position),
            destination: user.ip,
            session_id: user.token.clone(),
        };

        user.player.position = position;

        user.send_packet(packet);
        user.receive_queue.retain(|p| !p.command.is_move());
    }
}
