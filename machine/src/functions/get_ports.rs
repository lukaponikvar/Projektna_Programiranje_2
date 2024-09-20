use regex::Regex;

fn get_port_with_pattern(string: &String, reg: Regex) -> u16 {
    let findings = match reg.captures(&string) {
        Some(f) => f,
        None => return 9000,
    };
    let our_match = match findings.get(1) {
        Some(m) => m,
        None => return 9000,
    };
    let port = match our_match.as_str().to_string().parse() {
        Ok(r) => r,
        Err(_) => return 9000,
    };
    port
}

/// Returns `PORT` from a string of the form `"XXXX"`, where `X` represents a digit.
/// 
/// ## Errors
/// In case of any error the standard `9000` `PORT` address is used.
pub fn get_port(string: &String) -> u16 {
    let reg = match Regex::new(r"^(\d+)$") {
        Ok(r) => r,
        Err(_) => return 9000,
    };
    get_port_with_pattern(string, reg)
}

/// Returns `PORT` from a string of the form `"XXXX"`, where `X` represents a digit.
/// 
/// ## Errors
/// In case of any error the standard `9000` `PORT` address is used.
pub fn get_register_port(string: &String) -> u16 {
    let reg = match Regex::new(r":(\d+)$") {
        Ok(r) => r,
        Err(_) => return 9000,
    };
    get_port_with_pattern(string, reg)
}

//TODO: verjetno se morta get_port in get_register_port razlikovat?