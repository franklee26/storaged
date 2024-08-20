use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:1318").await?;

    // Write some data.
    stream.write_all(b"hello world!").await?;

    Ok(())
}
