use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, Error, Request, Response};

use crate::{
    communication::{
        expected::expected,
        http_handling::{collect_body, create_200, create_400},
        user_sequences::user_sequences,
    },
    functions::{
        check_sequences::check_sequences, get_foreign_vector::get_foreign_vector,
        get_vector::get_vector,
    },
    structs::sequences::{SequenceRequest, SequenceSyntax},
};

use super::get_name::get_name;

pub async fn eval(
    register_ip: [u8; 4],
    register_port: u16,
    request: Request<Incoming>,
) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    let name = match get_name(&request.uri().path().to_string()) {
        Ok(n) => n,
        Err(e) => return create_400(e.message),
    };
    let body = match collect_body(request).await {
        Ok(b) => b,
        Err(e) => return create_400(e.message),
    };
    let request: SequenceRequest = match serde_json::from_str(&body) {
        Ok(s) => s,
        Err(e) => return create_400(e.to_string()),
    };
    let syn = SequenceSyntax {
        name,
        parameters: request.parameters,
        sequences: request.sequences,
    };
    match expected(&syn) {
        Ok(_) => {
            println!("Je pričakovano");
            if check_sequences(&syn) {
                let vector = match get_vector(&syn, &request.range).await {
                    Ok(v) => v,
                    Err(e) => return create_400(e.message),
                };
                match serde_json::to_string(&vector) {
                    Ok(s) => create_200(s),
                    Err(e) => create_400(e.to_string()),
                }
            } else {
                let (projects, all_sequences) = user_sequences(register_ip, register_port).await;
                let vector =
                    match get_foreign_vector(&syn, &request.range, &projects, &all_sequences).await
                    {
                        Ok(v) => v,
                        Err(e) => return create_400(e.message),
                    };
                match serde_json::to_string(&vector) {
                    Ok(s) => create_200(s),
                    Err(e) => create_400(e.to_string()),
                }
            }
        }
        Err(e) => create_400(e.message),
    }
}
