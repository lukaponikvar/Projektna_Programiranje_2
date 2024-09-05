use crate::{functions::our_sequences::our_sequences, structs::sequences::SequenceSyntax};

/// Funkcija preveri, ali najin strežnik poseduje iskano zaporedje.
pub fn expected(syn: &SequenceSyntax) -> bool {
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
