pub mod communication;
pub mod expressions;
pub mod functions;
pub mod mathematical_functions;
pub mod sequences;
pub mod structs;

// use communication::get_vector::get_vector;
use communication::log_in::{log_in, make_project};
use communication::other::{collect_body, empty, full};
use communication::user_sequences::user_sequences;
use communication::users::users;
use functions::eval::eval;
use functions::get_ip::get_ip;
use functions::get_name::get_name;
use functions::get_port::get_port;
use functions::our_sequences::our_sequences;
// use structs::range::Range;
use structs::sequences::{SequenceRequest, SequenceSyntax};

use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Error, Method, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (register_ip, generator_ip, port) = match args.len() {
        1 => ([127, 0, 0, 1], [127, 0, 0, 1], 9000),
        2 => (get_ip(&args[1]), [127, 0, 0, 1], 9000),
        3 => (get_ip(&args[1]), get_ip(&args[2]), 9000),
        4 => (get_ip(&args[1]), get_ip(&args[2]), get_port(&args[3])),
        _ => panic!("Too many arguments."),
    };

    let generator_address: SocketAddr = (generator_ip, port).into();

    let a = users(register_ip).await;
    println!("{:#?}", a);

    log_in(register_ip, port).await;

    let b = user_sequences(register_ip).await;
    println!("zaporedja: {:#?}", b);

    // let a = get_vector(
    //     Range {
    //         from: 0,
    //         to: 10,
    //         step: 1,
    //     },
    //     SequenceSyntax {
    //         name: "Arithmetic".to_string(),
    //         parameters: vec![2.0, 7.0],
    //         sequences: Vec::new(),
    //     },
    // )
    // .await;
    // println!("lastnik: {:#?}", a);

    let listener = TcpListener::bind(generator_address).await?;
    println!("Listening on http://{}", generator_address);

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
                        serde_json::to_string(&make_project(port)).unwrap(),
                    ))),
                    (&Method::GET, "/sequence") => {
                        let sequences = our_sequences();
                        Ok(Response::new(full(
                            serde_json::to_string(&sequences).unwrap(),
                        )))
                    }
                    (&Method::POST, r) => {
                        // let seqs = sequences();
                        // let _sequence: Option<&SequenceInfo> = seqs
                        //     .iter()
                        //     .find(|&x| ("/sequence/".to_string() + &x.name) == r);
                        let ime = r.to_string();
                        let body = collect_body(req).await?;
                        let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                        let range = request.range;
                        let s = eval(SequenceSyntax {
                            name: get_name(&ime),
                            parameters: request.parameters,
                            sequences: request.sequences,
                        })
                        .await;
                        // println!("khjn{}", (*s).sequences.len());
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
