// FILE: xcaf_dim_tol_objects_geom_tolerance_mat_req_modif.rs
// occt: XCAFDimTolObjects_GeomToleranceMatReqModif

/// Enum representing XCAFDimTolObjects_GeomToleranceMatReqModif from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum XCAFDimTolObjects_GeomToleranceMatReqModif {
    None,
    M,
    L,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_dim_tol_objects_geom_tolerance_mat_req_modif_variants() {
        let _ = XCAFDimTolObjects_GeomToleranceMatReqModif::None;
        let _ = XCAFDimTolObjects_GeomToleranceMatReqModif::M;
        let _ = XCAFDimTolObjects_GeomToleranceMatReqModif::L;
    }
}
