use static_init::dynamic;
use std::env;

// static MAX_HEADER_LENGTH_DEFAULT: u16 = 65535;
// static MAX_PACKET_LENGTH_DEFAULT: u16 = 16384;
// static RESP_HTTP_HEADER_MAX_DEFAULT: u16 = 4096;
// static PACKET_HEADER_LEN_DEFAULT: u8 = 8;
static ENDIAN_DEFAULT: u8 = 0; // 0 is little endian, 1 is big endian

// #[dynamic]
// pub(crate) static MAX_HEADER_LENGTH: u16 = env::var("MAX_HEADER_LENGTH")
//     .map_or(MAX_HEADER_LENGTH_DEFAULT, |value| {
//         value.parse().unwrap_or(MAX_HEADER_LENGTH_DEFAULT)
//     });

// #[dynamic]
// pub(crate) static MAX_PACKET_LENGTH: u16 = env::var("MAX_PACKET_LENGTH")
//     .map_or(MAX_PACKET_LENGTH_DEFAULT, |value| {
//         value.parse().unwrap_or(MAX_PACKET_LENGTH_DEFAULT)
//     });

// #[dynamic]
// pub(crate) static RESP_HTTP_HEADER_MAX: u16 = env::var("RESP_HTTP_HEADER_MAX")
//     .map_or(RESP_HTTP_HEADER_MAX_DEFAULT, |value| {
//         value.parse().unwrap_or(RESP_HTTP_HEADER_MAX_DEFAULT)
//     });

// #[dynamic]
// pub(crate) static PACKET_HEADER_LEN: u8 = env::var("PACKET_HEADER_LEN")
//     .map_or(PACKET_HEADER_LEN_DEFAULT, |value| {
//         value.parse().unwrap_or(PACKET_HEADER_LEN_DEFAULT)
//     });

#[dynamic]
pub(crate) static ENDIAN: u8 = env::var("ENDIAN").map_or(ENDIAN_DEFAULT, |value| {
    value.parse().unwrap_or(ENDIAN_DEFAULT)
});
