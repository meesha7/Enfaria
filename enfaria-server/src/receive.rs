use crate::prelude::*;
use enfaria_common::{Packet, Command};
use std::{
    collections::hash_map::Entry,
    sync::Arc,
    net::SocketAddr,
};
use parking_lot::RwLock;
use async_std::{task, net::UdpSocket};
use sqlx::{Row, mysql::MySqlPool};


pub fn receive_data(server: Arc<RwLock<ServerData>>, socket: Arc<UdpSocket>, pool: Arc<MySqlPool>) {
    task::block_on(async move {
        loop {
            let mut buf = [0; 1000];
            let (amt, ip) = urcontinue!(socket.recv_from(&mut buf).await);
            if amt == 0 {
                continue
            }
            let packet: Packet = urcontinue!(bincode::deserialize(&buf));
            match packet.command {
                Command::Connect => { connect_player(server.clone(), ip, &packet, pool.as_ref()).await; }
                _ => { queue_packet(server.clone(), ip, packet.clone()); }
            }
        }
    });
}


pub async fn connect_player(server: Arc<RwLock<ServerData>>, ip: SocketAddr, packet: &Packet, pool: &MySqlPool) {
    let row = sqlx::query("SELECT * FROM sessions WHERE secret = ?").bind(&packet.session_id).fetch_one(pool).await;
    if row.is_err() {
        return;
    };
    let row = row.unwrap();
    if row.is_empty() {
        return;
    };
    let mut s = server.write();
    let mut pid = PLAYER_ID.write();
    let id = UserId(*pid);
    *pid += 1;
    s.players.insert(ip, id);
    s.tokens.insert(id, packet.session_id.clone());
}


pub fn queue_packet(server: Arc<RwLock<ServerData>>, ip: SocketAddr, packet: Packet) {
    let mut s = server.write();
    let id = match s.players.get(&ip) {
        Some(i) => *i,
        None => return,
    };
    let session_id = match s.tokens.get(&id) {
        Some(i) => i,
        None => return,
    };
    if session_id != &packet.session_id {
        return;
    };
    match s.receive_queue.entry(id) {
        Entry::Occupied(o) => { o.into_mut().push(packet); },
        Entry::Vacant(v) => { v.insert(vec![packet]); }
    }
}
