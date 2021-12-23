use std::error::Error;
use corevox::network::client::{VoxClient, VoxClientImpl};
use corevox::network::messages::VoxPack;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting");

    let mut client = VoxClientImpl::new("0.0.0.0:1990".to_string()).await.unwrap();

    client.send_pack(VoxPack {
        z: 7,
        raw: vec![0x00, 0x01, 0x02, 0x03]
    }).await.unwrap();

    return Ok(());
}
