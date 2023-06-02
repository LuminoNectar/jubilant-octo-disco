use srt_tokio::SrtSocket;
use std::io::Error;
use tokio_stream::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut srt_socket = SrtSocket::builder().call("127.0.0.1:3333", None).await?;
    let file = File::create("temp.pdf").await?;
    let mut write_buffer = tokio::io::BufWriter::new(file);
    while let Some((_instant, bytes)) = srt_socket.try_next().await? {
       write_buffer.write(&bytes).await?; 
    }

    println!("\nConnection closed");

    Ok(())
}
