use super::get_and_post::send_get;
use crate::structs::{custom_error::CustomError, project::Project};

///Returns a vector of all users logged in the main server.
pub async fn users(register_ip: [u8; 4], register_port: u16) -> Result<Vec<Project>, CustomError> {
    let response_as_result = send_get(&format!(
        "http://{}.{}.{}.{}:{}/generator",
        register_ip[0], register_ip[1], register_ip[2], register_ip[3], register_port
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
