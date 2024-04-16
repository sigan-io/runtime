use crate::errors::HttpHeaderError;
use bytes::{BufMut, Bytes, BytesMut};
use std::mem::size_of_val;

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
        size_of_val(&self.name_length)
            + size_of_val(&self.value_length)
            + size_of_val(&self.name)
            + size_of_val(&self.value)
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
pub enum HttpHeaders {
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,
    Authorization,
    Connection,
    ContentType,
    ContentLength,
    Cookie,
    Cookie2,
    Host,
    Pragma,
    Referer,
    UserAgent,
    CacheControl,
    IfModifiedSince,
    IfMatch,
    IfNoneMatch,
    IfRange,
    IfUnmodifiedSince,
    KeepAlive,
    Range,
    XForwardedFor,
    Via,
    TransferEncoding,
}

impl HttpHeaders {
    // Total HTTP headers.
    // Will move to std::mem::variants_count once it is stable.
    pub const COUNT: usize = 25;
}

impl TryFrom<u8> for HttpHeaders {
    type Error = HttpHeaderError;

    fn try_from(value: u8) -> Result<Self, HttpHeaderError> {
        match value {
            0 => Ok(Self::Accept),
            1 => Ok(Self::AcceptCharset),
            2 => Ok(Self::AcceptEncoding),
            3 => Ok(Self::AcceptLanguage),
            4 => Ok(Self::Authorization),
            5 => Ok(Self::Connection),
            6 => Ok(Self::ContentType),
            7 => Ok(Self::ContentLength),
            8 => Ok(Self::Cookie),
            9 => Ok(Self::Cookie2),
            10 => Ok(Self::Host),
            11 => Ok(Self::Pragma),
            12 => Ok(Self::Referer),
            13 => Ok(Self::UserAgent),
            14 => Ok(Self::CacheControl),
            15 => Ok(Self::IfModifiedSince),
            16 => Ok(Self::IfMatch),
            17 => Ok(Self::IfNoneMatch),
            18 => Ok(Self::IfRange),
            19 => Ok(Self::IfUnmodifiedSince),
            20 => Ok(Self::KeepAlive),
            21 => Ok(Self::Range),
            22 => Ok(Self::XForwardedFor),
            23 => Ok(Self::Via),
            24 => Ok(Self::TransferEncoding),
            _ => Err(HttpHeaderError::UnknownHeader),
        }
    }
}

impl From<HttpHeaders> for u8 {
    fn from(value: HttpHeaders) -> Self {
        match value {
            HttpHeaders::Accept => 0,
            HttpHeaders::AcceptCharset => 1,
            HttpHeaders::AcceptEncoding => 2,
            HttpHeaders::AcceptLanguage => 3,
            HttpHeaders::Authorization => 4,
            HttpHeaders::Connection => 5,
            HttpHeaders::ContentType => 6,
            HttpHeaders::ContentLength => 7,
            HttpHeaders::Cookie => 8,
            HttpHeaders::Cookie2 => 9,
            HttpHeaders::Host => 10,
            HttpHeaders::Pragma => 11,
            HttpHeaders::Referer => 12,
            HttpHeaders::UserAgent => 13,
            HttpHeaders::CacheControl => 14,
            HttpHeaders::IfModifiedSince => 15,
            HttpHeaders::IfMatch => 16,
            HttpHeaders::IfNoneMatch => 17,
            HttpHeaders::IfRange => 18,
            HttpHeaders::IfUnmodifiedSince => 19,
            HttpHeaders::KeepAlive => 20,
            HttpHeaders::Range => 21,
            HttpHeaders::XForwardedFor => 22,
            HttpHeaders::Via => 23,
            HttpHeaders::TransferEncoding => 24,
        }
    }
}

// pub struct KnownHttpHeaderIndex {
//     header_length: [u16; HttpHeaders::COUNT],
//     header_offset: [u32; HttpHeaders::COUNT],
// }

// pub struct UnknownHeaderOffset {
//     name_offset: u32,
//     name_length: u32,
//     value_offset: u32,
//     value_length: u32,
// }

// pub struct ResponseInfo {
//     headers_count: u32,
//     status: u32,
// }

// pub struct ResponseHeader {
//     packet_header: PacketHeader,
//     response_info: ResponseInfo,
// }
