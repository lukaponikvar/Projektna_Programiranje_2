use regex::Regex;

///Funkcija vzame niz oblike `"XXXX"` in vrne `PORT`.
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

pub fn get_port(string: &String) -> u16 {
    let reg = match Regex::new(r"^(\d+)$") {
        Ok(r) => r,
        Err(_) => return 9000,
    };
    get_port_with_pattern(string, reg)
}

pub fn get_register_port(string: &String) -> u16 {
    let reg = match Regex::new(r":(\d+)$") {
        Ok(r) => r,
        Err(_) => return 9000,
    };
    get_port_with_pattern(string, reg)
}