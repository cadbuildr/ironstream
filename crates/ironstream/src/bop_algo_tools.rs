// FILE: bop_algo_tools.rs
// occt: BOPAlgo_Tools

pub struct BopAlgoTools;

impl BopAlgoTools {
    pub fn new() -> Self {
        BopAlgoTools
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoTools::new();
    }
}
