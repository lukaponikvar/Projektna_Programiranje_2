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
pub async fn log_in(port: u16) {
    let result = send_post(
        "http://127.0.0.1:7878/generator".to_string(),
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
