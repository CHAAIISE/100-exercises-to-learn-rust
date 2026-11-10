// TODO: fix the `assert_eq` at the end of the tests.
//  Do you understand why that's the resulting output?
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener, n_messages: usize, timeout: Duration) -> Vec<u8> {
    let mut buffer = Vec::new();
    // Keep the streams alive: dropping one would reset its connection and
    // make the corresponding client fail with a broken pipe on its next write.
    let mut streams = Vec::new();
    for _ in 0..n_messages {
        let (mut stream, _) = listener.accept().await.unwrap();
        let _ = tokio::time::timeout(timeout, async {
            stream.read_to_end(&mut buffer).await.unwrap();
        })
        .await;
        streams.push(stream);
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn ping() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let messages = vec!["hello", "from", "this", "task"];
        let timeout = Duration::from_millis(20);
        let handle = tokio::spawn(run(listener, messages.len(), timeout.clone()));

        for message in messages {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (_, mut writer) = socket.split();

            let (beginning, end) = message.split_at(message.len() / 2);

            // Send first half
            writer.write_all(beginning.as_bytes()).await.unwrap();
            tokio::time::sleep(timeout * 2).await;
            writer.write_all(end.as_bytes()).await.unwrap();
            // Close the write side of the socket
            let _ = writer.shutdown().await;
        }

        let buffered = handle.await.unwrap();
        let buffered = std::str::from_utf8(&buffered).unwrap();
        // Each read_to_end is cancelled by the timeout before the client
        // sends the second half of the message. The first message is not
        // read at all: its connection takes longer than the timeout to be
        // established, so only the first halves of the remaining messages
        // make it into the buffer.
        assert_eq!(buffered, "frthta");
    }
}
