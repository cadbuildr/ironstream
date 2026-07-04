// FILE: transfer_b_rep_shape_info.rs
// occt: TransferBRep_ShapeInfo

/// Information provider for Shape objects in the transfer framework.
/// Provides type information for shape entities.
pub struct TransferBRepShapeInfo;

impl TransferBRepShapeInfo {
    /// Returns the type name of a shape.
    /// In OCCT, shapes don't have dynamic types; we return a static type name.
    pub fn type_name() -> &'static str {
        "TopoDS_Shape"
    }

    /// Returns the discriminant type of a shape based on a simple identifier.
    /// Maps a shape id to its actual type name.
    pub fn shape_type_name(shape_id: u32) -> &'static str {
        match shape_id {
            0 => "TopoDS_Compound",
            1 => "TopoDS_CompSolid",
            2 => "TopoDS_Solid",
            3 => "TopoDS_Shell",
            4 => "TopoDS_Face",
            5 => "TopoDS_Wire",
            6 => "TopoDS_Edge",
            7 => "TopoDS_Vertex",
            _ => "TopoDS_Shape",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_name() {
        assert_eq!(TransferBRepShapeInfo::type_name(), "TopoDS_Shape");
    }

    #[test]
    fn test_shape_type_names() {
        assert_eq!(TransferBRepShapeInfo::shape_type_name(0), "TopoDS_Compound");
        assert_eq!(TransferBRepShapeInfo::shape_type_name(1), "TopoDS_CompSolid");
        assert_eq!(TransferBRepShapeInfo::shape_type_name(2), "TopoDS_Solid");
        assert_eq!(TransferBRepShapeInfo::shape_type_name(7), "TopoDS_Vertex");
        assert_eq!(TransferBRepShapeInfo::shape_type_name(999), "TopoDS_Shape");
    }
}
