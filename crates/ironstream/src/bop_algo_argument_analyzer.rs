// FILE: bop_algo_argument_analyzer.rs
// occt: BOPAlgo_ArgumentAnalyzer

pub struct BopAlgoArgumentAnalyzer;

impl BopAlgoArgumentAnalyzer {
    pub fn new() -> Self {
        BopAlgoArgumentAnalyzer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoArgumentAnalyzer::new();
    }
}
