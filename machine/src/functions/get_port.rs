use regex::Regex;

///Funkcija vzame niz oblike `"XXXX"` in vrne `PORT`.
pub fn get_port(string: &String) -> u16 {
    let reg = match Regex::new(r"^(\d+)$") {
        Ok(r) => r,
        Err(_) => return 9000,
    };
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
