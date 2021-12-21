use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    let clients = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (client_socket, _) = listener.accept().await?;
        let clients = clients.clone();
        let (mut readhalf, writehalf) = client_socket.into_split();
        let writehalf = Arc::new(Mutex::new(writehalf));
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let name_size = match readhalf.read(&mut buf).await {
                Ok(name_size) if name_size == 0 => return,
                Ok(name_size) => name_size,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            let name = format!("{}: ", (str::from_utf8(&buf[0..name_size])).unwrap().trim());
            {
                let mut clients_ref = clients.lock().await;
                clients_ref.insert(name.clone(), writehalf.clone());
            }
            loop {
                let n = match readhalf.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        let mut clients_ref = clients.lock().await;
                        clients_ref.remove(&name);
                        return;
                    }
                };
                for (key, value) in clients.lock().await.iter() {
                    if key != &name {
                        if let Err(e) = value.lock().await.write_all( &[(&name.as_bytes()), &buf[0..n]].concat()).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            let mut clients_ref = clients.lock().await;
                            clients_ref.remove(&name);
                        }
                    }
                }
                
            }
        });
    }
}


// (client_addr.to_string() + ": ").as_bytes()

