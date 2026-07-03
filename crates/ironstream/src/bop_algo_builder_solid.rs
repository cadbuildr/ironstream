// FILE: bop_algo_builder_solid.rs
// occt: BOPAlgo_BuilderSolid

pub struct BopAlgoBuilderSolid;

impl BopAlgoBuilderSolid {
    pub fn new() -> Self {
        BopAlgoBuilderSolid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BopAlgoBuilderSolid::new();
    }
}
