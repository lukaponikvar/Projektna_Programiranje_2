pub mod expressions;
pub mod functions;
pub mod mathematical_functions;
pub mod sequences;
pub mod structs;

use functions::eval::evall;
use functions::our_sequences::sequences;
use structs::project::Project;
use structs::sequences::{SequenceInfo, SequenceRequest, SequenceSyntax};

// use core::panicking::panic;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Error;
use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const PORT: u16 = 12345;

fn get_project() -> Project {
    return Project {
        name: "Luka & Anara".to_string(),
        ip: "127.0.0.1".to_string(),
        port: PORT,
    };
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
async fn collect_body(req: Request<Incoming>) -> Result<String, hyper::Error> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 64 {
        panic!("Body too big");
    }

    let whole_body = req.collect().await?.to_bytes();
    let whole_body = std::str::from_utf8(&whole_body).unwrap().to_string();
    return Ok(whole_body);
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

async fn send_post(url: String, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url).body(body).send().await?.text().await?;
    return Ok(res);
}

async fn send_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.text().await?;
    return Ok(res);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([127, 0, 0, 1], PORT).into();

    let b = send_get("http://127.0.0.1:7878/project".to_string()).await?;
    println!("HERE {}", b);

    let b = send_post(
        "http://127.0.0.1:7878/project".to_string(),
        serde_json::to_string(&get_project()).unwrap(),
    )
    .await?;
    println!("HERE {}", b);

    let b = send_get("http://127.0.0.1:7878".to_string()).await?;
    println!("HERE {}", b);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let create_404 = || {
        let mut not_found = Response::new(empty());
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        Ok(not_found)
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| async move {
                match (req.method(), req.uri().path()) {
                    (&Method::GET, "/ping") => Ok::<_, Error>(Response::new(full(
                        serde_json::to_string(&get_project()).unwrap(),
                    ))),
                    (&Method::GET, "/sequence") => {
                        let sequences = sequences();
                        Ok(Response::new(full(
                            serde_json::to_string(&sequences).unwrap(),
                        )))
                    }
                    (&Method::POST, r) => {
                        let seqs = sequences();
                        let _sequence: Option<&SequenceInfo> = seqs
                            .iter()
                            .find(|&x| ("/sequence/".to_string() + &x.name) == r);
                        let ime = r.to_string();
                        let body = collect_body(req).await?;
                        let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                        let range = request.range;
                        let s = evall(SequenceSyntax {
                            name: ime,
                            parameters: request.parameters,
                            sequences: request.sequences,
                        })
                        .await;
                        let neki = (*s).range(range);
                        Ok(Response::new(full(serde_json::to_string(&neki).unwrap())))
                        // match sequence {
                        // None => create_404(),
                        // Some(s) if *s.name == "Arithmetic".to_string() => {
                        //     let seq = sequences::arithmetic::Arithmetic::new(
                        //         request.parameters[0],
                        //         request.parameters[1],
                        //     );
                        //     let neki = &seq.range(range);
                        //     Ok(Response::new(full(serde_json::to_string(neki).unwrap())))
                        // }
                        // Some(s) if *s.name == "Constant".to_string() => {
                        //     let seq = sequences::constant::Constant::new(request.parameters[0]);
                        //     let neki = &seq.range(range).await;
                        //     Ok(Response::new(full(serde_json::to_string(neki).unwrap())))
                        // }
                        // Some(s) if *s.name == "Geometric".to_string() => {
                        //     let seq = sequences::geometric::Geometric::new(
                        //         request.parameters[0],
                        //         request.parameters[1],
                        //     );
                        //     let neki = &seq.range(range).await;
                        //     Ok(Response::new(full(serde_json::to_string(neki).unwrap())))
                        // }
                        // Some(s) if *s.name == "Sum".to_string() => {
                        //     let seq = sequences::sum::Sum::new(request.sequences);
                        //     let neki = &seq.range(range).await;
                        //     Ok(Response::new(full(serde_json::to_string(neki).unwrap())))
                        // }
                        // _ => panic!("Not implemented"),
                        // }
                    }

                    _ => create_404(),
                }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
