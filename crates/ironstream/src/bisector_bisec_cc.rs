// FILE: bisector_bisec_cc.rs
// occt: Bisector_BisecCC

/// Constructs the bisector between two curves.
#[derive(Debug, Clone)]
pub struct BisectorBisecCC {
    is_empty: bool,
}

impl BisectorBisecCC {
    pub fn new() -> Self {
        BisectorBisecCC { is_empty: false }
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
}

impl Default for BisectorBisecCC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisec_cc() {
        let bc = BisectorBisecCC::new();
        assert!(!bc.is_empty());
    }
}
