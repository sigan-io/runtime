use bytes::Bytes;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration, Instant};

#[derive(Clone, Debug)]
pub struct Client {
    address: &'static str,
    stream: Arc<Mutex<UnixStream>>,
}

impl Client {
    pub async fn new(address: &'static str) -> Self {
        let stream = Self::connect(address).await;

        Self {
            address,
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    pub async fn default() -> Self {
        Self::new("/tmp/lsphp.sock").await
    }

    async fn connect(address: &str) -> UnixStream {
        let timeout = Duration::from_secs(5);
        let mut interval = interval(Duration::from_millis(10));
        let start_time = Instant::now();

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    println!("Attempting to connect to {}...", address);

                    if let Ok(stream) = UnixStream::connect(address).await {
                        println!("Successfully connected to {}", address);
                        break stream;
                    }

                    if start_time.elapsed() > timeout {
                        panic!("Failed to connect to {}", address);
                    }
                }
            }
        }
    }

    pub fn address(&self) -> &str {
        self.address
    }

    pub fn stream(&self) -> Arc<Mutex<UnixStream>> {
        self.stream.clone()
    }
}

impl Client {
    pub async fn send(&mut self, packet: Bytes) -> io::Result<()> {
        println!("Sending data...");
        let mut stream = self.stream.lock().await;
        let _ = stream.write_all(&packet).await;
        stream.flush().await
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
            } // If no bytes read, we're likely at EOF or client closed

            buffer.extend_from_slice(&temp_buf[..bytes_read]);
            println!("buffer: {:#?}", buffer);
        }

        Ok(buffer)
    }
}
