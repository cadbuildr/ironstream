// FILE: bop_algo_wire_splitter.rs
// occt: BOPAlgo_WireSplitter

pub struct BopAlgoWireSplitter;

impl BopAlgoWireSplitter {
    pub fn new() -> Self {
        BopAlgoWireSplitter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoWireSplitter::new();
    }
}
