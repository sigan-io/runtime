mod fast_cgi;
mod handler;
mod php_cgi;
mod context;

use fast_cgi::FastCgiClient;
use handler::handler;
use lambda_http::{run, service_fn};
use php_cgi::PhpCgi;
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    // Get environment variables.

    let host_port = std::env::var("HOST_PORT").unwrap_or(3000.to_string());

    // Set up tracing.

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .without_time()
        .with_target(false)
        .with_ansi(true)
        .init();

    // Start php-cgi process.

    let socket = PhpCgi::new().get_socket();

    // Initialize FastCGI client.

    let client = FastCgiClient::new(socket).await;

    // Start server.

    let server = run(service_fn(move |req| handler(req, client.clone())));

    tokio::spawn(server);

    info!("Runtime listening to http://localhost:{host_port}\n");

    elegant_departure::tokio::depart().on_termination().await
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
