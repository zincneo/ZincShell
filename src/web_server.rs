use crate::Event;
use smol::channel::Sender;
use smol::{io::AsyncReadExt, net, stream::StreamExt};

pub async fn run(_tx: Sender<Event>) -> anyhow::Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1:4343").await?;
    let mut buffer = [0; 1024];
    while let Some(stream) = listener.incoming().next().await {
        let mut stream = stream?;
        let bytes_read = stream.read(&mut buffer).await?;
        let received = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        match received.as_str() {
            _ => (),
        }
    }
    Ok(())
}
