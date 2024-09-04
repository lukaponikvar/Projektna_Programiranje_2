use crate::sequences::arithmetic::Arithmetic;
use crate::sequences::models::Sequence;
use crate::sequences::sum::Sum;
use crate::structs::sequences::SequenceSyntax;
use async_recursion::async_recursion;

#[async_recursion]
pub async fn eval(syn: SequenceSyntax) -> Box<dyn Sequence<f64, dyn Send> + Send> {
    let sequence: Box<dyn Sequence<f64, dyn Send> + Send> = match (syn).name.clone() {
        s if s == "Sum".to_string() => {
            let a = eval(*(syn.sequences[0].clone())).await;
            let b = eval(*(syn.sequences[1].clone())).await;
            Sum::new(vec![a, b])
        }
        s if s == "Arithmetic".to_string() => Arithmetic::new(syn.parameters[0], syn.parameters[1]),
        _ => Arithmetic::new(1.0, 1.0),
    };
    sequence
}
