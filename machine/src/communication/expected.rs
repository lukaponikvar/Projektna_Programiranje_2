use crate::{
    functions::our_sequences::our_sequences, 
    structs::custom_error::CustomError,
    structs::sequences::SequenceSyntax,
};

/// Funkcija preveri, ali najin strežnik poseduje iskano zaporedje.
pub fn expected(syn: &SequenceSyntax) -> Result<bool, CustomError> {
    println!("Je pričakovan exp");
    let our_sequences = our_sequences();
    let index = our_sequences.iter().position(|seq| seq.name == syn.name);
    let index = match index {
        Some(i) => i,
        None => {
            return Err(CustomError::new(format!(
                "Requested an invalid sequence.\nSequence name: {} does not match any of ours.\n",
                syn.name
            )));
        }
    };
    if our_sequences[index].parameters as usize != syn.parameters.len() {
        return Err(CustomError::new(format!(
                "Requested an invalid sequence.\nSequence name: {} does match one of ours, nevertheless number of given parameters: {} is not equal to the required number of parameters: {}.\n", 
                syn.name, 
                syn.parameters.len(),
                our_sequences[index].parameters
        )),);
    } else if our_sequences[index].sequences as usize != syn.sequences.len() {
        return Err(CustomError::new(format!(
                "Requested an invalid sequence.\nSequence name: {} does match one of ours, nevertheless number of given sequences: {} is not equal to the required number of sequences: {}.\n",
                syn.name, 
                syn.sequences.len(),
                our_sequences[index].sequences
            ),
        ));
    } else {
        Ok(true)
    }
}
