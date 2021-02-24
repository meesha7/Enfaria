use crate::prelude::*;
use async_std::io::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task::block_on;
use parking_lot::RwLock;
use sqlx::mysql::MySqlPool;
use std::sync::Arc;

pub fn accept_connections(server: Arc<RwLock<Server>>, listener: TcpListener, pool: Arc<MySqlPool>) {
    block_on(async move {
        loop {
            // Listen for connections
            let (mut stream, ip) = match listener.accept().await {
                Ok(v) => v,
                Err(e) => {
                    info!("Failed to accept incoming connection: {:?}", e);
                    continue;
                }
            };

            match stream.set_nodelay(true) {
                Ok(_) => {}
                Err(e) => {
                    info!("Failed to set stream as no-delay: {:?}", e);
                    continue;
                }
            }

            let mut buf = vec![0u8; 1024];
            match stream.read(&mut buf).await {
                Ok(_) => {}
                Err(e) => {
                    info!("Failed to receive incoming packet: {:?}", e);
                    continue;
                }
            };

            let packet: Packet = match bincode::deserialize(&buf[..]) {
                Ok(p) => p,
                Err(e) => {
                    info!("Failed to deserialize incoming packet: {:?}", e);
                    continue;
                }
            };

            match add_user(stream, ip, packet, pool.as_ref()).await {
                Some(user) => {
                    let mut s = server.write();
                    if s.user_by_username(&user.username).is_some() {
                        info!("User named {:?} already exists!", &user.username);
                        continue;
                    }
                    info!("Added user named: {:?}", &user.username);
                    s.users.push(user)
                }
                None => continue,
            };
        }
    });
}

pub async fn add_user(stream: TcpStream, ip: SocketAddr, packet: Packet, pool: &MySqlPool) -> Option<User> {
    if packet.message != Message::Connect {
        info!("Received invalid first command");
        return None;
    }
    let record = sqlx::query!(
        "SELECT username FROM users WHERE id = (SELECT user_id FROM sessions WHERE secret = ?)",
        &packet.session_id
    )
    .fetch_one(pool)
    .await;

    let record = match record {
        Ok(r) => r,
        Err(e) => {
            info!("Database error: {:?}", e);
            return None;
        }
    };

    Some(User::new(ip, stream, record.username, packet.session_id))
}
