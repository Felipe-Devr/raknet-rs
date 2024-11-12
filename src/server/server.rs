use std::{fmt::Error, net::SocketAddr};

use tokio::net::UdpSocket;

pub struct RakNetConfiguration {
	pub address: SocketAddr
}

pub struct RakNetServer {
	pub address: SocketAddr,
    pub connections: Vec<u8>,
    pub socket: UdpSocket,
    pub guid: i64
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
		loop {
			let mut buffer = [0; 4096];
            let (size, peer_addr) = self.socket.recv_from(&mut buffer).await.unwrap();
		}
	}
}

