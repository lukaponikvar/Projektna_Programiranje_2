use crate::structs::{
    project::Project,
    sequences::{SequenceInfo, SequenceSyntax},
};

///Funkcija vrne vse lastnike `Vec<Project>` iskanega zaporedja `sequence: SequenceSyntax`
/// ob podanih uporabnikih `users` in njihovih zaporedjih `all_sequences`.
pub async fn find_owners(
    sequence: SequenceSyntax,
    users: Vec<Project>,
    all_sequences: Vec<Vec<SequenceInfo>>,
) -> Vec<Project> {
    let indices: Vec<usize> = all_sequences
        .iter()
        .enumerate()
        .filter(|(_, sequences)| {
            sequences.iter().position(|seq| {
                seq.name == sequence.name
                    && seq.parameters as usize == sequence.parameters.len()
                    && seq.sequences as usize == sequence.sequences.len()
            }) != None
        })
        .map(|(index, _)| index)
        .collect();
    let mut owners = Vec::new();
    for index in indices {
        owners.push(users[index].clone())
    }
    owners
}
