use std::{fmt::Error, net::SocketAddr};


use tokio::net::UdpSocket;

use crate::{misc::BinaryStream, protocol::Packet};



pub struct RakNetConnection<'a> {
	pub address: SocketAddr,
	server: &'a UdpSocket
}


impl <'a>RakNetConnection<'a> {
	pub fn new(address: SocketAddr, server: &'a UdpSocket) -> RakNetConnection {
		RakNetConnection {
			address,
			server
		}
	}
	
	pub async fn reply(&self, packet: &impl Packet) -> Result<bool, Error> {
		let mut stream: BinaryStream = BinaryStream::new(None);
        packet.serialize(&mut stream);

		if self.server.send_to(&stream.buffer, self.address).await.is_err() {
			Err::<bool, &str>("Failed to reply to connection.").unwrap();
		}
		Ok(true)
	}
}
