use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub ip: String,
    pub port: u16,
}