use regex::Regex;

///Funkcija vzame niz oblike `"/sequence/{ime_zaporedja}"` in vrne `ime_zaporedja`
pub fn get_name(string: String) -> String {
    let re = Regex::new(r"/sequence/(.+)$").unwrap();
    let result = re.find(&string);
    result.unwrap().as_str().to_string()
}
