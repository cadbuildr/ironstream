// FILE: bopds_p_iterator_si.rs
// occt: BOPDS_PIteratorSI

/// Parallel Sub-iterator for shape interference
pub struct BodsPIteratorSI;

impl BodsPIteratorSI {
    pub fn new() -> Self {
        BodsPIteratorSI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _it = BodsPIteratorSI::new();
    }
}
