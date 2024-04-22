#[derive(Debug, Clone, Copy)]
#[repr(u8)]

pub enum Role {
    Responder = 1,
    Authorizer,
    Filter,
}

pub enum RequestType {
    BeginRequest = 1,
    AbortRequest, // Not supported by PHP
    EndRequest,
    Params,
    Stdin,
    Stdout,
    Stderr,
    Data, // Not supported by PHP
    GetValues,
    GetValuesResult,
}

pub enum ProtocolStatus {
    RequestComplete = 0,
    CantMultiplexConnection,
    Overloaded,
    UnknownRole,
}

pub struct Header {
    version: u8,
    r#type: u8,
    request_id: u16,
    content_length: u16,
    padding_length: u8,
    reserved: u8, // Always initialize to 0
}

pub struct BeginRequest {
    role: u16,
    flags: u8,
    reserved: [u8; 5], // Always initialize to 0
}

pub struct BeginRequestRecord {
    header: Header,
    body: BeginRequest,
}

pub struct EndRequest {
    app_status: u32,
    protocol_status: u8,
    reserved: [u8; 3], // Always initialize to 0
}

pub struct EndRequestRecord {
    header: Header,
    body: EndRequest,
}

