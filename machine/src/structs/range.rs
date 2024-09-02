use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    pub from: u64,
    pub to: u64,
    pub step: u64,
}
