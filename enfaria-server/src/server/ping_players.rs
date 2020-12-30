use crate::prelude::*;

pub fn ping_players(server: &mut ServerData) {
	let mut packets = vec![];
	for (ip, id) in server.players.iter() {
		let packet = Packet {
			beat: server.beat,
			command: Command::Ping,
			session_id: server.tokens.get(id).unwrap().to_string(),
			destination: ip.clone(),
		};
		packets.push(packet);
	};
	for packet in packets {
		send_packet(server, packet.destination.clone(), packet);
	}
}
