use crate::{errors::PacketHeaderError, statics::ENDIAN};
use bytes::{BufMut, Bytes, BytesMut};
use std::mem::size_of;

#[derive(Clone, Debug, Copy)]
#[repr(u8)]
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
}

impl TryFrom<u8> for PacketType {
    type Error = PacketHeaderError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::BeginRequest),
            2 => Ok(Self::AbortRequest),
            3 => Ok(Self::ResponseHeader),
            4 => Ok(Self::ResponseStream),
            5 => Ok(Self::ResponseEnd),
            6 => Ok(Self::StderrStream),
            7 => Ok(Self::RequestReceived),
            8 => Ok(Self::ConnectionClose),
            9 => Ok(Self::InternalError),
            _ => Err(Self::Error::UnknownPacketType),
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
        }
    }
}

#[derive(Clone, Debug, Copy)]
#[repr(u8)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

impl TryFrom<u8> for Endianness {
    type Error = PacketHeaderError;

    fn try_from(value: u8) -> Result<Self, PacketHeaderError> {
        match value {
            0 => Ok(Self::LittleEndian),
            1 => Ok(Self::BigEndian),
            _ => Err(PacketHeaderError::InvalidEndianness),
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

#[derive(Clone, Debug, Copy)]
pub struct PacketHeader {
    version_b0: u8,
    version_b1: u8,
    packet_type: PacketType,
    endianness: Endianness,
    packet_length: u32, // Does not include body length.
}

impl PacketHeader {
    fn new(
        version_b0: u8,
        version_b1: u8,
        packet_type: PacketType,
        endianness: Endianness,
        packet_length: u32,
    ) -> Self {
        Self {
            version_b0,
            version_b1,
            packet_type,
            endianness,
            packet_length,
        }
    }

    pub fn version_b0(&mut self, version: u8) -> &Self {
        self.version_b0 = version;
        self
    }

    pub fn version_b1(&mut self, version: u8) -> &Self {
        self.version_b1 = version;
        self
    }

    pub fn packet_type(&mut self, packet_type: PacketType) -> &Self {
        self.packet_type = packet_type;
        self
    }

    pub fn endianness(&mut self, endianness: Endianness) -> &Self {
        self.endianness = endianness;
        self
    }

    pub fn packet_length(&mut self, packet_length: u32) -> &Self {
        self.packet_length = packet_length;
        self
    }

    pub fn len(&self) -> usize {
        size_of::<u8>() * 4 + size_of::<u32>()
    }
}

impl Default for PacketHeader {
    fn default() -> Self {
        Self::new(
            b'L',
            b'S',
            PacketType::BeginRequest,
            Endianness::try_from(*ENDIAN).unwrap_or(Endianness::LittleEndian),
            8,
        )
    }
}

impl Into<Bytes> for PacketHeader {
    fn into(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.len());

        buffer.put_u8(self.version_b0);
        buffer.put_u8(self.version_b1);
        buffer.put_u8(self.packet_type.into());
        buffer.put_u8(self.endianness.into());
        buffer.put_u32(self.packet_length);

        buffer.into()
    }
}
