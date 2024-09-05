use regex::Regex;

///Funkcija vzame niz oblike `"X.X.X.X"` in vrne `ip` naslov.
pub fn get_ip(string: &String) -> [u8; 4] {
    let reg = Regex::new(r"^(\d*)\.(\d*)\.(\d*)\.(\d*)$").unwrap();
    let findings = reg.captures(&string).unwrap();
    let mut ip = Vec::new();
    for i in 1..5 {
        ip.push(
            findings
                .get(i)
                .unwrap()
                .as_str()
                .to_string()
                .parse()
                .expect("Failed to parse string to integer"),
        );
    }
    [ip[0], ip[1], ip[2], ip[3]]
}
