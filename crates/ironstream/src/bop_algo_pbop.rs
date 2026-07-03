// FILE: bop_algo_pbop.rs
// occt: BOPAlgo_PBOP

pub struct BopAlgoPbop;

impl BopAlgoPbop {
    pub fn new() -> Self {
        BopAlgoPbop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoPbop::new();
    }
}
