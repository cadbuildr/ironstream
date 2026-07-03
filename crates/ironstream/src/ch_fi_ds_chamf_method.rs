// FILE: ch_fi_ds_chamf_method.rs
// occt: ChFiDS_ChamfMethod

/// Method for defining a chamfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChFiDsChamfMethod {
    /// Symmetric chamfer
    Sym,
    /// Chamfer defined by two distances
    TwoDist,
    /// Chamfer defined by distance and angle
    DistAngle,
}

impl Default for ChFiDsChamfMethod {
    fn default() -> Self {
        Self::Sym
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(ChFiDsChamfMethod::Sym, ChFiDsChamfMethod::Sym);
        assert_ne!(ChFiDsChamfMethod::Sym, ChFiDsChamfMethod::TwoDist);
    }

    #[test]
    fn test_default() {
        assert_eq!(ChFiDsChamfMethod::default(), ChFiDsChamfMethod::Sym);
    }

    #[test]
    fn test_copy_semantics() {
        let m = ChFiDsChamfMethod::DistAngle;
        let m2 = m;
        assert_eq!(m, m2);
    }
}
