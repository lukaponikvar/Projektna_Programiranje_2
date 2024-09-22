use crate::structs::custom_error::CustomError;
use regex::Regex;

/// Returns `sequence_name` from a string of the form `"/sequence/{sequence_name}"`
///
/// ## Errors
/// In case there is no sequence name found, the "No sequence name found" error is reported.
pub fn get_name(string: &String) -> Result<String, CustomError> {
    let reg = match Regex::new(r"/sequence/(.+)/") {
        Ok(r) => r,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    let name = match reg.captures(&string) {
        Some(n) => n,
        None => return get_name_without_the_slash(string),
    };
    match name.get(1) {
        Some(n) => Ok(n.as_str().to_string()),
        None => return Err(CustomError::new("No sequence name found".to_string())),
    }
}

fn get_name_without_the_slash(string: &String) -> Result<String, CustomError> {
    let reg = match Regex::new(r"/sequence/(.+)") {
        Ok(r) => r,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    let name = match reg.captures(&string) {
        Some(n) => n,
        None => return Err(CustomError::new("No sequence name found".to_string())),
    };
    match name.get(1) {
        Some(n) => Ok(n.as_str().to_string()),
        None => return Err(CustomError::new("No sequence name found".to_string())),
    }
}
