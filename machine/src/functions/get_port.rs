use regex::Regex;

///Funkcija vzame niz oblike `"XXXX"` in vrne `PORT`.
pub fn get_port(string: &String) -> u16 {
    let reg = Regex::new(r"^(\d+)$").unwrap();
    let findings = reg.captures(&string).unwrap();
    let port = findings
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
        .parse()
        .expect("Failed to parse string to integer");
    port
}
