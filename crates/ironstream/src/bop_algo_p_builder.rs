// FILE: bop_algo_p_builder.rs
// occt: BOPAlgo_PBuilder

pub struct BopAlgoPBuilder;

impl BopAlgoPBuilder {
    pub fn new() -> Self {
        BopAlgoPBuilder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoPBuilder::new();
    }
}
