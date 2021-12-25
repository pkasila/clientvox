use std::error::Error;
use std::io::{Read, BufRead};
use std::process::{Command, Stdio};
use corevox::network::client::{VoxClient, VoxClientImpl};
use corevox::network::messages::VoxPack;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting");

    let mut client = VoxClientImpl::new(std::env::args().nth(1).unwrap().to_string()).await.unwrap();

    let process = match Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("ffmpeg -i {} -s {}x{} -r 7 -t 1 -c:v h264_videotoolbox -f mp4 -movflags frag_keyframe+empty_moov pipe:",
                     std::env::args().nth(2).unwrap(),
                     client.device_info.frame_size[0], client.device_info.frame_size[1]))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("couldn't spawn ffmpeg: {}", why),
        Ok(process) => process,
    };

    let mut s: Vec<u8> = vec![];
    process.stdout.unwrap().read_to_end(&mut s).unwrap_or_else(|e| { panic!("couldn't read ffmpeg stdout: {}", e) });

    println!("File size: {}", s.len());

    client.send_pack(VoxPack {
        z: 7,
        raw: s,
    }).await.unwrap();

    return Ok(());
}
