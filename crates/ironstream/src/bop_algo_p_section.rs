// FILE: bop_algo_p_section.rs
// occt: BOPAlgo_PSection

pub struct BopAlgoPSection;

impl BopAlgoPSection {
    pub fn new() -> Self {
        BopAlgoPSection
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoPSection::new();
    }
}
