use crossbeam_channel::*;
use enfaria_common::*;
use log::info;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn start_networking(mut stream: TcpStream, receiver: Receiver<Packet>, sender: Sender<Packet>) {
    loop {
        // Send queued packets.
        if let Ok(p) = receiver.try_recv() {
            info!("Sending packet: {:?}", &p);
            let serialized = bincode::serialize(&p).unwrap();
            stream.write_all(&serialized[..]).unwrap();
        }

        // Propagate received packets.
        let mut buffer = vec![0u8; 1024];
        if let Ok(v) = stream.read(&mut buffer) {
            if v == 0 {
                continue;
            }
            let packet = bincode::deserialize(&buffer).unwrap();
            info!("Reading packet: {:?}", &packet);
            sender.send(packet).unwrap();
        }
    }
}
