// FILE: gc_pnts_deflection_type.rs
// occt: GCPnts_DeflectionType

/// Enum for deflection type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GcPntsDeflectionType {
    /// Linear deflection.
    Linear,
    /// Angular deflection.
    Angular,
    /// Sagitta deflection.
    Sagitta,
}

impl GcPntsDeflectionType {
    pub fn as_u32(&self) -> u32 {
        match self {
            GcPntsDeflectionType::Linear => 0,
            GcPntsDeflectionType::Angular => 1,
            GcPntsDeflectionType::Sagitta => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(GcPntsDeflectionType::Linear.as_u32(), 0);
        assert_eq!(GcPntsDeflectionType::Angular.as_u32(), 1);
        assert_eq!(GcPntsDeflectionType::Sagitta.as_u32(), 2);
    }

    #[test]
    fn test_equality() {
        let t1 = GcPntsDeflectionType::Linear;
        let t2 = GcPntsDeflectionType::Linear;
        assert_eq!(t1, t2);
    }
}
