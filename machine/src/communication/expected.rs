use crate::{functions::our_sequences::our_sequences, structs::sequences::SequenceRequest};

/// Funkcija preveri, ali najin strežnik poseduje iskano zaporedje.
pub fn expected(request: &SequenceRequest, name: &String) -> bool {
    let our_sequences = our_sequences();
    let index = our_sequences.iter().position(|seq| {
        seq.name == *name
            && seq.parameters as usize == request.parameters.len()
            && seq.sequences as usize == request.sequences.len()
    });
    match index {
        Some(_) => true,
        None => false,
    }
}
