use crate::prelude::*;

pub fn chat(server: &mut ServerData) {
    let mut messages = vec![];
    for user in server.users.iter_mut() {
        for packet in user.receive_queue.iter() {
            if let Command::ChatSend(ref msg) = packet.command {
                messages.push(msg.clone());
            }
        }
        user.receive_queue.retain(|p| !p.command.is_chat_send());
    }
    for message in messages.into_iter() {
        for user in server.users.iter_mut() {
            let packet = Packet {
                beat: server.beat,
                session_id: user.token.clone(),
                destination: user.ip,
                command: Command::ChatReceive(message.clone()),
            };
            user.send_packet(packet);
        }
    }
}
