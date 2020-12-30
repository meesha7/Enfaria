use crate::prelude::*;
use enfaria_common::map::get_map;
use std::{
    sync::Arc,
    net::SocketAddr,
    path::Path,
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
                info!("Received no data from {:?}", &ip);
                continue
            }
            let packet: Packet = urcontinue!(bincode::deserialize(&buf));
            let mut s = server.write();
            info!("Receiving: {:?}", &packet);
            match packet.command {
                Command::Connect => { connect_player(&mut s, ip, &packet, pool.as_ref()).await; },
                Command::Ping => { ping_user(&mut s, ip) },
                _ => { receive_packet(&mut s, ip, packet.clone()); },
            }
        }
    });
}


pub async fn connect_player(server: &mut ServerData, ip: SocketAddr, packet: &Packet, pool: &MySqlPool) {
    let row = sqlx::query("SELECT CAST(user_id as UNSIGNED) FROM sessions WHERE secret = ?").bind(&packet.session_id).fetch_one(pool).await;
    if row.is_err() {
        return;
    };
    let row = row.unwrap();
    if row.is_empty() {
        return;
    };

    let user_id: u64 = row.get(0);

    let row = sqlx::query("SELECT username FROM users WHERE id = ?").bind(&user_id).fetch_one(pool).await;
    if row.is_err() {
        return;
    };
    let row = row.unwrap();
    if row.is_empty() {
        return;
    };

    let username: String = row.get(0);

    let mut pid = PLAYER_ID.write();
    let id = UserId(*pid);

    *pid += 1;
    server.players.insert(ip, id);
    server.tokens.insert(id, packet.session_id.clone());
    server.usernames.insert(id, username.clone());

    info!("Player added: {:?}", &username);

    send_map(server, id, ip, &username, &packet.session_id)
}

pub fn send_map(server: &mut ServerData, id: UserId, ip: SocketAddr, username: &str, session_id: &str) {
    let map;

    if Path::new(&format!("data/{}", username)).exists() {
        map = get_map(&format!("data/{}", username));
        server.maps.insert(id, map.clone());
    } else {
        map = get_map("templates/farm.toml");
        server.maps.insert(id, map.clone());
    }

    let mut pos_x = 0;
    let mut pos_y = 0;
    for row in map.tiles {
        for column in row {
            let packet = Packet {
                beat: 0,
                command: Command::CreateTile((Position {x: pos_x, y: pos_y, z: id.0.into()}, column)),
                destination: ip,
                session_id: session_id.to_string(),
            };
            send_packet(server, ip, packet);
            pos_x += 32
        }
        pos_x = 0;
        pos_y += 32;
    }
}

pub fn ping_user(server: &mut ServerData, ip: SocketAddr) {
    let id = match server.players.get(&ip) {
        Some(i) => *i,
        None => return,
    };

    server.times.insert(id, get_timestamp());
}
