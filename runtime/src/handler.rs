use crate::fast_cgi::FastCgiClient;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use regex_lite::Regex;

pub async fn handler(req: Request, client: FastCgiClient) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(format!("{:#?}", req).into())
        .map_err(Box::new)?;

    let path = req.raw_http_path();

    /* Return 403 when trying to access protected files. */

    if Regex::new(r"^/(?:wp-content|wp-includes)/.*\.php$")?.is_match(path) {
        return access_forbidden();
    }

    if Regex::new(r"\.(?:crt|ini|htaccess|json|scss)$")?.is_match(path) {
        return access_forbidden();
    }

    let response = client.send("/index.php", req).await;

    println!("Response: {:#?}", response);

    Ok(resp)
}

fn access_forbidden() -> Result<Response<Body>, Error> {
    let response = Response::builder()
        .status(403)
        .header("content-type", "text/html")
        .body("403 Forbidden".into())
        .map_err(Box::new)?;

    Ok(response)
}
