use crate::structs::sequences::SequenceInfo;

///Funkcija vrne seznam najinih zaporedij.
pub fn our_sequences() -> Vec<SequenceInfo> {
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
        name: "Drop".to_string(),
        description: "A sequence which takes a sequence and a shift parameter and is 
        equivalent to shifting that particular sequence by the given amount."
            .to_string(),
        parameters: 1,
        sequences: 1,
    });
    sequences.push(SequenceInfo {
        name: "Sum".to_string(),
        description: "A sequence that takes two sequences and adds them term by term.".to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Product".to_string(),
        description: "A sequence that takes two sequences and multiplies them term by term."
            .to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences
}
