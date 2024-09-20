use serde::{Deserialize, Serialize};

/// `Project` that can be logged in the `main server` and holds information about the user: `name`, `ip` and `port`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub ip: String,
    pub port: u16,
}
