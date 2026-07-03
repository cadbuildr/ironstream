// FILE: bop_algo_cells_builder.rs
// occt: BOPAlgo_CellsBuilder

pub struct BopAlgoCellsBuilder;

impl BopAlgoCellsBuilder {
    pub fn new() -> Self {
        BopAlgoCellsBuilder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BopAlgoCellsBuilder::new();
    }
}
