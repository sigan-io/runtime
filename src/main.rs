// mod fast_cgi;
mod php_cgi;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use php_cgi::PhpCgi;
use tracing::{debug, info, Level};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
// async fn function_handler(
//     _event: Request,
//     client: Arc<Mutex<FastCGIClient<UnixStream, KeepAlive>>>,
// ) -> Result<Response<Body>, Error> {

#[tokio::main]
async fn main() {
    // Get environment variables.
    let host_port = std::env::var("HOST_PORT").expect("HOST_PORT must be set");

    // Set up tracing.
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .without_time()
        .with_target(false)
        .with_ansi(true)
        .init();

    // Get stream to communicate with php-cgi.
    let _stream = PhpCgi::new().await.get_stream();

    // let client = Arc::new(Mutex::new(FastCGIClient::new_keep_alive(stream)));

    // run(service_fn(|event| function_handler(event, client.clone()))).await

    info!("Runtime listening to http://localhost:{host_port}\n");

    tokio::spawn(run(service_fn(|event| function_handler(event))));

    elegant_departure::tokio::depart().on_termination().await
}

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    // let who = event
    //     .query_string_parameters_ref()
    //     .and_then(|params| params.first("name"))
    //     .unwrap_or("world");
    // let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    debug!("Request received");

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
