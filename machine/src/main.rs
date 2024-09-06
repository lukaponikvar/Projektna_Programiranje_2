pub mod communication;
pub mod expressions;
pub mod functions;
pub mod mathematical_functions;
pub mod sequences;
pub mod structs;

// use communication::get_vector::get_vector;
use communication::{
    expected::expected,
    http_handling::{collect_body, create_200, create_400, create_404},
    log_in::{log_in, make_project},
    user_sequences::user_sequences,
    users::users,
};
use functions::{
    check_sequences::check_sequences, get_foreign_vector::get_foreign_vector, get_ip::get_ip,
    get_name::get_name, get_port::get_port, get_vector::get_vector, our_sequences::our_sequences,
};
// use structs::range::Range;
use structs::sequences::{SequenceRequest, SequenceSyntax};

use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Method;
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

    // let create_404 = || {
    //     let mut not_found = Response::new(empty());
    //     *not_found.status_mut() = StatusCode::NOT_FOUND;
    //     Ok(not_found)
    // };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| async move {
                match (req.method(), req.uri().path()) {
                    (&Method::GET, "/ping") => {
                        create_200(serde_json::to_string(&make_project(port)).unwrap())
                    }
                    (&Method::GET, "/sequence") => {
                        create_200(serde_json::to_string(&our_sequences()).unwrap())
                    }
                    (&Method::POST, r) => {
                        let path = r.to_string();
                        let body = collect_body(req).await?;
                        let request: SequenceRequest = serde_json::from_str(&body).unwrap();
                        let name = get_name(&path);
                        let syn = SequenceSyntax {
                            name,
                            parameters: request.parameters,
                            sequences: request.sequences,
                        };
                        match expected(&syn) {
                            Ok(_) => {
                                println!("Je pričakovano");
                                if check_sequences(&syn) {
                                    create_200(
                                        serde_json::to_string(
                                            &(get_vector(syn, &request.range).await),
                                        )
                                        .expect("Tule sem"),
                                    )
                                } else {
                                    let (projects, all_sequences) =
                                        user_sequences(register_ip).await;
                                    create_200(
                                        serde_json::to_string(
                                            &(get_foreign_vector(
                                                syn,
                                                &request.range,
                                                projects,
                                                all_sequences,
                                            )
                                            .await),
                                        )
                                        .expect("Tule sem"),
                                    )
                                }
                            }
                            Err(e) => create_400(e.message),
                        }
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
