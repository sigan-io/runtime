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
    pub const COUNT: usize = 25; // TransferEncoding + 1
}

impl TryFrom<u8> for HttpHeaders {
    type Error = HeaderError;

    fn try_from(value: u8) -> Result<Self, HeaderError> {
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
            _ => Err(HeaderError::UnknownHeader),
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
    header_length: [u16; HttpHeaders::COUNT],
    header_offset: [i32; HttpHeaders::COUNT],
}

pub struct HeaderOffset {
    name_offset: u32,
    name_length: u32,
    value_offset: u32,
    value_length: u32,
}

pub struct ResponseInfo {
    headers_count: u32,
    status: u32,
}

pub struct ResponseHeader {
    packet_header: PacketHeader,
    response_info: ResponseInfo,
}
