use std::net::SocketAddr;

use tokio::net::UdpSocket;

use crate::{
    misc::{BinaryStream, Frame},
    protocol::{ConnectionRequest, FramePacket, FrameSetPacket, Packet},
};

pub struct RakNetConnection<'a> {
    pub address: SocketAddr,
    pub mtu: u16,
    server: &'a UdpSocket,
}

impl<'a> RakNetConnection<'a> {
    pub fn new(address: SocketAddr, mtu: u16, server: &'a UdpSocket) -> RakNetConnection {
        RakNetConnection {
            address,
            server,
            mtu,
        }
    }

    pub async fn reply(&self, packet: &impl Packet) -> Result<bool, &str> {
        let mut stream: BinaryStream = BinaryStream::new(None);
        packet.serialize(&mut stream);

        if self
            .server
            .send_to(&stream.buffer, self.address)
            .await
            .is_err()
        {
            return Err("Failed to send packet.");
        }
        Ok(true)
    }

    pub async fn send_frame(&self, _frame: Frame) -> Result<bool, &str> {
        todo!("Missing implementation on send_frame") // TODO: Implement frame queue
    }

    pub async fn handle_incoming(&self, stream: &mut BinaryStream, packet_id: u8) {
        let connected_packet_id = packet_id & 0xf0;

        println!("Got packet ID: 0x{:02x}", connected_packet_id);
        match connected_packet_id {
            0x80 => {
                self.handle_frameset(stream).await;
            }

            _ => {
                println!("Unhandled packet ID: 0x{:02x}", packet_id);
            }
        };
    }

    pub async fn handle_incoming_batch(&self, stream: &mut BinaryStream) {
        let packet_id = stream.read_u8().unwrap();

        println!("Received batch packet ID: 0x{:02x}", packet_id);

        match packet_id {
            ConnectionRequest::ID => {}

            0x1 => {}

            _ => {}
        }
    }

    pub async fn handle_frameset(&self, stream: &mut BinaryStream) {
        let deserialized = FrameSetPacket::deserialize(stream).unwrap();

        // TODO: Handle properly frame sets

        for frame in deserialized.frames {
            self.handle_frame(frame).await;
        }
    }

    pub async fn handle_frame(&self, frame: Frame) {
        // TODO: handle properly frames.

        self.handle_incoming_batch(&mut BinaryStream::new(Some(frame.body)))
            .await;
    }
}
