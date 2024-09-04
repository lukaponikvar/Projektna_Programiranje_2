use super::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SequenceSyntax {
    pub name: String,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceRequest {
    pub range: Range,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceInfo {
    pub name: String,
    pub description: String,
    pub parameters: u64,
    pub sequences: u64,
}
