// FILE: bop_algo_p_pave_filler.rs
// occt: BOPAlgo_PPaveFiller

pub struct BopAlgoPPaveFiller;

impl BopAlgoPPaveFiller {
    pub fn new() -> Self {
        BopAlgoPPaveFiller
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoPPaveFiller::new();
    }
}
