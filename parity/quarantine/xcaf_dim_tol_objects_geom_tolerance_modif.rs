// FILE: xcaf_dim_tol_objects_geom_tolerance_modif.rs
// occt: XCAFDimTolObjects_GeomToleranceModif

/// Enum representing XCAFDimTolObjects_GeomToleranceModif from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum XCAFDimTolObjects_GeomToleranceModif {
    Any_Cross_Section,
    Common_Zone,
    Each_Radial_Element,
    Free_State,
    Least_Material_Requirement,
    Line_Element,
    Major_Diameter,
    Maximum_Material_Requirement,
    Minor_Diameter,
    Not_Convex,
    Pitch_Diameter,
    Reciprocity_Requirement,
    Separate_Requirement,
    Statistical_Tolerance,
    Tangent_Plane,
    All_Around,
    All_Over,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_dim_tol_objects_geom_tolerance_modif_variants() {
        let _ = XCAFDimTolObjects_GeomToleranceModif::Any_Cross_Section;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Common_Zone;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Each_Radial_Element;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Free_State;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Least_Material_Requirement;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Line_Element;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Major_Diameter;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Maximum_Material_Requirement;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Minor_Diameter;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Not_Convex;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Pitch_Diameter;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Reciprocity_Requirement;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Separate_Requirement;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Statistical_Tolerance;
        let _ = XCAFDimTolObjects_GeomToleranceModif::Tangent_Plane;
        let _ = XCAFDimTolObjects_GeomToleranceModif::All_Around;
        let _ = XCAFDimTolObjects_GeomToleranceModif::All_Over;
    }
}
