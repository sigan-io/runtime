use crate::{EnvVariables, PacketHeader, RequestHeader, RequiredEnvVariables};
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Clone, Debug)]
pub struct Request<'a> {
    packet_header: PacketHeader,
    request_header: RequestHeader,
    special_env_variables: EnvVariables<'a>,
    required_env_variables: RequiredEnvVariables<'a>,
    general_env_variables: EnvVariables<'a>,
}

impl<'a> Request<'a> {
    pub fn new() -> Self {
        Self {
            packet_header: PacketHeader::default(),
            request_header: RequestHeader::default(),
            special_env_variables: EnvVariables::default(),
            required_env_variables: RequiredEnvVariables::default(),
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
        self.required_env_variables.script_filename(value);
        self
    }

    pub fn script_name(mut self, value: &'a str) -> Self {
        self.required_env_variables.script_name(value);
        self
    }

    pub fn query_string(mut self, value: &'a str) -> Self {
        self.required_env_variables.query_string(value);
        self
    }

    pub fn request_method(mut self, value: &'a str) -> Self {
        self.required_env_variables.request_method(value);
        self
    }

    pub fn len(&self) -> usize {
        let length = self.packet_header.len()
            + self.request_header.len()
            + self.special_env_variables.len()
            + self.general_env_variables.len();

        let padding = (8 - (length % 8)) % 8;

        length + padding
    }

    pub fn into_bytes(self) -> Bytes {
        self.into()
    }
}

impl<'a> Into<Bytes> for Request<'a> {
    fn into(mut self) -> Bytes {
        let mut packet_header_buffer = BytesMut::with_capacity(self.packet_header.len());
        let mut request_header_buffer = BytesMut::with_capacity(self.request_header.len());

        let mut buffer = BytesMut::with_capacity(
            self.special_env_variables.len()
                + self.required_env_variables.len()
                + self.general_env_variables.len(),
        );

        // Update the packet length
        self.packet_header.packet_length(self.len() as u32);

        // Update the number of environment variables
        self.request_header.env_variables_count(
            (self.required_env_variables.count() + self.general_env_variables.count()) as u32,
        );

        // Update script filename offset and append to buffer
        if let Some(script_filename) = self.required_env_variables.get_script_filename() {
            self.request_header.script_filename_offset(
                (self.packet_header.len() + self.request_header.len() + buffer.len()) as u32,
            );
            buffer.put::<Bytes>(script_filename.into());
        }

        // Update script name offset and append to buffer
        if let Some(script_name) = self.required_env_variables.get_script_name() {
            self.request_header.script_name_offset(
                (self.packet_header.len() + self.request_header.len() + buffer.len()) as u32,
            );
            buffer.put::<Bytes>(script_name.into());
        }

        // Update query string offset and append to buffer
        if let Some(query_string) = self.required_env_variables.get_query_string() {
            self.request_header.query_string_offset(
                (self.packet_header.len() + self.request_header.len() + buffer.len()) as u32,
            );
            buffer.put::<Bytes>(query_string.into());
        }

        // Update request method offset and append to buffer
        if let Some(request_method) = self.required_env_variables.get_request_method() {
            self.request_header.request_method_offset(
                (self.packet_header.len() + self.request_header.len() + buffer.len()) as u32,
            );
            buffer.put::<Bytes>(request_method.into());
        }

        // Add padding
        let buffer_length = self.packet_header.len() + self.request_header.len() + buffer.len();
        let padding = (8 - (buffer_length % 8)) % 8;
        buffer.put_bytes(0, padding);

        // Append packet header to buffer
        packet_header_buffer.put::<Bytes>(self.packet_header.into());

        // Append request header to buffer
        request_header_buffer.put::<Bytes>(self.request_header.into());

        let mut chain = packet_header_buffer
            .chain(request_header_buffer)
            .chain(buffer);

        // HTTP headers

        chain.copy_to_bytes(self.len())
    }
}
