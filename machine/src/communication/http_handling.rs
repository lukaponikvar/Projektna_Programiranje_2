use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{
    body::{Body, Incoming},
    Error, Request, Response, StatusCode,
};

use crate::structs::custom_error::CustomError;

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

///Funkcija iz `html` zahteve izlušči in vrne telo.
///
/// ## Errors
/// V primeru napake, jo vrne.
pub async fn collect_body(req: Request<Incoming>) -> Result<String, CustomError> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 64 {
        return Err(CustomError::new("Body too big".to_string()));
    }

    let whole_body = match req.collect().await {
        Ok(m) => m.to_bytes(),
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    match std::str::from_utf8(&whole_body) {
        Ok(b) => Ok(b.to_string()),
        Err(e) => Err(CustomError::new(e.to_string())),
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

//TODO: Tole zgorej pa enkrat drugič:)

/// The function signals the client that the server cannot return the requested page.
///
///The `404 (Not Found)` client error response status code indicates that the server cannot find the requested resource.
pub fn create_404() -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    let mut not_found = Response::new(empty());
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

/// The function signals the client that an error had been detected on their behalf.
///
/// The `400 (Bad Request)` status code indicates that the server cannot or will not process the request
///  due to something that is perceived to be a client error.
pub fn create_400<T: Into<Bytes>>(str: T) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    let mut bad_request = Response::new(full(str));
    *bad_request.status_mut() = StatusCode::BAD_REQUEST;
    Ok(bad_request)
}

/// The function signals the client that an error had been detected on their behalf.
///
/// The `400 (Bad Request)` status code indicates that the server cannot or will not process the request
///  due to something that is perceived to be a client error.
pub fn create_200<T: Into<Bytes>>(str: T) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    let ok = Response::new(full(str));
    Ok(ok)
}

/// The function signals the client that an error had been detected on their behalf.
///
/// The `500 (Internal server error)` status code indicates that the server cannot or will not process the request
///  due to something that is perceived to be a client error.
pub fn create_500<T: Into<Bytes>>(str: T) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    let mut server_error = Response::new(full(str));
    *server_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    Ok(server_error)
}
