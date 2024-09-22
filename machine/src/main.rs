pub mod communication;
pub mod functions;
pub mod mathematical_functions;
pub mod sequences;
pub mod structs;

use communication::{log_in::log_in, user_sequences::user_sequences, users::users};
use functions::{
    get_ip::get_ip,
    get_ports::{get_port, get_register_port},
    handle::handle,
};
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use std::{env, net::SocketAddr};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (register_ip, register_port, generator_ip, port) = match args.len() {
        1 => ([127, 0, 0, 1], 7878, [127, 0, 0, 1], 9000),
        2 => (get_ip(&args[1]), 7878, [127, 0, 0, 1], 9000),
        3 => (
            get_ip(&args[1]),
            get_register_port(&args[1]),
            get_ip(&args[2]),
            9000,
        ),
        4 => (
            get_ip(&args[1]),
            get_register_port(&args[1]),
            get_ip(&args[2]),
            get_port(&args[3]),
        ),
        _ => panic!("Too many arguments."),
    };

    let generator_address: SocketAddr = (generator_ip, port).into();

    log_in(register_ip, register_port, port).await;
    let a = users(register_ip, register_port).await;
    println!("{:#?}", a);
    let b = user_sequences(register_ip, register_port).await;
    println!("zaporedja: {:#?}", b);

    let listener = TcpListener::bind(generator_address).await?;
    println!("Listening on http://{}", generator_address);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| async move {
                handle(register_ip, register_port, port, req).await
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

//TODO: zrihtat main in v terminal ne oddajat brezveze printov