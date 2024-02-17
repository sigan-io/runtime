use std::{fs, path::Path, process::Command, sync::Arc};
use tokio::{
    net::UnixStream,
    time::{interval, Duration, Instant},
};
use tracing::info;

pub struct PhpCgi {
    stream: Arc<UnixStream>,
}

impl PhpCgi {
    const COMMAND: &'static str = "php-cgi";
    const SOCKET: &'static str = "/tmp/.sigan/php-cgi.sock";
}

impl PhpCgi {
    pub async fn new() -> Self {
        Self::ensure_socket();

        let mut process = Command::new(Self::COMMAND)
            .arg(&format!("-b={}", Self::SOCKET))
            .spawn()
            .expect(&format!("Failed to start {} process", Self::COMMAND));

        info!("Started {} process: {}", Self::COMMAND, process.id());

        let stream = Arc::new(Self::connect_to_socket().await);

        // Spawns a task to handle graceful shutdown and killing the php-cgi process.
        tokio::spawn(async move {
            elegant_departure::get_shutdown_guard().wait().await;

            info!("Shutting down {} process: {}", Self::COMMAND, process.id());

            process.kill().expect(&format!(
                "Failed to kill {} process: {}",
                Self::COMMAND,
                process.id()
            ));
        });

        Self { stream }
    }

    pub fn get_stream(&self) -> Arc<UnixStream> {
        self.stream.clone()
    }
}

impl PhpCgi {
    fn ensure_socket() {
        info!("Creating a socket...");
        let socket_path = Path::new(Self::SOCKET);

        if socket_path.exists() {
            info!("Socket already exist");
            fs::remove_file(socket_path).expect("Failed to remove socket");
            info!("Existing socket removed");
        }

        if let Some(parent) = socket_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create socket parent path");
        }

        fs::File::create(socket_path).expect("Failed to create socket");
        info!("Created new socket {:?}", socket_path);
    }

    async fn connect_to_socket() -> UnixStream {
        let timeout = Duration::from_secs(5);
        let mut interval = interval(Duration::from_millis(5));
        let start_time = Instant::now();

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    info!("Attempting to connect to {}...", Self::COMMAND);

                    if let Ok(stream) = UnixStream::connect(Self::SOCKET).await {
                        info!("Successfully connected to {}", Self::COMMAND);
                        return stream;
                    }

                    if start_time.elapsed() > timeout {
                        panic!("Failed to connect to {}", Self::COMMAND);
                    }
                }
            }
        }
    }
}
