// FILE: bop_algo_make_periodic.rs
// occt: BOPAlgo_MakePeriodic

pub struct BopAlgoMakePeriodic;

impl BopAlgoMakePeriodic {
    pub fn new() -> Self {
        BopAlgoMakePeriodic
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoMakePeriodic::new();
    }
}
