
#[cfg(test)]
mod tests {    
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    
    use crate::client::{RakNetClient, RakNetConfiguration};
    use tokio::net::UdpSocket;

    #[tokio::test]
    async fn it_works() {
        // Test the RakNetClient here
        let client = RakNetClient::new(
            RakNetConfiguration {
                address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6000)),
                mtu: 1024,
                socket: UdpSocket::bind("127.0.0.1:6000").await.unwrap()
            }
        );

        client.connect("127.0.0.1:19142").await;
        
    }
}