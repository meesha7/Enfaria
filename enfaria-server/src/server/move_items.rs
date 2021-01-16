use crate::prelude::*;

pub fn move_items(server: &mut ServerData) {
    for user in server.users.iter_mut() {
        for packet in user.receive_queue.iter() {
            if let Command::MoveItem((from, to)) = packet.command {
                if user.player.inventory.keys().any(|&ix| ix == to) {
                    continue;
                };
                if let Some(item) = user.player.inventory.remove(&from) {
                    user.player.inventory.insert(to, item);
                }
            }
        }
        user.receive_queue.retain(|p| !p.command.is_move_item());
    }
}
