use crate::communication::get_and_post::send_get;
use crate::structs::project::Project;

///Funkcija vrne seznam uporabnikov na glavnem strežniku.
pub async fn users(register_ip: [u8; 4]) -> Vec<Project> {
    let response_as_result = send_get(&format!(
        "http://{}.{}.{}.{}:7878/generator",
        register_ip[0], register_ip[1], register_ip[2], register_ip[3]
    ))
    .await;
    let response = match response_as_result {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    let projects = match serde_json::from_str(&response) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    projects
}
