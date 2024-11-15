use std::{collections::HashMap, fmt::Error, net::SocketAddr, time::UNIX_EPOCH};

use tokio::net::UdpSocket;

use crate::{
    misc::{Address, BinaryStream},
    protocol::{
        /* ConnectionRequest, ConnectionRequestAccepted, FrameSetPacket, */ OpenReply1,
        OpenReply2, OpenRequest1, OpenRequest2, Packet, UnconnectedPing, UnconnectedPong,
    },
};

use super::RakNetConnection;

pub struct RakNetConfiguration {
    pub address: SocketAddr,
    pub motd1: String,
    pub motd2: String,
    pub max_players: i32,
}

pub struct RakNetServer<'a> {
    pub address: SocketAddr,
    pub socket: UdpSocket,
    pub guid: u64,
    pub pong: String,
    pub connections: HashMap<SocketAddr, RakNetConnection<'a>>,
}

impl<'a> RakNetServer<'a> {
    pub async fn new(configuration: RakNetConfiguration) -> RakNetServer<'a> {
        let socket = UdpSocket::bind(configuration.address)
            .await
            .expect("Failed to create server socket");
        let port = configuration.address.port();
        let guid = 3242348208447;

        RakNetServer {
            address: configuration.address,
            connections: HashMap::new(),
            socket,
            pong: format!(
                "MCPE;{};100;1.0.0;{};{};{};{};Survival;1;{};{};",
                configuration.motd1,
                5,
                configuration.max_players,
                configuration.motd2,
                guid,
                port,
                port + 1
            ),
            guid,
        }
    }

    pub async fn listen(&'a mut self) -> Result<bool, Error> {
        println!("Server listening on port {}\n", self.address.port());

        loop {
            let mut buffer: [u8; 2048] = [0; 2048];
            let (size, peer_addr) = self.socket.recv_from(&mut buffer).await.unwrap();
            let mut buf = buffer.to_vec();
            buf.resize(size, 0x00);

            let mut stream = BinaryStream::new(Some(buf));
            let packet_id = stream.read_u8().expect("Failled to read packet id.");

            println!(
                "Received packet with ID: {} from {} with size: {}",
                packet_id, peer_addr, size
            );

            match packet_id {
                UnconnectedPing::ID => {
                    let response = UnconnectedPong {
                        id: self.pong.clone(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap(),
                        server_guid: self.guid,
                    };
                    self.send(&peer_addr, response).await;
                }

                OpenRequest1::ID => {
                    let deserialized = OpenRequest1::deserialize(&mut stream)
                        .expect("Failed to deserialize OpenRequest1");

                    let response = OpenReply1 {
                        use_security: false,
                        server_guid: self.guid,
                        cookie: None,
                        mtu: deserialized.mtu,
                    };

                    self.send(&peer_addr, response).await;
                }

                OpenRequest2::ID => {
                    let deserialized = OpenRequest2::deserialize(&mut stream)
                        .expect("Failed to deserialize OpenRequest2");

                    let response = OpenReply2 {
                        server_guid: self.guid,
                        mtu: deserialized.mtu,
                        client_address: Address::from(&peer_addr),
                        encryption_enabled: false,
                    };

                    let connection =
                        RakNetConnection::new(peer_addr, deserialized.mtu, &self.socket);

                    self.connections.insert(peer_addr, connection);
                    self.send(&peer_addr, response).await;
                }

                0x80..0x8d => {
                    if !self.connections.contains_key(&peer_addr) {
                        println!("Received packet from unconnected client: {}", peer_addr);
                        continue;
                    }
                    let connection = self.connections.get_mut(&peer_addr).unwrap();

                    connection.handle_incoming(&mut stream, packet_id).await;
                }

                _ => {
                    println!("Unknown packet ID: 0x{:02x}", packet_id);
                }
            }
        }
    }

    pub async fn send(&self, address: &SocketAddr, packet: impl Packet) {
        let mut stream = BinaryStream::new(None);
        packet.serialize(&mut stream);
        self.socket
            .send_to(&stream.buffer, address)
            .await
            .expect("Failed to send packet");
    }
}
