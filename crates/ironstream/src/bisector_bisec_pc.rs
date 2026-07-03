// FILE: bisector_bisec_pc.rs
// occt: Bisector_BisecPC

/// Provides the bisector between a point and a curve.
#[derive(Debug, Clone)]
pub struct BisectorBisecPC {
    is_empty: bool,
}

impl BisectorBisecPC {
    pub fn new() -> Self {
        BisectorBisecPC { is_empty: false }
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
}

impl Default for BisectorBisecPC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisec_pc() {
        let bp = BisectorBisecPC::new();
        assert!(!bp.is_empty());
    }
}
