// mod fast_cgi;
// mod handler;
// mod php_cgi;
// mod context;

// use fast_cgi::FastCgiClient;
// use handler::handler;
use lambda_http::{run, service_fn};
// use php_cgi::PhpCgi;
use litespeed_client::{Client, Request};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use tracing::{debug, error, info, Level};

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

    // Start PHP LiteSpeed process.

    let command = "lsphp";

    let mut process = Command::new(command)
        .arg("-b")
        .arg(socket_path.to_str().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to start {} process", command));

    info!("Started {} process with id {}", command, process.id());

    // Log lsphp process stdout.

    let process_stdout = process.stdout.take().expect("Failed to take stdout.");

    tokio::spawn(async move {
        let reader = BufReader::new(process_stdout);

        for line in reader.lines() {
            if let Ok(line) = line {
                info!("lsphp: {}", line);
            }
        }
    });

    // Log lsphp process stderr.

    let process_stderr = process.stderr.take().expect("Failed to take stderr.");

    tokio::spawn(async move {
        let reader = BufReader::new(process_stderr);

        for line in reader.lines() {
            if let Ok(line) = line {
                error!("lsphp: {}", line);
            }
        }
    });

    // Spawns a task to handle graceful shutdown and kill the lsphp process.

    tokio::spawn(async move {
        elegant_departure::get_shutdown_guard().wait().await;

        info!("Shutting down {} process with id {}", command, process.id());

        process.kill().expect(&format!(
            "Failed to kill {} process with id {}",
            command,
            process.id()
        ));
    });

    // Connect client to PHP LiteSpeed process.

    let socket = "/tmp/lsphp.sock";
    let client = Client::new(socket).await;

    // Start server.

    let server = run(service_fn(|req| {
        let mut client = client.clone();

        println!("request {:#?}", req);

        async move {
            let request = Request::new().into_bytes();
            if let Err(e) = client.send(request).await {
                panic!("Error sending data: {:?}", e);
            }

            let response = match client.receive().await {
                Ok(data) => data,
                Err(e) => {
                    panic!("Error receiving data: {:?}", e);
                }
            };
            debug!("response {:#?}", response);

            Result::<&str, std::convert::Infallible>::Ok("👋 world!")
        }
    }));

    let shutdown_listener = elegant_departure::tokio::depart()
        .on_termination()
        .on_completion(async {
            let _ = server.await;
        });

    info!("Runtime listening to http://localhost:{host_port}\n");

    Ok(shutdown_listener.await)
}

// async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
// Extract some useful information from the request
// let who = event
//     .query_string_parameters_ref()
//     .and_then(|params| params.first("name"))
//     .unwrap_or("world");
// let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

// let fastcgi_params = FastCGIParams::default()
//     .request_method("GET")
//     .request_uri("/hello/world")
//     .script_name("index.php")
//     .script_filename("/var/task");

// let request = FastCGIRequest::new(fastcgi_params.clone(), tokio::io::empty());

// let response = client
//     .lock()
//     .await
//     .execute(request)
//     .await
//     .expect("failed to execute request");

// let output = String::from_utf8(response.stdout.expect("failed to read stdout"))
//     .expect("failed to convert stdout to string");

// println!("output: {}", output);

// Return something that implements IntoResponse.
// It will be serialized to the right response event automatically by the runtime
// }
