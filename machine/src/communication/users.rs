use crate::communication::get_and_post::send_get;
use crate::structs::project::Project;

///Funkcija vrne seznam uporabnikov na glavnem strežniku.
pub async fn users() -> Vec<Project> {
    let response_as_result = send_get("http://127.0.0.1:7878/generator".to_string()).await;
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
