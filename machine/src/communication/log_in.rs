use super::get_and_post::send_post;
use crate::structs::project::Project;

/// Creates a `Project` with data: `name`, `ip` and `port`.
pub fn make_project(port: u16) -> Project {
    return Project {
        name: "Luka & Anara".to_string(),
        ip: "127.0.0.1".to_string(),
        port,
    };
}

// TODO: a bi mejbi nrdila zgornjo funkcijo tko, da vzame name, ip, port kot stvar in deluje bolj splošno in ne tok hard written v kodi?

/// Logs the register into the main server.
/// 
/// ## Errors
/// In case of errors, they are reported.
pub async fn log_in(register_ip: [u8; 4], register_port: u16, port: u16) {
    let body = match serde_json::to_string(&make_project(port)) {
        Ok(b) => b,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };
    let result = send_post(
        &format!(
            "http://{}.{}.{}.{}:{}/generator",
            register_ip[0], register_ip[1], register_ip[2], register_ip[3], register_port
        ),
        body,
    )
    .await;
    match result {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e.to_string());
        }
    }
}

//TODO: Preglej, katere funkcije ne potrebujejo zares lastništva in jim daj raje referenco
//TODO: preveri ali so poslali pravi vektor
//TODO: Koda naj deluje z ali brez / na koncu url-ja
//TODO: Javljanje napak naj bo koristno! (sledi navodilom na spletni strani) - tole morva še poštimat!
