use super::models::Sequence;
use crate::{
    functions::get_list::get_list,
    structs::{range::Range, sequences::SequenceSyntax},
};
// use std::error::Error;
use hyper::Error;

pub struct Sum {
    pub sequences: Vec<Box<SequenceSyntax>>,
}

// Vec<Box<&'a dyn Sequence<f64>>>

impl Sum {
    pub fn new(sequences: Vec<Box<SequenceSyntax>>) -> Box<Sum> {
        Box::new(Sum { sequences })
    }
}

impl Sequence<f64> for Sum {
    async fn range(&self, range: Range) -> Vec<f64> {
        let result1 = get_list((*(self.sequences[0])).clone(), range.clone()).await;
        let result2 = get_list((*(self.sequences[1])).clone(), range).await;
        let res1 = match result1 {
            Ok(s) => s,
            _ => vec![2.0],
        };
        // let res2 = match result2 {
        //     Ok(s) => s,
        //     _ => vec![2.0],
        // };
        return vec![res1[0]];
    }
}
