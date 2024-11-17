use crate::{
    misc::{Address, BinaryStream},
    protocol::{
        OpenReply1, OpenReply2, OpenRequest1, OpenRequest2, Packet, UnconnectedPing,
        UnconnectedPong,
    },
};
use std::{net::SocketAddr, str::FromStr, time::Duration};
use tokio::{net::UdpSocket, time::Instant};
use uuid::Uuid;

pub struct RakNetConfiguration {
    pub address: SocketAddr,
    pub mtu: u16,
}

pub struct RakNetClient {
    pub mtu: u16,
    pub address: SocketAddr,
    pub socket: UdpSocket,
    pub guid: u64,
}

impl RakNetClient {
    pub async fn new(configuration: RakNetConfiguration) -> RakNetClient {
        let guid = Uuid::new_v4().into_bytes();
        let mut bytes: [u8; 8] = [0u8; 8];
        bytes.copy_from_slice(&guid[0..8]);

        let socket = match UdpSocket::bind(configuration.address).await {
            Ok(socket) => socket,
            Err(_) => panic!("Failed to bind socket to: {}", configuration.address),
        };

        RakNetClient {
            guid: u64::from_be_bytes(bytes),
            mtu: configuration.mtu,
            address: configuration.address,
            socket,
        }
    }

    pub async fn send(&self, packet: &impl Packet) -> Result<(), &str> {
        let mut stream = BinaryStream::new(None);
        packet.serialize(&mut stream);

        match self.socket.send(&stream.buffer).await {
            Ok(_) => {
                println!("Sent packet with id: 0x{:02x}", stream.buffer[0]);
                Ok(())
            }
            Err(_) => Err("Failed to send packet"),
        }
    }

    pub async fn dial(&self, address: &str) -> Result<UnconnectedPong, &str> {
        let mut stream = BinaryStream::new(None);
        let ping = UnconnectedPing {
            guid: self.guid,
            timestamp: Duration::new(32482392, 2342349),
        };

        ping.serialize(&mut stream);

        match self.socket.send_to(&stream.buffer, address).await {
            Ok(_) => {
                let mut buffer = [0u8; 2048];

                match self.socket.recv_from(&mut buffer).await {
                    Ok(_) => {
                        let mut stream = BinaryStream::new(Some(buffer.to_vec()));
                        let pong: UnconnectedPong =
                            UnconnectedPong::deserialize(&mut stream).unwrap();

                        Ok(pong)
                    }

                    Err(_) => Err("Ping timeout"),
                }
            }
            Err(_) => Err("Failed to send ping packet"),
        }
    }

    pub async fn connect(&self, address: &str) {
        self.socket.connect(address).await.unwrap();
        let request1 = OpenRequest1 {
            mtu: self.mtu,
            protocol_version: 11,
        };

        let response = self.send_till_response(&request1).await;

        if response.is_err() {
            println!("Failed to connect to server");
            return;
        }
        let reply = OpenReply1::deserialize(&mut response.unwrap()).unwrap();
        println!("Received reply1: {:?}", reply);
        let request2 = OpenRequest2 {
            address: Address::from_str(address).unwrap(),
            client_guid: self.guid,
            cookie: None,
            supports_security: false,
            mtu: self.mtu,
        };

        let reply2 = self.send_till_response(&request2).await;

        if reply2.is_err() {
            println!("Failed to connect to server");
            return;
        }
        let reply2 = OpenReply2::deserialize(&mut reply2.unwrap()).unwrap();
        println!("Received reply2: {:?}", reply2);
    }

    async fn send_till_response(&self, packet: &impl Packet) -> Result<BinaryStream, &str> {
        let start = Instant::now();
        loop {
            match self.send(packet).await {
                Ok(_) => {
                    let response = self.receive().await;

                    if start.elapsed().as_secs() > 10 {
                        return Err("Timeout waiting for response");
                    }
                    if response.is_none() {
                        continue;
                    }
                    return Ok(response.unwrap());
                }
                Err(e) => return Err(e),
            }
        }
    }

    async fn receive(&self) -> Option<BinaryStream> {
        let mut buffer: [u8; 1792] = [0u8; 1792];

        match self.socket.recv_from(&mut buffer).await {
            Ok((_, _)) => Some(BinaryStream::new(Some(buffer.to_vec()))),

            Err(_) => {
                println!("Failed to receive packet");
                None
            }
        }
    }
}
