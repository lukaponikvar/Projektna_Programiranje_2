use crate::sequences::arithmetic::Arithmetic;
use crate::sequences::models::Sequence;
use crate::structs::sequences::SequenceSyntax;

pub async fn evall(syn: SequenceSyntax) -> Box<dyn Sequence<f64>> {
    let zaporedje = match (syn).name.clone() {
        s if s == "/sequence/Sum".to_string() => {
            let a = evall(*(syn.sequences[0].clone())).await;
            // let b = evall(*(syn.sequences[1])).await;
            // Sum::new(vec![a, b])
            Arithmetic::new(2.0, 1.0)
        }
        s if s == "/sequence/Arithmetic".to_string() => {
            Arithmetic::new(syn.parameters[0], syn.parameters[1])
        }
        _ => Arithmetic::new(1.0, 1.0),
    };
    zaporedje
}
