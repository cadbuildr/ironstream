// FILE: bop_algo_splitter.rs
// occt: BOPAlgo_Splitter

pub struct BopAlgoSplitter;

impl BopAlgoSplitter {
    pub fn new() -> Self {
        BopAlgoSplitter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoSplitter::new();
    }
}
