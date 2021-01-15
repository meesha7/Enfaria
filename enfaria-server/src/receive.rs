use crate::prelude::*;
use async_std::{net::UdpSocket, task};
use enfaria_common::map::get_map;
use parking_lot::RwLock;
use sqlx::{mysql::MySqlPool, Row};
use std::{net::SocketAddr, path::Path, sync::Arc};

pub fn receive_data(server: Arc<RwLock<ServerData>>, socket: Arc<UdpSocket>, pool: Arc<MySqlPool>) {
    task::block_on(async move {
        loop {
            let mut buf = [0; 10000];
            let (amt, ip) = urcontinue!(socket.recv_from(&mut buf).await);
            if amt == 0 {
                info!("Received no data from {:?}", &ip);
                continue;
            }
            let packet: Packet = urcontinue!(bincode::deserialize(&buf));
            let mut s = server.write();
            info!("Receiving: {:?}", &packet);
            match packet.command {
                Command::Connect => {
                    connect_player(&mut s, ip, &packet, pool.as_ref()).await;
                }
                Command::Ping => ping_user(&mut s, ip),
                _ => {
                    receive_packet(&mut s, ip, packet.clone());
                }
            }
        }
    });
}

pub async fn connect_player(server: &mut ServerData, ip: SocketAddr, packet: &Packet, pool: &MySqlPool) {
    let row = match sqlx::query("SELECT CAST(user_id as UNSIGNED) FROM sessions WHERE secret = ?")
        .bind(&packet.session_id)
        .fetch_one(pool)
        .await
    {
        Ok(r) => r,
        Err(_) => return,
    };

    let user_id: u64 = row.get(0);

    let row = match sqlx::query("SELECT username FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(pool)
        .await
    {
        Ok(r) => r,
        Err(_) => return,
    };

    let username: String = row.get(0);

    let mut pid = PLAYER_ID.write();
    let id = UserId(*pid);

    *pid += 1;

    let map;

    if Path::new(&format!("data/{}/map", username)).exists() {
        map = match get_map(&format!("data/{}/map", username)) {
            Ok(m) => m,
            Err(_) => get_map("templates/farm.json").expect("Default map template not found."),
        };
    } else {
        map = get_map("templates/farm.json").expect("Default map template not found.");
    }

    let player;

    if Path::new(&format!("data/{}/player", username)).exists() {
        player = match get_player(&format!("data/{}/player", username)) {
            Ok(p) => p,
            Err(_) => get_player("templates/player.json").expect("Default player template not found."),
        };
    } else {
        player = get_player("templates/player.json").expect("Default player template not found.");
    }

    info!("Player added: {:?}", &username);

    let user = User::new(id, ip, username, packet.session_id.clone(), map, player);
    server.users.push(user);

    send_map(server, id);
    send_player(server, id);
}

pub fn send_map(server: &mut ServerData, id: UserId) {
    let user = server.user_by_id(id).expect("Failed to get user that just connected.");
    let ip = user.ip;
    let token = user.token.clone();
    let map = user.map.clone();

    let mut pos_x = 0;
    let mut pos_y = 0;
    for row in map.tiles {
        for column in row {
            let packet = Packet {
                beat: 0,
                command: Command::CreateTile((
                    Position {
                        x: pos_x,
                        y: pos_y,
                        z: id.0,
                    },
                    column,
                )),
                destination: ip,
                session_id: token.clone(),
            };
            send_packet(server, ip, packet);
            pos_x += 32
        }
        pos_x = 0;
        pos_y += 32;
    }
}

pub fn send_player(server: &mut ServerData, id: UserId) {
    let user = server
        .user_by_id_mut(id)
        .expect("Failed to get user that just connected.");
    let ip = user.ip;
    let token = user.token.clone();
    let player = user.player.clone();
    let username = user.username.clone();

    let packet = Packet {
        beat: 0,
        command: Command::CreatePlayer((player.position, username)),
        destination: ip,
        session_id: token.clone(),
    };

    user.send_packet(packet);

    for (slot, item) in player.inventory.into_iter() {
        let packet = Packet {
            beat: 0,
            command: Command::CreateItem((slot, item)),
            destination: ip,
            session_id: token.clone(),
        };
        user.send_packet(packet);
    }
}

pub fn ping_user(server: &mut ServerData, ip: SocketAddr) {
    let user = match server.user_by_ip_mut(ip) {
        Some(u) => u,
        None => return,
    };
    user.time = get_timestamp();
}
