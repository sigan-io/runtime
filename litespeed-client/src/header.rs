pub enum PacketHeaderType {
    BeginRequest = 1,
    AbortRequest = 2,
    ResponseHeader = 3,
    ResponseStream = 4,
    ResponseEnd = 5,
    StderrStream = 6,
    RequestReceived = 7,
    ConnectionClose = 8,
    InternalError = 9,
}

pub struct PacketHeader {
    version_b0: u8,
    version_b1: u8,
    packet_type: u8,
    endianness: u8,
    packet_len: PacketLen,
}

impl PacketHeader {
  pub fn new() -> Self {
    viersion_b0 
  }
}

enum PacketLen {
    Length(i32),
    Bytes([u8; 4]),
}
