use crate::structs::custom_error::CustomError;
use regex::Regex;

///Funkcija vzame niz oblike `"/sequence/{ime_zaporedja}"` in vrne `ime_zaporedja`
pub fn get_name(string: &String) -> Result<String, CustomError> {
    let reg = match Regex::new(r"/sequence/(.+)") {
        Ok(r) => r,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    let name = match reg.captures(&string) {
        Some(n) => n,
        None => return Err(CustomError::new("No sequence name found.".to_string())),
    };
    match name.get(1) {
        Some(n) => Ok(n.as_str().to_string()),
        None => return Err(CustomError::new("No sequence name found.".to_string())),
    }
}
