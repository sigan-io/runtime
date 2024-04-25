use lambda_http::{run, service_fn};
use std::fs;
use std::path::Path;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get environment variables.

    let host_port = std::env::var("HOST_PORT").unwrap_or(3000.to_string());

    // Set up tracing.

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .without_time()
        .with_target(false)
        .with_ansi(true)
        .init();

    // Create socket file.

    info!("Creating a socket...");
    let socket_path = Path::new("/tmp/lsphp.sock");

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

    // Start server.

    let server = run(service_fn(|req| async move {
        println!("AWS Request:\n{:#?}", req);
        Result::<&str, std::convert::Infallible>::Ok("👋 world!")
    }));

    let shutdown_listener = elegant_departure::tokio::depart()
        .on_termination()
        .on_completion(async {
            let _ = server.await;
        });

    info!("Runtime listening to http://localhost:{host_port}\n");

    Ok(shutdown_listener.await)
}
