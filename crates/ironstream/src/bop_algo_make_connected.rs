// FILE: bop_algo_make_connected.rs
// occt: BOPAlgo_MakeConnected

pub struct BopAlgoMakeConnected;

impl BopAlgoMakeConnected {
    pub fn new() -> Self {
        BopAlgoMakeConnected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoMakeConnected::new();
    }
}
