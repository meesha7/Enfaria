use crate::prelude::*;

pub fn sort_data(server: &mut ServerData) {
    for user in server.users.iter_mut() {
        user.receive_queue.sort_by_key(|packet| packet.beat)
    }
}
