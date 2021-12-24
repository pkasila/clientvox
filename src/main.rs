use std::error::Error;
use std::io::Read;
use std::process::{Command, Stdio};
use corevox::network::client::{VoxClient, VoxClientImpl};
use corevox::network::messages::VoxPack;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting");

    let mut client = VoxClientImpl::new(std::env::args().nth(1).unwrap().to_string()).await.unwrap();

    let process = match Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("ffmpeg -i {} -s {}x{} -r 7 -t 1 -pix_fmt rgb565le -f rawvideo pipe:",
                     std::env::args().nth(2).unwrap(),
                     client.device_info.frame_size[0], client.device_info.frame_size[1]))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("couldn't spawn ffmpeg: {}", why),
        Ok(process) => process,
    };

    let mut s: Vec<u8> = vec![0; (client.device_info.frame_size[0] * client.device_info.frame_size[1] * 7 * 2) as usize];
    process.stdout.unwrap().read_exact(&mut s).unwrap_or_else(|e| { panic!("couldn't read ffmpeg stdout: {}", e) });

    println!("File size: {}", s.capacity());

    client.send_pack(VoxPack {
        z: 7,
        raw: s,
    }).await.unwrap();

    return Ok(());
}
