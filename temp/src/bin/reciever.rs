use srt_tokio::SrtSocket;
use std::io::Error;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut srt_socket = SrtSocket::builder().call("127.0.0.1:3333", None).await?;
    let mut count = 0;

    while let Some((instant, bytes)) = srt_socket.try_next().await? {
        count += 1;
        print!("\rReceived {count:?} packets");
        dbg!(instant, bytes);
    }

    println!("\nConnection closed");

    Ok(())
}
