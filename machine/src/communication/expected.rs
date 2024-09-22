use crate::{
    functions::our_sequences::our_sequences, 
    structs::custom_error::CustomError,
    structs::sequences::SequenceSyntax,
};

/// Checks if the searched sequence `syn` is one of sequences that belong to our server or not.
///
/// ## Errors
/// If requesting an invalid sequence (whether it is a name that does not match or number of given parameters or sequences), 
/// the request is reported as an invalid one, with additional information about expected sequence. 
pub fn expected(syn: &SequenceSyntax) -> Result<bool, CustomError> {
    let our_sequences = our_sequences();
    let index = our_sequences.iter().position(|seq| seq.name == syn.name);
    let index = match index {
        Some(i) => i,
        None => {
            return Err(CustomError::new(format!(
                "Invalid sequence name.\nSequence name: {} does not match any of ours.\n",
                syn.name
            )));
        }
    };
    if our_sequences[index].parameters as usize != syn.parameters.len() {
        return Err(CustomError::new(format!(
                "Invalid input format.\nSequence name: {} does match one of ours, nevertheless number of given parameters: {} is not equal to the required number of parameters: {}.\n", 
                syn.name, 
                syn.parameters.len(),
                our_sequences[index].parameters
        )),);
    } else if our_sequences[index].sequences as usize != syn.sequences.len() {
        return Err(CustomError::new(format!(
                "Invalid input format.\nSequence name: {} does match one of ours, nevertheless number of given sequences: {} is not equal to the required number of sequences: {}.\n",
                syn.name, 
                syn.sequences.len(),
                our_sequences[index].sequences
            ),
        ));
    } else {
        Ok(true)
    }
}
