use super::our_sequences::our_sequences;
use crate::structs::sequences::SequenceSyntax;

/// Checks if the searched sequence `syn` is one of sequences that belong to our server or not.
fn expected(syn: &SequenceSyntax) -> bool {
    let our_sequences = our_sequences();
    let index = our_sequences.iter().position(|seq| {
        seq.name == syn.name
            && seq.parameters as usize == syn.parameters.len()
            && seq.sequences as usize == syn.sequences.len()
    });
    match index {
        Some(_) => true,
        None => false,
    }
}

/// Returns `true` if all sequences (even nested ones, given in the `syn.sequences`) belong to our server, 
/// and `false` if there exists at least one sequence that does not belong to our server.
pub fn check_sequences(syn: &SequenceSyntax) -> bool {
    let mut result = expected(&syn);
    for seq in &syn.sequences {
        result = result && check_sequences(&seq);
        if result == false {
            break;
        }
    }
    result
}
