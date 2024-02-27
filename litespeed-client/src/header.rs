use crate::error::HeaderError;

pub enum PacketType {
    BeginRequest,
    AbortRequest,
    ResponseHeader,
    ResponseStream,
    ResponseEnd,
    StderrStream,
    RequestReceived,
    ConnectionClose,
    InternalError,
    UnknownType,
}

impl From<u8> for PacketType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::BeginRequest,
            2 => Self::AbortRequest,
            3 => Self::ResponseHeader,
            4 => Self::ResponseStream,
            5 => Self::ResponseEnd,
            6 => Self::StderrStream,
            7 => Self::RequestReceived,
            8 => Self::ConnectionClose,
            9 => Self::InternalError,
            _ => Self::UnknownType,
        }
    }
}

impl From<PacketType> for u8 {
    fn from(value: PacketType) -> Self {
        match value {
            PacketType::BeginRequest => 1,
            PacketType::AbortRequest => 2,
            PacketType::ResponseHeader => 3,
            PacketType::ResponseStream => 4,
            PacketType::ResponseEnd => 5,
            PacketType::StderrStream => 6,
            PacketType::RequestReceived => 7,
            PacketType::ConnectionClose => 8,
            PacketType::InternalError => 9,
            PacketType::UnknownType => 10,
        }
    }
}

pub enum KnownHttpHeaders {
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
    UnknownHeader,
}

impl KnownHttpHeaders {
    pub const COUNT: usize = 26;
}

impl From<u8> for KnownHttpHeaders {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Accept,
            1 => Self::AcceptCharset,
            2 => Self::AcceptEncoding,
            3 => Self::AcceptLanguage,
            4 => Self::Authorization,
            5 => Self::Connection,
            6 => Self::ContentType,
            7 => Self::ContentLength,
            8 => Self::Cookie,
            9 => Self::Cookie2,
            10 => Self::Host,
            11 => Self::Pragma,
            12 => Self::Referer,
            13 => Self::UserAgent,
            14 => Self::CacheControl,
            15 => Self::IfModifiedSince,
            16 => Self::IfMatch,
            17 => Self::IfNoneMatch,
            18 => Self::IfRange,
            19 => Self::IfUnmodifiedSince,
            20 => Self::KeepAlive,
            21 => Self::Range,
            22 => Self::XForwardedFor,
            23 => Self::Via,
            24 => Self::TransferEncoding,
            _ => Self::UnknownHeader,
        }
    }
}

impl From<KnownHttpHeaders> for u8 {
    fn from(value: KnownHttpHeaders) -> Self {
        match value {
            KnownHttpHeaders::Accept => 0,
            KnownHttpHeaders::AcceptCharset => 1,
            KnownHttpHeaders::AcceptEncoding => 2,
            KnownHttpHeaders::AcceptLanguage => 3,
            KnownHttpHeaders::Authorization => 4,
            KnownHttpHeaders::Connection => 5,
            KnownHttpHeaders::ContentType => 6,
            KnownHttpHeaders::ContentLength => 7,
            KnownHttpHeaders::Cookie => 8,
            KnownHttpHeaders::Cookie2 => 9,
            KnownHttpHeaders::Host => 10,
            KnownHttpHeaders::Pragma => 11,
            KnownHttpHeaders::Referer => 12,
            KnownHttpHeaders::UserAgent => 13,
            KnownHttpHeaders::CacheControl => 14,
            KnownHttpHeaders::IfModifiedSince => 15,
            KnownHttpHeaders::IfMatch => 16,
            KnownHttpHeaders::IfNoneMatch => 17,
            KnownHttpHeaders::IfRange => 18,
            KnownHttpHeaders::IfUnmodifiedSince => 19,
            KnownHttpHeaders::KeepAlive => 20,
            KnownHttpHeaders::Range => 21,
            KnownHttpHeaders::XForwardedFor => 22,
            KnownHttpHeaders::Via => 23,
            KnownHttpHeaders::TransferEncoding => 24,
            KnownHttpHeaders::UnknownHeader => 25,
        }
    }
}

pub enum Endianness {
    LittleEndian = 0,
    BigEndian = 1,
}

impl TryFrom<u8> for Endianness {
    type Error = HeaderError;

    fn try_from(value: u8) -> Result<Self, HeaderError> {
        match value {
            0 => Ok(Self::LittleEndian),
            1 => Ok(Self::BigEndian),
            _ => Err(HeaderError::Endianness),
        }
    }
}

impl From<Endianness> for u8 {
    fn from(value: Endianness) -> Self {
        match value {
            Endianness::LittleEndian => 0,
            Endianness::BigEndian => 1,
        }
    }
}

pub enum PacketLength {
    Length(i32),
    Bytes([u8; 4]),
}

pub struct PacketHeader {
    version_b0: u8,
    version_b1: u8,
    packet_type: PacketType,
    endianness: Endianness,
    packet_length: PacketLength,
}

impl PacketHeader {
    fn get_packet_type(&self) -> &PacketType {
        &self.packet_type
    }
    fn get_endianness(&self) -> &Endianness {
        &self.endianness
    }
    fn get_packet_length(&self) -> &PacketLength {
        &self.packet_length
    }
}

pub struct RequestHeader {
    packet_header: PacketHeader,
    http_header_length: u32,
    request_body_length: u32,
    script_filename_offset: u32,
    script_name_offset: u32,
    query_string_offset: u32,
    request_method_offset: u32,
    unknown_headers_count: u32,
    env_variables_count: u32,
    special_env_variables_count: u32,
}

pub struct HttpHeaderIndex {
    header_length: [u16; KnownHttpHeaders::COUNT],
    header_offset: [i32; KnownHttpHeaders::COUNT],
}

pub struct HeaderOffset {
    name_offset: u32,
    name_length: u32,
    value_offset: u32,
    value_length: u32,
}

pub struct ResponseInfo {
    headers_count: u32,
    tatus: u32,
}

pub struct ResponseHeader {
    packet_header: PacketHeader,
    response_info: ResponseInfo,
}
