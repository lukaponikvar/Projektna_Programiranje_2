use super::get_foreign_vector_support::{
    arithmetic::arithmetic, average::average, constant::constant, drop::drop, fibonacci::fibonacci,
    floor::floor, geometric::geometric, linear_combination::linear_combination, max::max, min::min,
    other::other, product::product, random::random, sum::sum,
};
use crate::{
    communication::expected::expected,
    structs::{
        custom_error::CustomError,
        project::Project,
        range::Range,
        sequences::{SequenceInfo, SequenceSyntax},
    },
};
use async_recursion::async_recursion;

/// Returns vector of searched sequence terms from its owner (found among given `users`) in the given `range`.
///
/// ## Errors
/// In case there is no sequence that matches the requested sequence `syn`, the "Invalid input format" error is reported
/// with additional info about requested sequence.
#[async_recursion]
pub async fn get_foreign_vector(
    syn: &SequenceSyntax,
    range: &Range,
    users: &Vec<Project>,
    all_sequences: &Vec<Vec<SequenceInfo>>,
) -> Result<Vec<f64>, CustomError> {
    let ours = match expected(syn) {
        Ok(b) => b,
        Err(_) => false,
    };
    match &(syn).name {
        s if s == &"Arithmetic".to_string() && ours => arithmetic(syn, range),
        s if s == &"Average".to_string() => average(syn, range, users, all_sequences).await,
        s if s == &"Constant".to_string() && ours => constant(syn, range),
        s if s == &"Drop".to_string() && ours => drop(syn, range, users, all_sequences).await,
        s if s == &"Fibonacci".to_string() => fibonacci(syn, range),
        s if s == &"Floor".to_string() => floor(syn, range, users, all_sequences).await,
        s if s == &"Geometric".to_string() && ours => geometric(syn, range),
        s if s == &"LinearCombination".to_string() => {
            linear_combination(syn, range, users, all_sequences).await
        }
        s if s == &"Max".to_string() => max(syn, range, users, all_sequences).await,
        s if s == &"Min".to_string() && ours => min(syn, range, users, all_sequences).await,
        s if s == &"Product".to_string() && ours => {
            return product(syn, range, users, all_sequences).await
        }
        s if s == &"Random".to_string() => random(syn, range),
        s if s == &"Sum".to_string() && ours => sum(syn, range, users, all_sequences).await,
        _ => other(syn, range, users, all_sequences).await,
    }
}
