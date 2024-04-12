use crate::{PacketHeader, RequestHeader};
use bytes::{BufMut, Bytes, BytesMut};

pub struct Request {
    packet_header: PacketHeader,
    request_header: RequestHeader,
}

impl Request {
    pub fn new() -> Self {
        Self {
            packet_header: PacketHeader::default(),
            request_header: RequestHeader::default(),
        }
    }

    pub fn into_bytes(&self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.put::<Bytes>(self.packet_header.into());
        buffer.put::<Bytes>(self.request_header.into());
        buffer.into()
    }
}
