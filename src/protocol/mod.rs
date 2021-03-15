pub mod client {
    include!(concat!(env!("OUT_DIR"), "/client_protocol.rs"));
}
pub mod server {
    include!(concat!(env!("OUT_DIR"), "/server_protocol.rs"));
}
