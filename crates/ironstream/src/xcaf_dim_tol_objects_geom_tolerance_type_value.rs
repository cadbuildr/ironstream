// FILE: xcaf_dim_tol_objects_geom_tolerance_type_value.rs
// occt: XCAFDimTolObjects_GeomToleranceTypeValue

/// Enum representing XCAFDimTolObjects_GeomToleranceTypeValue from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum XCAFDimTolObjects_GeomToleranceTypeValue {
    None,
    Diameter,
    SphericalDiameter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_dim_tol_objects_geom_tolerance_type_value_variants() {
        let _ = XCAFDimTolObjects_GeomToleranceTypeValue::None;
        let _ = XCAFDimTolObjects_GeomToleranceTypeValue::Diameter;
        let _ = XCAFDimTolObjects_GeomToleranceTypeValue::SphericalDiameter;
    }
}
