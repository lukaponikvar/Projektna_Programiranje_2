use crate::structs::sequences::SequenceSyntax;

use super::our_sequences::our_sequences;

/// Funkcija preveri, ali najin strežnik poseduje iskano zaporedje.
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

///Funkcija preveri ali so vsa morebiti gnezdena zaporedja v najini lasti ali ne.
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
