// FILE: rw_obj_sub_mesh_reason.rs
// occt: RWObj_SubMeshReason

//! Reason for creating a new group within OBJ reader.
//! Faithful port of the `RWObj_SubMeshReason` enumeration.

/// Reason for creating a new group within OBJ reader.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RWObjSubMeshReason {
    /// New object, should occur only once in a valid OBJ file
    /// (at the very beginning).
    NewObject,
    /// New group (`g` item).
    NewGroup,
    /// New material (`usemtl` item).
    NewMaterial,
    /// New smoothing group (`s` item).
    NewSmoothGroup,
}

impl RWObjSubMeshReason {
    /// The OBJ statement keyword that triggers this reason.
    pub fn triggering_keyword(&self) -> &'static str {
        match self {
            RWObjSubMeshReason::NewObject => "o",
            RWObjSubMeshReason::NewGroup => "g",
            RWObjSubMeshReason::NewMaterial => "usemtl",
            RWObjSubMeshReason::NewSmoothGroup => "s",
        }
    }

    /// Maps an OBJ statement keyword to the sub-mesh split reason,
    /// or None when the keyword does not start a new sub-mesh.
    pub fn from_keyword(keyword: &str) -> Option<RWObjSubMeshReason> {
        match keyword {
            "o" => Some(RWObjSubMeshReason::NewObject),
            "g" => Some(RWObjSubMeshReason::NewGroup),
            "usemtl" => Some(RWObjSubMeshReason::NewMaterial),
            "s" => Some(RWObjSubMeshReason::NewSmoothGroup),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_values_are_distinct() {
        let all = [
            RWObjSubMeshReason::NewObject,
            RWObjSubMeshReason::NewGroup,
            RWObjSubMeshReason::NewMaterial,
            RWObjSubMeshReason::NewSmoothGroup,
        ];
        for (i, a) in all.iter().enumerate() {
            for (j, b) in all.iter().enumerate() {
                assert_eq!(a == b, i == j);
            }
        }
    }

    #[test]
    fn keyword_roundtrip() {
        for r in [
            RWObjSubMeshReason::NewObject,
            RWObjSubMeshReason::NewGroup,
            RWObjSubMeshReason::NewMaterial,
            RWObjSubMeshReason::NewSmoothGroup,
        ] {
            assert_eq!(RWObjSubMeshReason::from_keyword(r.triggering_keyword()), Some(r));
        }
        assert_eq!(RWObjSubMeshReason::from_keyword("v"), None);
        assert_eq!(RWObjSubMeshReason::from_keyword("f"), None);
    }
}
