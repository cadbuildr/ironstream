// FILE: bisector_inter.rs
// occt: Bisector_Inter

/// Intersection between two bisector curves.
#[derive(Debug, Clone)]
pub struct BisectorInter;

impl BisectorInter {
    pub fn new() -> Self {
        BisectorInter
    }
}

impl Default for BisectorInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inter() {
        let _bi = BisectorInter::new();
    }
}
