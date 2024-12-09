mod to_bin;
mod request;

use bytes::{BufMut, BytesMut};
use derive_builder::Builder;
pub use crate::message::to_bin::ToBeBytes;

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct Response {
    pub message_size: i32,
}

impl ToBeBytes for Response {
    fn to_be_bytes(&self) -> Vec<u8> {
        let mut response_bytes = BytesMut::new();
        response_bytes.put_i32(0);
        response_bytes.put_i32(7);
        response_bytes.to_vec()
    }
}