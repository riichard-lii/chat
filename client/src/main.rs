#![warn(clippy::pedantic, clippy::nursery)]
use std::error::Error;
use std::io;
use std::str;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8888").await?;
    let (mut read_half, mut write_half) = stream.into_split();
    println!("Enter your username: ");
    tokio::spawn(async move {
        loop {
            let mut buf = [0; 1024];
            let msg_size = match read_half.read(&mut buf).await {
                Ok(msg_size) if msg_size == 0 => return,
                Ok(msg_size) => msg_size,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            print!("{}", str::from_utf8(&buf[0..msg_size]).unwrap());
        }
    });

    // get username from stdin
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    write_half.write_all(name.as_bytes()).await?;
    loop {
        let mut name = String::new();
        io::stdin().read_line(&mut name)?;
        write_half.write_all(name.as_bytes()).await?;
    }
}
