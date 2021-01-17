use crate::prelude::*;

pub fn ping(server: &mut ServerData) {
    let mut packets = vec![];
    for user in server.users.iter() {
        let packet = Packet {
            beat: server.beat,
            command: Command::Ping,
            session_id: user.token.to_string(),
            destination: user.ip,
        };
        packets.push(packet);
    }
    for packet in packets {
        send_packet(server, packet.destination, packet);
    }
}
