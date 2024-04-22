// Copyright 2022 jmjoy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Modified by Eduardo Campaña (Sigan LLC) on Feb 21, 2024.

use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// Define a public structure named `Params` with a lifetime 'a.
/// The struct wraps a HashMap where keys and values are both Cow (clone-on-write) strings,
/// which allows for flexible borrowing and ownership semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Params<'a>(HashMap<Cow<'a, str>, Cow<'a, str>>);

/// Implementation block for the Params struct.
impl<'a> Params<'a> {
    /// Public constructor method to create a new instance of Params
    /// initialized with an empty HashMap.
    pub fn new() -> Self {
        Params(HashMap::new())
    }
}

/// Additional implementation block for the Params struct to provide methods
/// that add or modify FastCGI parameters within the Params struct.
/// References:
/// - https://github.com/nginx/nginx/blob/master/conf/fastcgi.conf
/// - https://www.nginx.com/resources/wiki/start/topics/examples/phpfcgi/
impl<'a> Params<'a> {
    /// Method to set the GATEWAY_INTERFACE parameter.
    #[inline]
    pub fn gateway_interface<S: Into<Cow<'a, str>>>(mut self, gateway_interface: S) -> Self {
        self.insert("GATEWAY_INTERFACE".into(), gateway_interface.into());
        self
    }

    /// Method to set the SERVER_SOFTWARE parameter.
    #[inline]
    pub fn server_software<S: Into<Cow<'a, str>>>(mut self, server_software: S) -> Self {
        self.insert("SERVER_SOFTWARE".into(), server_software.into());
        self
    }

    /// Method to set the SERVER_PROTOCOL parameter.
    #[inline]
    pub fn server_protocol<S: Into<Cow<'a, str>>>(mut self, server_protocol: S) -> Self {
        self.insert("SERVER_PROTOCOL".into(), server_protocol.into());
        self
    }

    /// Method to set the REQUEST_METHOD parameter.
    #[inline]
    pub fn request_method<S: Into<Cow<'a, str>>>(mut self, request_method: S) -> Self {
        self.insert("REQUEST_METHOD".into(), request_method.into());
        self
    }

    /// Method to set the SCRIPT_FILENAME parameter.
    #[inline]
    pub fn script_filename<S: Into<Cow<'a, str>>>(mut self, script_filename: S) -> Self {
        self.insert("SCRIPT_FILENAME".into(), script_filename.into());
        self
    }

    /// Method to set the SCRIPT_NAME parameter.
    #[inline]
    pub fn script_name<S: Into<Cow<'a, str>>>(mut self, script_name: S) -> Self {
        self.insert("SCRIPT_NAME".into(), script_name.into());
        self
    }

    /// Method to set the QUERY_STRING parameter.
    #[inline]
    pub fn query_string<S: Into<Cow<'a, str>>>(mut self, query_string: S) -> Self {
        self.insert("QUERY_STRING".into(), query_string.into());
        self
    }

    /// Method to set the REQUEST_URI parameter.
    #[inline]
    pub fn request_uri<S: Into<Cow<'a, str>>>(mut self, request_uri: S) -> Self {
        self.insert("REQUEST_URI".into(), request_uri.into());
        self
    }

    /// Method to set the DOCUMENT_ROOT parameter.
    #[inline]
    pub fn document_root<S: Into<Cow<'a, str>>>(mut self, document_root: S) -> Self {
        self.insert("DOCUMENT_ROOT".into(), document_root.into());
        self
    }

    /// Method to set the DOCUMENT_URI parameter.
    #[inline]
    pub fn document_uri<S: Into<Cow<'a, str>>>(mut self, document_uri: S) -> Self {
        self.insert("DOCUMENT_URI".into(), document_uri.into());
        self
    }

    /// Method to set the REMOTE_ADDR parameter.
    #[inline]
    pub fn remote_addr<S: Into<Cow<'a, str>>>(mut self, remote_addr: S) -> Self {
        self.insert("REMOTE_ADDR".into(), remote_addr.into());
        self
    }

    /// Method to set the REMOTE_PORT parameter as a string.
    #[inline]
    pub fn remote_port(mut self, remote_port: u16) -> Self {
        self.insert("REMOTE_PORT".into(), remote_port.to_string().into());
        self
    }

    /// Method to set the SERVER_ADDR parameter.
    #[inline]
    pub fn server_addr<S: Into<Cow<'a, str>>>(mut self, server_addr: S) -> Self {
        self.insert("SERVER_ADDR".into(), server_addr.into());
        self
    }

    /// Method to set the SERVER_PORT parameter as a string.
    #[inline]
    pub fn server_port(mut self, server_port: u16) -> Self {
        self.insert("SERVER_PORT".into(), server_port.to_string().into());
        self
    }

    /// Method to set the SERVER_NAME parameter.
    #[inline]
    pub fn server_name<S: Into<Cow<'a, str>>>(mut self, server_name: S) -> Self {
        self.insert("SERVER_NAME".into(), server_name.into());
        self
    }

    /// Method to set the CONTENT_TYPE parameter.
    #[inline]
    pub fn content_type<S: Into<Cow<'a, str>>>(mut self, content_type: S) -> Self {
        self.insert("CONTENT_TYPE".into(), content_type.into());
        self
    }

    /// Method to set the CONTENT_LENGTH parameter as a string.
    #[inline]
    pub fn content_length(mut self, content_length: usize) -> Self {
        self.insert("CONTENT_LENGTH".into(), content_length.to_string().into());
        self
    }

    /// Method to set the PATH_INFO parameter.
    #[inline]
    pub fn path_info<S: Into<Cow<'a, str>>>(mut self, path_info: S) -> Self {
        self.insert("PATH_INFO".into(), path_info.into());
        self
    }

    /// Method to set the PATH_TRANSLATED parameter.
    #[inline]
    pub fn path_translated<S: Into<Cow<'a, str>>>(mut self, path_translated: S) -> Self {
        self.insert("PATH_TRANSLATED".into(), path_translated.into());
        self
    }

    /// Method to set the HTTPS parameter based on a boolean value.
    #[inline]
    pub fn https(mut self, https: bool) -> Self {
        self.insert("HTTPS".into(), https.to_string().into());
        self
    }

    /// Generic method to insert a custom HTTP header into the params.
    #[inline]
    pub fn http_header<S: Into<Cow<'a, str>>>(mut self, header_key: S, header_value: S) -> Self {
        self.insert(header_key.into(), header_value.into());
        self
    }
}

/// Implement the Default trait for Params.
impl<'a> Default for Params<'a> {
    /// Provides a default value for Params which sets the GATEWAY_INTERFACE
    /// parameter to "CGI/1.1".
    fn default() -> Self {
        Params(HashMap::new()).gateway_interface("CGI/1.1")
    }
}

/// Implement Deref trait allowing Params to be treated like its inner HashMap.
impl<'a> Deref for Params<'a> {
    type Target = HashMap<Cow<'a, str>, Cow<'a, str>>;

    /// Method to dereference to the inner HashMap.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implement DerefMut trait allowing mutable access to the inner HashMap.
impl<'a> DerefMut for Params<'a> {
    /// Method to mutably dereference to the inner HashMap.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Implement the From trait for converting Params into its inner HashMap.
impl<'a> From<Params<'a>> for HashMap<Cow<'a, str>, Cow<'a, str>> {
    /// Convert Params into the inner HashMap by returning the inner HashMap.
    fn from(params: Params<'a>) -> Self {
        params.0
    }
}
