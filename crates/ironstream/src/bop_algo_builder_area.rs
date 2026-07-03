// FILE: bop_algo_builder_area.rs
// occt: BOPAlgo_BuilderArea

pub struct BopAlgoBuilderArea;

impl BopAlgoBuilderArea {
    pub fn new() -> Self { BopAlgoBuilderArea }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { let _ = BopAlgoBuilderArea::new(); }
}
