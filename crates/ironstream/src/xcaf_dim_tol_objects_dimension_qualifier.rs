// FILE: xcaf_dim_tol_objects_dimension_qualifier.rs
// occt: XCAFDimTolObjects_DimensionQualifier

/// Enum representing XCAFDimTolObjects_DimensionQualifier from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum XCAFDimTolObjects_DimensionQualifier {
    None,
    Min,
    Max,
    Avg,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_dim_tol_objects_dimension_qualifier_variants() {
        let _ = XCAFDimTolObjects_DimensionQualifier::None;
        let _ = XCAFDimTolObjects_DimensionQualifier::Min;
        let _ = XCAFDimTolObjects_DimensionQualifier::Max;
        let _ = XCAFDimTolObjects_DimensionQualifier::Avg;
    }
}
