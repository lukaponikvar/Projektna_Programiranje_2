use crate::communication::get_and_post::send_post;
use crate::structs::project::Project;

pub fn get_project(port: u16) -> Project {
    return Project {
        name: "Luka & Anara".to_string(),
        ip: "127.0.0.1".to_string(),
        port,
    };
}

pub async fn log_in(port: u16) {
    let result = send_post(
        "http://127.0.0.1:7878/project".to_string(),
        serde_json::to_string(&get_project(port)).unwrap(),
    )
    .await;
    match result {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
