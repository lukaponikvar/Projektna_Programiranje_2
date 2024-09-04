use super::user_sequences::user_sequences;
use crate::structs::{project::Project, sequences::SequenceSyntax};

pub async fn find_owner(sequence: SequenceSyntax) -> Project {
    let (users, zaporedja) = user_sequences().await;
    let index = zaporedja.iter().position(|y| {
        y.iter().position(|x| {
            x.name == sequence.name
                && x.parameters as usize == sequence.parameters.len()
                && x.sequences as usize == sequence.sequences.len()
        }) != None
    });
    let a = &(users[index.unwrap()]);
    a.clone()
}
