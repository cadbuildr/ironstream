// FILE: bop_algo_p_argument_analyzer.rs
// occt: BOPAlgo_PArgumentAnalyzer

pub struct BopAlgoPArgumentAnalyzer;

impl BopAlgoPArgumentAnalyzer {
    pub fn new() -> Self {
        BopAlgoPArgumentAnalyzer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoPArgumentAnalyzer::new();
    }
}
