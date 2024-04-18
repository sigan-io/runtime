use bytes::{BufMut, Bytes, BytesMut};
use std::mem::size_of;

#[derive(Clone, Debug, Copy)]
pub struct RequestHeader {
    http_header_length: u32,
    request_body_length: u32,
    script_filename_offset: u32,
    script_name_offset: u32,
    query_string_offset: u32,
    request_method_offset: u32,
    unknown_headers_count: u32,
    env_variables_count: u32,
    special_env_variables_count: u32, // Always 0
}

impl RequestHeader {
    pub fn new() -> Self {
        Self {
            http_header_length: 0,
            request_body_length: 0,
            script_filename_offset: 0,
            script_name_offset: 0,
            query_string_offset: 0,
            request_method_offset: 0,
            unknown_headers_count: 0,
            env_variables_count: 0,
            special_env_variables_count: 0,
        }
    }

    pub fn http_header_length(&mut self, length: u32) -> &Self {
        self.http_header_length = length;
        self
    }

    pub fn request_body_length(&mut self, length: u32) -> &Self {
        self.request_body_length = length;
        self
    }

    pub fn script_filename_offset(&mut self, offset: u32) -> &Self {
        self.script_filename_offset = offset;
        self
    }

    pub fn get_script_filename_offset(&self) -> u32 {
        self.script_filename_offset
    }

    pub fn script_name_offset(&mut self, offset: u32) -> &Self {
        self.script_name_offset = offset;
        self
    }

    pub fn get_script_name_offset(&self) -> u32 {
        self.script_name_offset
    }

    pub fn query_string_offset(&mut self, offset: u32) -> &Self {
        self.query_string_offset = offset;
        self
    }

    pub fn get_query_string_offset(&self) -> u32 {
        self.query_string_offset
    }

    pub fn request_method_offset(&mut self, offset: u32) -> &Self {
        self.request_method_offset = offset;
        self
    }

    pub fn get_request_method_offset(&self) -> u32 {
        self.request_method_offset
    }

    pub fn unknown_headers_count(&mut self, count: u32) -> &Self {
        self.unknown_headers_count = count;
        self
    }

    pub fn env_variables_count(&mut self, count: u32) -> &Self {
        self.env_variables_count = count;
        self
    }

    pub fn special_env_variables_count(&mut self, count: u32) -> &Self {
        self.special_env_variables_count = count;
        self
    }

    pub fn len(&self) -> usize {
        size_of::<u32>() * 9
    }

    pub fn into_bytes(self) -> Bytes {
        self.into()
    }
}

impl Default for RequestHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Into<Bytes> for RequestHeader {
    fn into(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.len());

        buffer.put_u32(self.http_header_length);
        buffer.put_u32(self.request_body_length);
        buffer.put_u32(self.script_filename_offset);
        buffer.put_u32(self.script_name_offset);
        buffer.put_u32(self.query_string_offset);
        buffer.put_u32(self.request_method_offset);
        buffer.put_u32(self.unknown_headers_count);
        buffer.put_u32(self.env_variables_count);
        buffer.put_u32(self.special_env_variables_count);

        buffer.into()
    }
}
