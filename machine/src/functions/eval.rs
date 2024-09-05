use crate::sequences::arithmetic::Arithmetic;
use crate::sequences::constant::Constant;
use crate::sequences::drop::Drop;
use crate::sequences::geometric::Geometric;
use crate::sequences::models::Sequence;
use crate::sequences::product::Product;
use crate::sequences::sum::Sum;
use crate::structs::sequences::SequenceSyntax;
use async_recursion::async_recursion;

#[async_recursion]
pub async fn eval(syn: SequenceSyntax) -> Box<dyn Sequence<f64, dyn Send> + Send> {
    let sequence: Box<dyn Sequence<f64, dyn Send> + Send> = match (syn).name.clone() {
        s if s == "Constant".to_string() => Constant::new(syn.parameters[0]),
        s if s == "Arithmetic".to_string() => Arithmetic::new(syn.parameters[0], syn.parameters[1]),
        s if s == "Geometric".to_string() => Geometric::new(syn.parameters[0], syn.parameters[1]),
        s if s == "Sum".to_string() => {
            let mut sequences = Vec::new();
            for seq in syn.sequences {
                sequences.push(eval(*(seq.clone())).await);
            }
            Sum::new(sequences)
        }
        s if s == "Product".to_string() => {
            let mut sequences = Vec::new();
            for seq in syn.sequences {
                sequences.push(eval(*(seq.clone())).await);
            }
            println!("{:#?}", sequences.len());
            Product::new(sequences)
        }
        s if s == "Drop".to_string() => Drop::new(
            syn.parameters[0] as u64,
            eval(*(syn.sequences[0]).clone()).await,
        ),
        _ => Arithmetic::new(1.0, 1.0),
    };
    sequence
}
