// FILE: xcaf_dim_tol_objects_geom_tolerance_zone_modif.rs
// occt: XCAFDimTolObjects_GeomToleranceZoneModif

/// Enum representing XCAFDimTolObjects_GeomToleranceZoneModif from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum XCAFDimTolObjects_GeomToleranceZoneModif {
    None,
    Projected,
    Runout,
    NonUniform,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_dim_tol_objects_geom_tolerance_zone_modif_variants() {
        let _ = XCAFDimTolObjects_GeomToleranceZoneModif::None;
        let _ = XCAFDimTolObjects_GeomToleranceZoneModif::Projected;
        let _ = XCAFDimTolObjects_GeomToleranceZoneModif::Runout;
        let _ = XCAFDimTolObjects_GeomToleranceZoneModif::NonUniform;
    }
}
