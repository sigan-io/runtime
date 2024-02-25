use fastcgi_client::Params;
use lambda_http::{Error, Request};
use std::borrow::Cow;

pub trait RuntimeExt {
    fn set_runtime_context(&mut self, context: RuntimeContext) -> Option<RuntimeContext>;
    fn runtime_context(&self) -> Option<&RuntimeContext>;
}

#[derive(Clone)]
pub struct RuntimeContext<'a> {
    script_name: &'a str,
    document_root: &'a str,
}

impl<'a> RuntimeContext<'a> {
    pub fn new<S: AsRef<str>>(script_name: S, document_root: S) -> Self {
        Self {
            script_name: script_name.as_ref(),
            document_root: document_root.as_ref(),
        }
    }

    pub fn script_name(&self) -> &str {
        &self.script_name
    }

    pub fn document_root(&self) -> &str {
        &self.script_name
    }
}

impl RuntimeExt for Request {
    fn set_runtime_context(&mut self, context: RuntimeContext) -> Option<RuntimeContext> {
        self.extensions_mut().insert(context)
    }
    fn runtime_context(&self) -> Option<&RuntimeContext> {
        self.extensions().get::<RuntimeContext>()
    }
}

// impl<'a> From<lambda_http::Request> for Params<'a> {
//     fn from(mut req: lambda_http::Request) -> Self {
//         let content_type = req
//             .headers_mut()
//             .remove("content-type")
//             .map(|value| value.to_str().unwrap_or_default().to_owned())
//             .unwrap_or_default()
//             .to_string();
//         let content_length = req
//             .headers_mut()
//             .remove("content-length")
//             .map(|value| value.to_str().unwrap_or_default().to_owned())
//             .map(|value| usize::from_str(&value).unwrap_or_default())
//             .unwrap_or_default();

//         let request_method = req.method().as_str().to_string();
//         let request_uri = req.uri().to_string();
//         let query_string = req.uri().query().unwrap_or_default().to_string();

//         // let document_root =

//         Self::default()
//             .request_method(request_method)
//             .request_uri(request_uri)
//             .query_string(query_string)
//             .content_type(content_type)
//             .content_length(content_length)
//     }
// }
