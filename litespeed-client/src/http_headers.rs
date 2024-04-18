use crate::errors::HttpHeaderError;

#[derive(Clone, Debug, Copy)]
#[repr(u8)]
pub enum HttpHeader {
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

impl HttpHeader {
    // Total HTTP headers.
    // Will replace with std::mem::variants_count once it is stable.
    pub const VARIANTS_COUNT: usize = 25;
}

impl TryFrom<u8> for HttpHeader {
    type Error = HttpHeaderError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
            _ => Err(Self::Error::UnknownHeader),
        }
    }
}

impl TryFrom<&str> for HttpHeader {
    type Error = HttpHeaderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "accept" => Ok(Self::Accept),
            "accept-charset" => Ok(Self::AcceptCharset),
            "accept-encoding" => Ok(Self::AcceptEncoding),
            "accept-language" => Ok(Self::AcceptLanguage),
            "authorization" => Ok(Self::Authorization),
            "connection" => Ok(Self::Connection),
            "content-type" => Ok(Self::ContentType),
            "content-length" => Ok(Self::ContentLength),
            "cookie" => Ok(Self::Cookie),
            "cookie2" => Ok(Self::Cookie2),
            "host" => Ok(Self::Host),
            "pragma" => Ok(Self::Pragma),
            "referer" => Ok(Self::Referer),
            "user-agent" => Ok(Self::UserAgent),
            "cache-control" => Ok(Self::CacheControl),
            "if-modified-since" => Ok(Self::IfModifiedSince),
            "if-match" => Ok(Self::IfMatch),
            "if-none-match" => Ok(Self::IfNoneMatch),
            "if-range" => Ok(Self::IfRange),
            "if-unmodified-since" => Ok(Self::IfUnmodifiedSince),
            "keep-alive" => Ok(Self::KeepAlive),
            "range" => Ok(Self::Range),
            "x-forwarded-for" => Ok(Self::XForwardedFor),
            "via" => Ok(Self::Via),
            "transfer-encoding" => Ok(Self::TransferEncoding),
            _ => Err(Self::Error::UnknownHeader),
        }
    }
}

impl From<HttpHeader> for u8 {
    fn from(value: HttpHeader) -> Self {
        match value {
            HttpHeader::Accept => 0,
            HttpHeader::AcceptCharset => 1,
            HttpHeader::AcceptEncoding => 2,
            HttpHeader::AcceptLanguage => 3,
            HttpHeader::Authorization => 4,
            HttpHeader::Connection => 5,
            HttpHeader::ContentType => 6,
            HttpHeader::ContentLength => 7,
            HttpHeader::Cookie => 8,
            HttpHeader::Cookie2 => 9,
            HttpHeader::Host => 10,
            HttpHeader::Pragma => 11,
            HttpHeader::Referer => 12,
            HttpHeader::UserAgent => 13,
            HttpHeader::CacheControl => 14,
            HttpHeader::IfModifiedSince => 15,
            HttpHeader::IfMatch => 16,
            HttpHeader::IfNoneMatch => 17,
            HttpHeader::IfRange => 18,
            HttpHeader::IfUnmodifiedSince => 19,
            HttpHeader::KeepAlive => 20,
            HttpHeader::Range => 21,
            HttpHeader::XForwardedFor => 22,
            HttpHeader::Via => 23,
            HttpHeader::TransferEncoding => 24,
        }
    }
}

pub struct CommonHttpHeadersIndex {
    header_length: [u16; HttpHeader::VARIANTS_COUNT],
    header_offset: [u32; HttpHeader::VARIANTS_COUNT],
}

impl CommonHttpHeadersIndex {
    pub fn new() -> Self {
        Self {
            header_length: [0; HttpHeader::VARIANTS_COUNT],
            header_offset: [0; HttpHeader::VARIANTS_COUNT],
        }
    }

    pub fn set_header(&mut self, name: HttpHeader, length: u16, offset: u32) {
        let index = name as usize;
        self.header_length[index] = length;
        self.header_offset[index] = offset;
    }
}

pub struct UnknownHttpHeader {
    name_offset: u32,
    name_length: u32,
    value_offset: u32,
    value_length: u32,
}

// pub struct ResponseInfo {
//     headers_count: u32,
//     status: u32,
// }

// pub struct ResponseHeader {
//     packet_header: PacketHeader,
//     response_info: ResponseInfo,
// }
