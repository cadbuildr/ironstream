// FILE: xcaf_dim_tol_objects_tolerance_zone_affected_plane.rs
// occt: XCAFDimTolObjects_ToleranceZoneAffectedPlane

/// Enum representing XCAFDimTolObjects_ToleranceZoneAffectedPlane from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum XCAFDimTolObjects_ToleranceZoneAffectedPlane {
    None,
    Intersection,
    Orientation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_dim_tol_objects_tolerance_zone_affected_plane_variants() {
        let _ = XCAFDimTolObjects_ToleranceZoneAffectedPlane::None;
        let _ = XCAFDimTolObjects_ToleranceZoneAffectedPlane::Intersection;
        let _ = XCAFDimTolObjects_ToleranceZoneAffectedPlane::Orientation;
    }
}
