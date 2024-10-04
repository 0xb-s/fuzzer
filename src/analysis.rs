use crate::utils::ExecutionResult;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Analyzer {
    pub results: HashMap<ExecutionResult, u64>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            results: HashMap::new(),
        }
    }

    pub fn record_result(&mut self, result: ExecutionResult) {
        *self.results.entry(result).or_insert(0) += 1;
    }

    pub fn report(&self) {
        println!("--- Fuzzing Report ---");
        for (result, count) in &self.results {
            println!("{:?}: {}", result, count);
        }
        println!("----------------------");
    }
}
