use derive_builder::Builder;

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct Response {
    pub message_size: i32,
}

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct Request {
    pub correlation_id: i32,
}