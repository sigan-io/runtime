use fastcgi_client::conn::KeepAlive;
use fastcgi_client::Client;
use lambda_http::{Request, RequestExt};
use std::sync::{Arc, Mutex};
use tokio::net::UnixStream;
use tokio::time::{interval, Duration, Instant};
use tracing::info;

pub struct FastCgiClient {
    client: Arc<Mutex<Client<UnixStream, KeepAlive>>>,
}

impl FastCgiClient {
    pub async fn new(socket: &str) -> Self {
        let stream = Self::connect_to_server(socket).await;
        let client = Client::new_keep_alive(stream);

        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn send(&self, script_name: &str, req: Request) {
        println!("context: {:#?} {:#?}", script_name, req.request_context());
    }
}

impl Clone for FastCgiClient {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
        }
    }
}

impl FastCgiClient {
    async fn connect_to_server(socket: &str) -> UnixStream {
        let timeout = Duration::from_secs(5);
        let mut interval = interval(Duration::from_millis(10));
        let start_time = Instant::now();

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    info!("Attempting to connect to {}...", socket);

                    if let Ok(stream) = UnixStream::connect(socket).await {
                        info!("Successfully connected to {}", socket);
                        return stream;
                    }

                    if start_time.elapsed() > timeout {
                        panic!("Failed to connect to {}", socket);
                    }
                }
            }
        }
    }
}
