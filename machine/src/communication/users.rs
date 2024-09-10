use super::get_and_post::send_get;
use crate::structs::{custom_error::CustomError, project::Project};

///Funkcija vrne seznam uporabnikov na glavnem strežniku.
pub async fn users(register_ip: [u8; 4]) -> Result<Vec<Project>, CustomError> {
    let response_as_result = send_get(&format!(
        "http://{}.{}.{}.{}:7878/generator",
        register_ip[0], register_ip[1], register_ip[2], register_ip[3]
    ))
    .await;
    let response = match response_as_result {
        Ok(s) => s,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    match serde_json::from_str(&response) {
        Ok(s) => return Ok(s),
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
}
