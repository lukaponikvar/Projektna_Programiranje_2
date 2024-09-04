use super::user_sequences::user_sequences;
use crate::structs::{project::Project, sequences::SequenceSyntax};

///Funkcija vrne lastnika (`project`) iskanega zaporedja (`SequenceSyntax`).
pub async fn find_owner(sequence: SequenceSyntax) -> Project {
    let (users, all_sequences) = user_sequences().await;
    let index = all_sequences.iter().position(|sequences| {
        sequences.iter().position(|seq| {
            seq.name == sequence.name
                && seq.parameters as usize == sequence.parameters.len()
                && seq.sequences as usize == sequence.sequences.len()
        }) != None
    });
    (&(users[index.unwrap()])).clone()
}
