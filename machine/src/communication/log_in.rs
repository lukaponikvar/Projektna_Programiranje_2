use crate::communication::get_and_post::send_post;
use crate::structs::project::Project;

///Funkcija ustvari `Project` z najinimi podatki.
pub fn make_project(port: u16) -> Project {
    return Project {
        name: "Luka & Anara".to_string(),
        ip: "127.0.0.1".to_string(),
        port,
    };
}

///Funkcija naju prijavi v glavni strežnik.
pub async fn log_in(register_ip: [u8; 4], port: u16) {
    let result = send_post(
        format!(
            "http://{}.{}.{}.{}:7878/generator",
            register_ip[0], register_ip[1], register_ip[2], register_ip[3]
        ),
        serde_json::to_string(&make_project(port)).unwrap(),
    )
    .await;
    match result {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}

//TODO: Javljanje napak naj bo koristno! (sledi navodilom na spletni strani)
//TODO: Jezik komunikacije naj bo angleščina
//TODO: Popravi, da vrača zaporedje dolgo: (to - from) / step
//TODO: Koda naj deluje z ali brez / na koncu url-ja
//TODO: Preglej, katere funkcije ne potrebujejo zares lastništva in jim daj raje referenco
