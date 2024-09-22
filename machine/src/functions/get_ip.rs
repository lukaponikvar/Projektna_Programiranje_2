use regex::Regex;

/// Makes `ip` address from a string of the form `"X.X.X.X"`, where `X` represents a digit.
/// 
/// ## Errors
/// In case of any error the standard `127.0.0.1` `ip` address is used.
pub fn get_ip(string: &String) -> [u8; 4] {
    let reg = match Regex::new(r"^(\d*)\.(\d*)\.(\d*)\.(\d*)$") {
        Ok(r) => r,
        Err(_) => return [127, 0, 0, 1],
    };
    let findings = match reg.captures(&string) {
        Some(f) => f,
        None => return [127, 0, 0, 1],
    };
    let mut ips = Vec::new();
    for i in 1..5 {
        let our_match = match findings.get(i) {
            Some(m) => m,
            None => return [127, 0, 0, 1],
        };
        let ip = match our_match.as_str().to_string().parse() {
            Ok(r) => r,
            Err(_) => return [127, 0, 0, 1],
        };
        ips.push(ip);
    }
    [ips[0], ips[1], ips[2], ips[3]]
}
