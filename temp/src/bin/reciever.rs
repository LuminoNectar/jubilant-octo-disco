use srt_tokio::SrtSocket;
use std::io::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut srt_socket =
        SrtSocket::builder().call("127.0.0.1:3333", None).await?;
    let file = File::create("other_file.txt").await?;
    let mut write_buffer = tokio::io::BufWriter::new(file);
    while let Some((_instant, bytes)) = srt_socket.try_next().await? {
        println!("Received {:?}", bytes);
        write_buffer.write_all(&bytes).await?;
    }
    write_buffer.flush().await?;

    println!("\nConnection closed");

    Ok(())
}
