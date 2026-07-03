// FILE: top_ope_b_rep_define.rs
// occt: TopOpeBRep_define

//! Macro definitions for TopOpeBRep types.
//! These are Rust equivalents of C++ preprocessor macros.

pub struct Define;

impl Define {
    /// MTPvpic -> TopOpeBRep_VPointInterClassifier
    pub const MTP_VPIC: &'static str = "TopOpeBRep_VPointInterClassifier";

    /// MTPvpii -> TopOpeBRep_VPointInterIterator
    pub const MTP_VPII: &'static str = "TopOpeBRep_VPointInterIterator";

    /// MTPvpi -> TopOpeBRep_VPointInter
    pub const MTP_VPI: &'static str = "TopOpeBRep_VPointInter";

    /// MTPli -> TopOpeBRep_LineInter
    pub const MTP_LI: &'static str = "TopOpeBRep_LineInter";

    /// MTPfi -> TopOpeBRep_FacesIntersector
    pub const MTP_FI: &'static str = "TopOpeBRep_FacesIntersector";

    /// MTPei -> TopOpeBRep_EdgesIntersector
    pub const MTP_EI: &'static str = "TopOpeBRep_EdgesIntersector";

    /// MTPfei -> TopOpeBRep_FaceEdgeIntersector
    pub const MTP_FEI: &'static str = "TopOpeBRep_FaceEdgeIntersector";

    /// MTPff -> TopOpeBRep_FacesFiller
    pub const MTP_FF: &'static str = "TopOpeBRep_FacesFiller";

    /// MTPef -> TopOpeBRep_EdgesFiller
    pub const MTP_EF: &'static str = "TopOpeBRep_EdgesFiller";

    /// MTPfef -> TopOpeBRep_FaceEdgeFiller
    pub const MTP_FEF: &'static str = "TopOpeBRep_FaceEdgeFiller";

    /// MTPpgt -> TopOpeBRep_PointGeomTool
    pub const MTP_PGT: &'static str = "TopOpeBRep_PointGeomTool";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_constants() {
        assert_eq!(Define::MTP_VPIC, "TopOpeBRep_VPointInterClassifier");
        assert_eq!(Define::MTP_VPII, "TopOpeBRep_VPointInterIterator");
        assert_eq!(Define::MTP_VPI, "TopOpeBRep_VPointInter");
        assert_eq!(Define::MTP_LI, "TopOpeBRep_LineInter");
    }

    #[test]
    fn test_all_defines() {
        assert!(!Define::MTP_FI.is_empty());
        assert!(!Define::MTP_EI.is_empty());
        assert!(!Define::MTP_FEI.is_empty());
        assert!(!Define::MTP_FF.is_empty());
        assert!(!Define::MTP_EF.is_empty());
        assert!(!Define::MTP_FEF.is_empty());
        assert!(!Define::MTP_PGT.is_empty());
    }
}
