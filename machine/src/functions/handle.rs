use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{body::Incoming, Error, Method, Request, Response};

use crate::communication::{
    http_handling::{create_200, create_400, create_404},
    log_in::make_project,
};

use super::{eval::eval, our_sequences::our_sequences};

pub async fn handle(
    register_ip: [u8; 4],
    register_port: u16,
    port: u16,
    request: Request<Incoming>,
) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/ping") | (&Method::GET, "/ping/") => {
            match serde_json::to_string(&make_project(port)) {
                Ok(s) => create_200(s),
                Err(e) => create_400(e.to_string()),
            }
        }
        (&Method::GET, "/sequence") | (&Method::GET, "/sequence/") => {
            match serde_json::to_string(&our_sequences()) {
                Ok(s) => create_200(s),
                Err(e) => create_400(e.to_string()),
            }
        }
        (&Method::POST, _) => eval(register_ip, register_port, request).await,
        _ => create_404(),
    }
}
