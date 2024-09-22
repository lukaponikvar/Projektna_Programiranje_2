use serde::{Deserialize, Serialize};

/// `Range` holds the information about desired output of a sequence: on which element it starts, on which it stops and what step is made between.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Range {
    pub from: u64,
    pub to: u64,
    pub step: u64,
}
