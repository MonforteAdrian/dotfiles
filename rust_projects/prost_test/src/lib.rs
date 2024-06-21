use std::io::Cursor;

use prost::Message;

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub fn create_message(name: String) -> messages::HelloRequest {
    messages::HelloRequest { name }
}

pub fn serialize_message(hello: &messages::HelloRequest) -> Vec<u8> {
    let mut buf = Vec::with_capacity(hello.encoded_len());
    buf.reserve(hello.encoded_len());

    hello.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_message(buf: &[u8]) -> Result<messages::HelloRequest, prost::DecodeError> {
    messages::HelloRequest::decode(&mut Cursor::new(buf))
}
