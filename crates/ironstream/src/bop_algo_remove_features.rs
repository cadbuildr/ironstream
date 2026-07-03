// FILE: bop_algo_remove_features.rs
// occt: BOPAlgo_RemoveFeatures

pub struct BopAlgoRemoveFeatures;

impl BopAlgoRemoveFeatures {
    pub fn new() -> Self {
        BopAlgoRemoveFeatures
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoRemoveFeatures::new();
    }
}
