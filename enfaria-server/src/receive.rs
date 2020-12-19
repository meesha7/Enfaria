use crate::prelude::*;
use farmer_common::{Packet, Command};
use std::{
    collections::hash_map::Entry,
    sync::Arc,
    net::{UdpSocket, SocketAddr},
};
use parking_lot::RwLock;
use smol::Async;


pub async fn receive_data(server: Arc<RwLock<ServerData>>, server_ip: SocketAddr) {
    let socket = Async::<UdpSocket>::bind(server_ip).unwrap();
    loop {
        let mut buf = [0; 1000];
        let (amt, ip) = urcontinue!(socket.recv_from(&mut buf).await);
        if amt == 0 {
            continue
        }
        let packet: Packet = urcontinue!(bincode::deserialize(&buf));
        match packet.command {
            Command::Connect => { connect_player(server.clone(), ip); }
            _ => { queue_packet(server.clone(), ip, packet.clone()); }
        }
    }
}


pub fn connect_player(server: Arc<RwLock<ServerData>>, ip: SocketAddr) {
    let mut s = server.write();
    let mut pid = PLAYER_ID.write();
    let id = UserId(*pid);
    *pid += 1;
    s.players.insert(ip, id);
}


pub fn queue_packet(server: Arc<RwLock<ServerData>>, ip: SocketAddr, packet: Packet) {
    let mut s = server.write();
    let id = match s.players.get(&ip) {
        Some(i) => *i,
        None => return,
    };
    match s.receive_queue.entry(id) {
        Entry::Occupied(o) => { o.into_mut().push(packet); },
        Entry::Vacant(v) => { v.insert(vec![packet]); }
    }
}
