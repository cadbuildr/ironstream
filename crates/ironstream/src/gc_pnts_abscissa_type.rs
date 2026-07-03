// FILE: gc_pnts_abscissa_type.rs
// occt: GCPnts_AbscissaType

/// Enum for abscissa parameter type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GcPntsAbscissaType {
    /// Length parametrized.
    LengthParametrized,
    /// Parametrized.
    Parametrized,
    /// Absolute composite.
    AbsComposite,
}

impl GcPntsAbscissaType {
    pub fn as_u32(&self) -> u32 {
        match self {
            GcPntsAbscissaType::LengthParametrized => 0,
            GcPntsAbscissaType::Parametrized => 1,
            GcPntsAbscissaType::AbsComposite => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(GcPntsAbscissaType::LengthParametrized.as_u32(), 0);
        assert_eq!(GcPntsAbscissaType::Parametrized.as_u32(), 1);
        assert_eq!(GcPntsAbscissaType::AbsComposite.as_u32(), 2);
    }

    #[test]
    fn test_clone() {
        let t = GcPntsAbscissaType::Parametrized;
        let t2 = t.clone();
        assert_eq!(t, t2);
    }
}
