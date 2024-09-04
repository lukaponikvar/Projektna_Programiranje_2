pub mod communication;
pub mod expressions;
pub mod functions;
pub mod mathematical_functions;
pub mod sequences;
pub mod structs;

use communication::log_in::{get_project, log_in};
use communication::other::{collect_body, empty, full};
use communication::users::users;
use functions::eval::eval;
use functions::get_name::get_name;
use functions::our_sequences::sequences;
use structs::sequences::{SequenceInfo, SequenceRequest, SequenceSyntax};

use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Error, Method, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const PORT: u16 = 12345;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([127, 0, 0, 1], PORT).into();

    let b = users().await;
    println!("{:#?}", b);

    log_in(PORT).await;

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
                        serde_json::to_string(&get_project(PORT)).unwrap(),
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
                        println!("to je ta{}", get_name(&ime));
                        let s = eval(SequenceSyntax {
                            name: get_name(&ime),
                            parameters: request.parameters,
                            sequences: request.sequences,
                        })
                        .await;
                        let neki = (*s).range(&range);
                        Ok(Response::new(full(serde_json::to_string(&neki).unwrap())))
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
