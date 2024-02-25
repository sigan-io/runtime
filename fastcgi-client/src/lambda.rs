use crate::Params;
use std::str::FromStr;

#[cfg(feature = "lambda_http")]
impl<'a> From<lambda_http::Request> for Params<'a> {
    fn from(mut req: lambda_http::Request) -> Self {
        let content_type = req
            .headers_mut()
            .remove("content-type")
            .map(|value| value.to_str().unwrap_or_default().to_owned())
            .unwrap_or_default()
            .to_string();
        let content_length = req
            .headers_mut()
            .remove("content-length")
            .map(|value| value.to_str().unwrap_or_default().to_owned())
            .map(|value| usize::from_str(&value).unwrap_or_default())
            .unwrap_or_default();

        let request_method = req.method().as_str().to_string();
        let request_uri = req.uri().to_string();
        let query_string = req.uri().query().unwrap_or_default().to_string();

        // let document_root =

        Self::default()
            .request_method(request_method)
            .request_uri(request_uri)
            .query_string(query_string)
            .content_type(content_type)
            .content_length(content_length)
    }
}
