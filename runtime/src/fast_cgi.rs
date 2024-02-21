use fastcgi_client::conn::KeepAlive;
use fastcgi_client::{Client, Params, Request as FastCgiRequest};
use lambda_http::{request::RequestContext::ApiGatewayV2, Request, RequestExt};
use std::{
    env,
    sync::{Arc, Mutex},
};
use tokio::net::UnixStream;
use tokio::time::{interval, Duration, Instant};
use tracing::info;

pub struct FastCgiClient {
    client: Arc<Mutex<Client<UnixStream, KeepAlive>>>,
    wp_root: String,
}

impl FastCgiClient {
    pub async fn new(socket: &str) -> Self {
        let wp_root = env::var("WORDPRESS_PATH").unwrap_or(String::from("/mnt/wordpress"));
        let stream = Self::connect_to_server(socket).await;
        let client = Client::new_keep_alive(stream);

        Self {
            client: Arc::new(Mutex::new(client)),
            wp_root,
        }
    }

    pub async fn send(&self, script_name: &str, req: Request) {
        let params = self.prepare_params(script_name, &req);

        println!("context: {:#?}", req.request_context());

        println!("params: {:#?}", params);
    }

    fn prepare_params<'a>(&'a self, script_name: &'a str, req: &'a Request) -> Params {
        let method = req.method().to_string();
        let request_uri = req.uri().to_string();
        let query_string = req.query_string_parameters().to_query_string();
        let content_type = "".to_string();
        let content_length = 0;

        let document_root = &self.wp_root;
        let document_uri = req.raw_http_path();
        let script_filename = format!("{}{}", &self.wp_root, script_name);
        let path_info = "".to_string();
        let path_translated = "".to_string(); 

        let params = Params::default()
            .gateway_interface("CGI/1.1")
            .server_software("sigan/runtime")
            .server_protocol("HTTP/1.1")
            .request_method(method)
            .request_uri(request_uri)
            .query_string(query_string)
            .content_type(content_type)
            .content_length(content_length)
            .script_filename(script_filename)
            .script_name(script_name)
            .document_root(document_root)
            .document_uri(document_uri);

        // if let ApiGatewayV2(context) = req.request_context() {
        //     let remote_addr = context.http.source_ip.unwrap_or("".to_string());

        //     params.remote_addr(remote_addr);
        // }

        // .remote_addr("remote_addr")
        // .remote_port("remote_port")
        // .server_addr("server_addr")
        // .server_port("server_port")
        // .server_name("php-cgi")
        // .content_type("content_type")
        // .content_length("content_length");

        params
    }
}

impl Clone for FastCgiClient {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            wp_root: self.wp_root.clone(),
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
