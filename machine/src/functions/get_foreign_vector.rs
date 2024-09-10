use super::{
    get_foreign_vector_support::{drop::drop, min::min, other::other, product::product, sum::sum},
    get_vector_support::{arithmetic::arithmetic, constant::constant, geometric::geometric},
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
        s if s == &"Constant".to_string() && ours => constant(syn, range),
        s if s == &"Arithmetic".to_string() && ours => arithmetic(syn, range),
        s if s == &"Geometric".to_string() && ours => geometric(syn, range),
        s if s == &"Sum".to_string() && ours => sum(syn, range, users, all_sequences).await,
        s if s == &"Product".to_string() && ours => {
            return product(syn, range, users, all_sequences).await
        }
        s if s == &"Drop".to_string() && ours => drop(syn, range, users, all_sequences).await,
        s if s == &"Min".to_string() && ours => min(syn, range, users, all_sequences).await,
        _ => other(syn, range, users, all_sequences).await,
    }
}
