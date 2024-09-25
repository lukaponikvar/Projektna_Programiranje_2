use crate::{
    sequences::{
        arithmetic::Arithmetic, average::Average, binary::Binary, constant::Constant, drop::Drop,
        fibonacci::Fibonacci, floor::Floor, geometric::Geometric,
        linear_combination::LinearCombination, max::Max, min::Min, models::Sequence,
        product::Product, random::Random, sum::Sum,
    },
    structs::sequences::SequenceSyntax,
};

/// Returns vector of searched sequence terms from our server in the given `range`.
///
/// ## Panics
/// If the match somehow fails, the program panics.
pub fn get_sequence(syn: &SequenceSyntax) -> Box<dyn Sequence<f64>> {
    match &(syn).name {
        s if s == &"Arithmetic".to_string() => {
            Arithmetic::new(syn.parameters[0], syn.parameters[1])
        }
        s if s == &"Average".to_string() => {
            let mut sequences = vec![];
            for seq in &syn.sequences {
                sequences.push(get_sequence(&seq))
            }
            Average::new(sequences)
        }
        s if s == &"Binary".to_string() => Binary::new(get_sequence(&*syn.sequences[0])),
        s if s == &"Constant".to_string() => Constant::new(syn.parameters[0]),
        s if s == &"Drop".to_string() => {
            Drop::new(syn.parameters[0] as u64, get_sequence(&*syn.sequences[0]))
        }
        s if s == &"Fibonacci".to_string() => Fibonacci::new(syn.parameters[0], syn.parameters[1]),
        s if s == &"Floor".to_string() => Floor::new(get_sequence(&syn.sequences[0])),
        s if s == &"Geometric".to_string() => Geometric::new(syn.parameters[0], syn.parameters[1]),
        s if s == &"LinearCombination".to_string() => {
            let mut sequences = vec![];
            for seq in &syn.sequences {
                sequences.push(get_sequence(&seq))
            }
            LinearCombination::new(syn.parameters.clone(), sequences)
        }
        s if s == &"Max".to_string() => {
            let mut sequences = vec![];
            for seq in &syn.sequences {
                sequences.push(get_sequence(&seq))
            }
            Max::new(sequences)
        }
        s if s == &"Min".to_string() => {
            let mut sequences = vec![];
            for seq in &syn.sequences {
                sequences.push(get_sequence(&seq))
            }
            Min::new(sequences)
        }
        s if s == &"Product".to_string() => {
            let mut sequences = vec![];
            for seq in &syn.sequences {
                sequences.push(get_sequence(&seq))
            }
            Product::new(sequences)
        }
        s if s == &"Random".to_string() => Random::new(syn.parameters[0], syn.parameters[1]),
        s if s == &"Sum".to_string() => {
            let mut sequences = vec![];
            for seq in &syn.sequences {
                sequences.push(get_sequence(&seq))
            }
            Sum::new(sequences)
        }
        _ => panic!("Should not have gotten here!"),
    }
}
