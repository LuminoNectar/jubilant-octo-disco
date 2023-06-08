use bytes::Bytes;
use futures::stream;
use futures::{SinkExt, StreamExt};
use srt_tokio::SrtSocket;
use std::io::Error;
use std::time::{Duration, Instant};
use tokio::io::AsyncBufReadExt;
use tokio::time::sleep;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut srt_socket = SrtSocket::builder().listen_on(":3333").await?;

    //Open video file using std::fs::files to read into bytes and transmit
    let file = File::open("video.mp4").await?;
    let read_buffer = tokio::io::BufReader::with_capacity(8192, file);

    let mut stream = stream::unfold((read_buffer, 0), |(mut read_buffer,count)| async move {

        let chunk = read_buffer.fill_buf().await.ok()?.to_vec();
        if chunk.is_empty() {
            return None;
        }
        // print!("\r{count}");
        read_buffer.consume(chunk.len());

        sleep(Duration::from_millis(10)).await;
        Some((Ok((Instant::now(),Bytes::from(chunk))), (read_buffer,count+1)))
    })
    .boxed();

    srt_socket.send_all(&mut stream).await?;
    Ok(())
}
