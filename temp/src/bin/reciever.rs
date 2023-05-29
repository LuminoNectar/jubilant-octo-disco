use srt_tokio::SrtSocket;
use std::io::Error;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut srt_socket = SrtSocket::builder().call("127.0.0.1:3333", None).await?;
    while let Some((_instant, bytes)) = srt_socket.try_next().await? {
        println!("{bytes:?}");
    }

    println!("\nConnection closed");

    Ok(())
}
