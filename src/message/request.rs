use bytes::{BufMut, BytesMut};
use derive_builder::Builder;
use crate::message::to_bin::ToBeBytes;

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct Request {
    pub correlation_id: i32,
}

impl ToBeBytes for Request {
    fn to_be_bytes(&self) -> Vec<u8> {
        let mut request_bytes = BytesMut::new();
        let correlation_id_bytes = self.correlation_id.to_be_bytes();
        request_bytes.put(correlation_id_bytes.as_slice());
        request_bytes.to_vec()
    }
}