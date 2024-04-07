use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Connection {
    stream: Arc<Mutex<UnixStream>>,
}

impl Connection {
    pub fn new(stream: UnixStream) -> Self {
        Self {
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    pub async fn send(&mut self, data: &[u8]) -> io::Result<()> {
        println!("Sending data...");
        self.stream.lock().await.write_all(data).await
    }

    pub async fn receive(&mut self) -> io::Result<Vec<u8>> {
        println!("Receiving data...");
        let mut buffer = Vec::new();
        let mut temp_buf = [0; 1024]; // Temporary buffer

        loop {
            println!("in the loop");
            let bytes_read = self.stream.lock().await.read(&mut temp_buf).await?;
            if bytes_read == 0 {
                break;
            } // If no bytes read, we're likely at EOF or connection closed

            buffer.extend_from_slice(&temp_buf[..bytes_read]);
            println!("buffer: {:#?}", buffer);
        }

        Ok(buffer)
    }
}
