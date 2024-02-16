use fastcgi_client::conn::KeepAlive;
use fastcgi_client::{Client as FastCGIClient, Params as FastCGIParams, Request as FastCGIRequest};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration, Instant};
use tracing::info;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
// async fn function_handler(
//     _event: Request,
//     client: Arc<Mutex<FastCGIClient<UnixStream, KeepAlive>>>,
// ) -> Result<Response<Body>, Error> {

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .init();

    // .with_env_filter(
    //     EnvFilter::builder()
    //         .with_default_directive(LevelFilter::INFO.into())
    //         .from_env_lossy(),
    // )
    // // disable printing the name of the module in every log line.
    // .with_target(false)
    // // disabling time is handy because CloudWatch will add the ingestion time.
    // .without_time()
    // .init();
    // let socket_path = ensure_socket()?;

    // let php = Command::new("php-cgi")
    //     .arg(format!("-b={socket_path}"))
    //     .spawn()
    //     .expect("Failed to spawn php-cgi process");

    // let stream = connect_to_socket(socket_path).await?;

    // println!("php-cgi process started: {:?}", php.id());

    // let client = Arc::new(Mutex::new(FastCGIClient::new_keep_alive(stream)));

    // run(service_fn(|event| function_handler(event, client.clone()))).await

    info!("Starting server...\n");

    run(service_fn(|event| function_handler(event))).await
}

fn ensure_socket() -> Result<&'static str, Error> {
    let socket_path = Path::new("/tmp/.sigan/php-cgi.sock");

    if socket_path.exists() {
        info!("The socket file already exists.");

        fs::remove_file(socket_path)?;

        info!("Removed socket file: {:?}", socket_path);
    }

    if let Some(parent) = socket_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::File::create(socket_path)?;

    info!("Created socket file: {:?}", socket_path);

    Ok(socket_path
        .to_str()
        .expect("Failed to convert path to string"))
}

async fn connect_to_socket(socket_path: &'static str) -> Result<UnixStream, Error> {
    let timeout = Duration::from_secs(5);
    let mut interval = interval(Duration::from_millis(100));
    let start_time = Instant::now();

    loop {
        tokio::select! {
            _ = interval.tick() => {
                info!("Attempting to connect to php-cgi...");

                if let Ok(stream) = UnixStream::connect(socket_path).await {
                    info!("Successfully connected to php-cgi");

                    return Ok(stream);
                }

                if start_time.elapsed() > timeout {
                    return Err("Failed to connect to php-cgi".into());
                }
            }
        }
    }
}

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    // let who = event
    //     .query_string_parameters_ref()
    //     .and_then(|params| params.first("name"))
    //     .unwrap_or("world");
    // let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    info!("Request received");

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
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("toto".into())
        .map_err(Box::new)?;

    Ok(resp)
}
