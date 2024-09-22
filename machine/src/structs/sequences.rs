use super::range::Range;
use serde::{Deserialize, Serialize};

/// `SequenceSyntax` describes a sequence through `name`, `parameters` and `sequences`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SequenceSyntax {
    pub name: String,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

/// `SequenceRequest` is used when sending a request for a sequence with `range`, `parameters` and `sequences`.
#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceRequest {
    pub range: Range,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

/// `SequenceInfo` describes a sequence through `name`, `parameters` and `sequences` with additional `description`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SequenceInfo {
    pub name: String,
    pub description: String,
    pub parameters: u64,
    pub sequences: u64,
}
