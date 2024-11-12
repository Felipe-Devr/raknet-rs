use std::{io::Cursor, net::SocketAddr, time::{SystemTime, UNIX_EPOCH}, vec};
use tokio::net::UdpSocket;
use uuid::Uuid;
use crate::protocol::{OpenReply1, OpenRequest1, OpenRequest2, Packet, UnconnectedPing, UnconnectedPong};

pub struct RakNetConfiguration {
    pub address: SocketAddr,
    pub mtu: u16,
    pub socket: UdpSocket,
}

pub struct RakNetClient {
    pub mtu: u16,
    pub address: SocketAddr,
    pub socket: UdpSocket,
    pub guid: u64
}

impl RakNetClient {
    pub fn new(configuration: RakNetConfiguration) -> RakNetClient {
        let guid= Uuid::new_v4().into_bytes();
        let mut bytes: [u8; 8] = [0u8; 8];
        bytes.copy_from_slice(&guid[0..8]);

        RakNetClient {
            guid: u64::from_be_bytes(bytes),
            mtu: configuration.mtu,
            address: configuration.address,
            socket: configuration.socket,
        }
    }

    pub async fn connect(&self, address: &str) {
        self.socket.connect(address).await.unwrap();
        /* let packet = OpenRequest1 {  mtu: 1427, protocol_version: 11 }; */
        let packet = UnconnectedPing { 
            guid:self.guid,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
        };
        let mut a: Vec<u8> = vec![];
        packet.serialize(&mut a);

        self.socket
            .send(&a)
            .await
            .expect("Failed to send message");

		let mut response_buf: [u8; 1024] = [0; 1024];
		match self.socket.recv(&mut response_buf).await {
			Ok(_bytes) => {
                
				let packet = UnconnectedPong::deserialize(&mut Cursor::new(Vec::from(response_buf)));

                if packet.is_some() {
                    println!("Received packet: {:?}", packet);
                }
			}
			Err(e) => {
                eprintln!("Error receiving response: {}", e);
            }
		}
    }
}
