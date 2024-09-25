use crate::structs::sequences::SequenceInfo;

/// Returns a vector of sequences that our server can produce.
pub fn our_sequences() -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: "Arithmetic".to_string(),
        description: "Arithmetic sequence which takes two parameters: a start and a step."
            .to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Average".to_string(),
        description:
            "A sequence that takes two sequences and calculates their average term by term."
                .to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Constant".to_string(),
        description: "Constant sequence with a single parameter: a value.".to_string(),
        parameters: 1,
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
        name: "Fibonacci".to_string(),
        description: "Fibonacci sequence starting with given parameters `zeroth` and `first`."
            .to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Geometric".to_string(),
        description: "Geometric sequence with two parameters: a start and a quotient.".to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "LinearCombination".to_string(),
        description: "Linear combination of two sequences `a` and `b` as given by: `(x*a_i + y*b_i + w)_i` where `x`, `y` and `w` are the three parameters.".to_string(),
        parameters: 3,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Max".to_string(),
        description: "A sequence that takes two sequences and uses their maximum term by term."
            .to_string(),
        parameters: 0,
        sequences: 2,
    });
    sequences.push(SequenceInfo {
        name: "Min".to_string(),
        description: "A sequence that takes two sequences and uses their minimum term by term."
            .to_string(),
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
    sequences.push(SequenceInfo {
        name: "Random".to_string(),
        description: "A sequence that takes two arguments: a and b, and generates random values in a specified range [min(a, b), max(a,b)). If the range is empty it returns all a's."
            .to_string(),
        parameters: 2,
        sequences: 0,
    });
    sequences.push(SequenceInfo {
        name: "Sum".to_string(),
        description: "A sequence that takes two sequences and adds them term by term.".to_string(),
        parameters: 0,
        sequences: 2,
    });

    sequences
}
