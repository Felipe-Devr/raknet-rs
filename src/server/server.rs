use std::{fmt::Error, io::Cursor, net::SocketAddr, time::UNIX_EPOCH};

use byteorder::ReadBytesExt;
use tokio::net::UdpSocket;

use crate::protocol::{Packet, UnconnectedPing, UnconnectedPong};

pub struct RakNetConfiguration {
	pub address: SocketAddr
}

pub struct RakNetServer {
	pub address: SocketAddr,
    pub connections: Vec<u8>,
    pub socket: UdpSocket,
    pub guid: u64
}

impl RakNetServer {
	pub async fn new(configuration: RakNetConfiguration) -> RakNetServer {
        let socket = UdpSocket::bind(configuration.address).await.expect("Failed to create server socket");

		RakNetServer {
			address: configuration.address,
            connections: Vec::new(),
            socket,
            guid: 3242348208447
		}
    }

	pub async fn listen(&self) -> Result<bool, Error> {
		println!("Server listening on port {}\n", self.address.port());
		loop {
			let mut buffer = [0; 4096];
            let (_, peer_addr) = self.socket.recv_from(&mut buffer).await.unwrap();
			let mut cursor = Cursor::new(Vec::from(buffer));
			let packet_id = cursor.read_u8().expect("Failled to read packet id.");

			println!("Received packet with ID: {} from {}", packet_id, peer_addr);

			if packet_id == UnconnectedPing::ID {
				let response = UnconnectedPong {
					id: String::from("MCPE;Rustified!;100;1.0.0;0;10;3242348208447;Rust Raknet;Survival;1;19142;19143;"),
					timestamp: std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
                    server_guid: self.guid
				};

				println!("{:?}", response);
				let mut response_buf: Vec<u8> = Vec::new();
                response.serialize(&mut response_buf);
                self.socket.send_to(&response_buf, peer_addr).await.expect("Failed to send response");
			}
		}
	}
}

