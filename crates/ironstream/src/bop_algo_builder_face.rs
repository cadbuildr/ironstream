// FILE: bop_algo_builder_face.rs
// occt: BOPAlgo_BuilderFace

pub struct BopAlgoBuilderFace;

impl BopAlgoBuilderFace {
    pub fn new() -> Self {
        BopAlgoBuilderFace
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BopAlgoBuilderFace::new();
    }
}
