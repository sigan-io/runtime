use crate::{EnvVariables, PacketHeader, RequestHeader};
use bytes::{BufMut, Bytes, BytesMut};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Request<'a> {
    packet_header: PacketHeader,
    request_header: RequestHeader,
    special_env_variables: EnvVariables<'a>,
    general_env_variables: EnvVariables<'a>,
}

impl<'a> Request<'a> {
    pub fn new() -> Self {
        Self {
            packet_header: PacketHeader::default(),
            request_header: RequestHeader::default(),
            special_env_variables: EnvVariables::default(),
            general_env_variables: EnvVariables::default(),
        }
    }

    pub fn document_root(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("DOCUMENT_ROOT", value);
        self
    }

    pub fn remote_addr(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("REMOTE_ADDR", value);
        self
    }

    pub fn remote_port(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("REMOTE_PORT", value);
        self
    }

    pub fn server_addr(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("SERVER_ADDR", value);
        self
    }

    pub fn server_name(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("SERVER_NAME", value);
        self
    }

    pub fn server_port(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("SERVER_PORT", value);
        self
    }

    pub fn request_uri(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("REQUEST_URI", value);
        self
    }

    pub fn path_info(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("PATH_INFO", value);
        self
    }

    pub fn path_translated(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("PATH_TRANSLATED", value);
        self
    }

    pub fn orig_path_info(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("ORIG_PATH_INFO", value);
        self
    }

    pub fn redirect_status(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("REDIRECT_STATUS", value);
        self
    }

    pub fn redirect_url(mut self, value: &'a str) -> Self {
        self.general_env_variables.add("REDIRECT_URL", value);
        self
    }

    pub fn redirect_query_string(mut self, value: &'a str) -> Self {
        self.general_env_variables
            .add("REDIRECT_QUERY_STRING", value);
        self
    }

    pub fn script_filename(mut self, value: &'a str) -> Self {
        let index = self.general_env_variables.add("SCRIPT_FILENAME", value);
        self.request_header.script_filename_offset(index as u32);
        self
    }

    pub fn script_name(mut self, value: &'a str) -> Self {
        let index = self.general_env_variables.add("SCRIPT_NAME", value);
        self.request_header.script_name_offset(index as u32);
        self
    }

    pub fn query_string(mut self, value: &'a str) -> Self {
        let index = self.general_env_variables.add("QUERY_STRING", value);
        self.request_header.query_string_offset(index as u32);
        self
    }

    pub fn request_method(mut self, value: &'a str) -> Self {
        let index = self.general_env_variables.add("REQUEST_METHOD", value);
        self.request_header.request_method_offset(index as u32);
        self
    }

    pub fn add_env_variable(mut self, name: &'a str, value: &'a str) -> Self {
        self.general_env_variables.add(name, value);
        self
    }

    pub fn add_env_variables(mut self, env_variables: HashMap<&'a str, &'a str>) -> Self {
        for (name, value) in env_variables {
            self.general_env_variables.add(name, value);
        }
        self
    }
}

impl<'a> Into<Bytes> for Request<'a> {
    fn into(self) -> Bytes {
        let mut buffer = BytesMut::new();

        // Request header
        buffer.put::<Bytes>(self.packet_header.into());
        buffer.put::<Bytes>(self.request_header.into());

        // Special and general environment variables
        buffer.put::<Bytes>(self.special_env_variables.into());
        buffer.put::<Bytes>(self.general_env_variables.into());

        // Add padding
        let padding = (8 - (buffer.len() % 8)) % 8;
        buffer.put_bytes(0, padding);

        // HTTP headers

        buffer.into()
    }
}
