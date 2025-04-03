use std::net::SocketAddr;

pub fn get_server_address() -> SocketAddr {
    "127.0.0.1:8080".parse().expect("Invalid server address")
}
