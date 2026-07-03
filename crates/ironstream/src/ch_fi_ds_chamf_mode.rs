// FILE: ch_fi_ds_chamf_mode.rs
// occt: ChFiDS_ChamfMode

/// Modes of chamfer definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChFiDsChamfMode {
    /// Chamfer with constant distance from spine to one of the two surfaces
    ClassicChamfer,
    /// Symmetric chamfer with constant throat (height of isosceles triangle in section)
    ConstThroatChamfer,
    /// Chamfer with constant throat and penetration:
    /// the section is a right-angled triangle,
    /// the first surface (top of chamfer) is virtually moved inside by offset,
    /// the apex is on the intersection between moved surface and second surface,
    /// right angle at the top, constant throat length
    ConstThroatWithPenetrationChamfer,
}

impl Default for ChFiDsChamfMode {
    fn default() -> Self {
        Self::ClassicChamfer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(ChFiDsChamfMode::ClassicChamfer, ChFiDsChamfMode::ClassicChamfer);
        assert_ne!(
            ChFiDsChamfMode::ClassicChamfer,
            ChFiDsChamfMode::ConstThroatChamfer
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(ChFiDsChamfMode::default(), ChFiDsChamfMode::ClassicChamfer);
    }

    #[test]
    fn test_copy_semantics() {
        let m = ChFiDsChamfMode::ConstThroatWithPenetrationChamfer;
        let m2 = m;
        assert_eq!(m, m2);
    }
}
