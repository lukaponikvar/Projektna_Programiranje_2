use crate::structs::sequences::SequenceInfo;

pub fn sequences() -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Arithmetic".to_string(),
        description: "Arithmetic sequence which takes two parameters: start and step.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Constant".to_string(),
        description: "Constant sequence with a single parameter: value.".to_string(),
        parameters: 1,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Geometric".to_string(),
        description: "Geometric sequence with two parameters: start and quotient.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Sum".to_string(),
        description: "Sum sequence with two parameters: start and quotient.".to_string(),
        parameters: 0,
        sequences: 1,
    });
    sequences
}
