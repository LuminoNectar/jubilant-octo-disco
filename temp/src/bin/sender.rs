use bytes::{Bytes, BytesMut};
use futures::stream;
use futures::{SinkExt, StreamExt};
use srt_tokio::SrtSocket;
use std::io::Error;
use std::time::{Duration, Instant};
use tokio::io::AsyncReadExt;
use tokio::time::sleep;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut srt_socket = SrtSocket::builder().listen_on(":3333").await?;

    //Open video file using std::fs::files to read into bytes and transmit
    let file = File::open("Keybinds.pdf").await?;

    let mut read_buffer = tokio::io::BufReader::new(file);
    //let mut list_of_chunks = vec::new();
    //let chunk_size = 0x4000;

    // loop {
    //     let mut chunk = vec::with_capacity(chunk_size);
    //     let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk)?;
    //     if n == 0 { break; }
    //     list_of_chunks.push(chunk);
    //     if n < chunk_size { break; }
    // }
    let mut stream = stream::unfold(&mut read_buffer, |read_buffer| async move {
        let mut chunk = BytesMut::with_capacity(8192);
        if let Ok(size) = read_buffer.read(chunk.as_mut()).await {
            if size == 0 {
                return None;
            }

            print!("\rSent {size:?} packets");
            sleep(Duration::from_millis(10)).await;
            Some((Ok((Instant::now(),Bytes::from(chunk))),read_buffer))
        } else {
            None
        }
    })
    .boxed();

    srt_socket.send_all(&mut stream).await?;
    Ok(())
}
