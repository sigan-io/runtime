use bytes::{BufMut, Bytes, BytesMut};
use std::mem::{size_of, size_of_val};

#[derive(Clone, Debug, Copy)]
pub struct EnvVariable<'a> {
    name_length: u16,
    value_length: u16,
    name: &'a str,
    value: &'a str,
}

impl<'a> EnvVariable<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self {
            name_length: (name.len() + 1) as u16, // +1 for null terminator.
            value_length: (value.len() + 1) as u16, // +1 for null terminator.
            name,
            value,
        }
    }

    pub fn len(&self) -> usize {
        size_of::<u16>() * 2 + size_of_val(&self.name) + size_of_val(&self.value)
    }

    pub fn into_bytes(self) -> Bytes {
        self.into()
    }
}

impl<'a> Into<Bytes> for EnvVariable<'a> {
    fn into(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.len());

        buffer.put_u16(self.name_length);
        buffer.put_u16(self.value_length);
        buffer.extend_from_slice(self.name.as_bytes());
        buffer.put_u8(0); // Null terminator required by LiteSpeed protocol.
        buffer.extend_from_slice(self.value.as_bytes());
        buffer.put_u8(0); // Null terminator required by LiteSpeed protocol.

        buffer.into()
    }
}

#[derive(Clone, Debug)]
pub struct EnvVariables<'a>(Vec<EnvVariable<'a>>);

impl<'a> EnvVariables<'a> {
    pub fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn add(&mut self, name: &'a str, value: &'a str) -> usize {
        let offset = self.0.len();
        self.0.push(EnvVariable::new(name, value));
        offset
    }

    pub fn get(&self, index: usize) -> Option<&EnvVariable<'a>> {
        self.0.get(index)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn len(&self) -> usize {
        self.0.iter().map(|env_variable| env_variable.len()).sum()
    }
}

impl<'a> Default for EnvVariables<'a> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<'a> Into<Bytes> for EnvVariables<'a> {
    fn into(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.len() + 4); // +4 bytes for null terminator.

        for env_variable in self.0 {
            buffer.put::<Bytes>(env_variable.into());
        }

        buffer.put_bytes(0, 4); // Null terminator required by LiteSpeed protocol.

        buffer.into()
    }
}

#[derive(Clone, Debug, Copy)]
pub struct RequiredEnvVariables<'a> {
    script_filename: Option<EnvVariable<'a>>,
    script_name: Option<EnvVariable<'a>>,
    query_string: Option<EnvVariable<'a>>,
    request_method: Option<EnvVariable<'a>>,
}

impl<'a> RequiredEnvVariables<'a> {
    pub fn new() -> Self {
        Self {
            script_filename: None,
            script_name: None,
            query_string: None,
            request_method: None,
        }
    }

    pub fn script_filename(&mut self, value: &'a str) {
        self.script_filename = Some(EnvVariable::new("SCRIPT_FILENAME", value));
    }

    pub fn get_script_filename(&self) -> Option<EnvVariable<'a>> {
        self.script_filename
    }

    pub fn script_name(&mut self, value: &'a str) {
        self.script_name = Some(EnvVariable::new("SCRIPT_NAME", value));
    }

    pub fn get_script_name(&self) -> Option<EnvVariable<'a>> {
        self.script_name
    }

    pub fn query_string(&mut self, value: &'a str) {
        self.query_string = Some(EnvVariable::new("QUERY_STRING", value));
    }

    pub fn get_query_string(&self) -> Option<EnvVariable<'a>> {
        self.query_string
    }

    pub fn request_method(&mut self, value: &'a str) {
        self.request_method = Some(EnvVariable::new("REQUEST_METHOD", value));
    }

    pub fn get_request_method(&self) -> Option<EnvVariable<'a>> {
        self.request_method
    }

    pub fn count(&self) -> usize {
        self.script_filename.map_or(0, |_| 1)
            + self.script_name.map_or(0, |_| 1)
            + self.query_string.map_or(0, |_| 1)
            + self.request_method.map_or(0, |_| 1)
    }

    pub fn len(&self) -> usize {
        self.script_filename.map_or(0, |value| value.len())
            + self.script_name.map_or(0, |value| value.len())
            + self.query_string.map_or(0, |value| value.len())
            + self.request_method.map_or(0, |value| value.len())
    }
}

impl<'a> Default for RequiredEnvVariables<'a> {
    fn default() -> Self {
        Self::new()
    }
}
