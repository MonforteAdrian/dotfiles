use prost_test::*;

fn main() -> Result<(), prost::DecodeError> {
    let request = String::from("Hello, World!");

    let messages_request = create_message(request);
    let request_vector = serialize_message(&messages_request);

    let request_deserialized_result = match deserialize_message(&request_vector) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(e) => return Err(e),
    };
    println!("{:#?}", request_deserialized_result);
    Ok(())
}
