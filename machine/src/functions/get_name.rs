use regex::Regex;

///Funkcija vzame niz oblike `"/sequence/{ime_zaporedja}"` in vrne `ime_zaporedja`
pub fn get_name(string: &String) -> String {
    let reg = Regex::new(r"/sequence/(.+)$").unwrap();
    reg.captures(&string)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
}
