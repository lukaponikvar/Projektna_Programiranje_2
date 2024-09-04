use crate::sequences::arithmetic::Arithmetic;
use crate::sequences::models::Sequence;
use crate::sequences::sum::Sum;
use crate::structs::sequences::SequenceSyntax;
use async_recursion::async_recursion;

/// Tralala
#[async_recursion]
pub async fn eval(syn: SequenceSyntax) -> Box<dyn Sequence<f64, dyn Send> + Send> {
    println!("{}", (syn).name.clone());
    let zaporedje: Box<dyn Sequence<f64, dyn Send> + Send> = match (syn).name.clone() {
        s if s == "/sequence/Sum".to_string() => {
            let a = eval(*(syn.sequences[0].clone())).await;
            let b = eval(*(syn.sequences[1].clone())).await;
            // let b = Arithmetic::new(1.0, 1.0);
            Sum::new(vec![a, b])
        }
        s if s == "/sequence/Arithmetic".to_string() => {
            Arithmetic::new(syn.parameters[0], syn.parameters[1])
        }
        _ => Arithmetic::new(1.0, 1.0),
    };
    zaporedje
}
