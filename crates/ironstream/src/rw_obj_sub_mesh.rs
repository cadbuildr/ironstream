// FILE: rw_obj_sub_mesh.rs
// occt: RWObj_SubMesh

//! Sub-mesh definition for OBJ reader.
//! Faithful port of the `RWObj_SubMesh` struct: the four active-context
//! strings the OBJ reader tracks while splitting the file into sub-meshes.

/// Sub-mesh definition for OBJ reader.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RWObjSubMesh {
    /// Name of active object (`o` statement).
    pub object: String,
    /// Name of active group (`g` statement).
    pub group: String,
    /// Name of active smoothing group (`s` statement).
    pub smooth_group: String,
    /// Name of active material (`usemtl` statement).
    pub material: String,
}

impl RWObjSubMesh {
    pub fn new() -> Self {
        RWObjSubMesh::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_all_empty() {
        let sm = RWObjSubMesh::new();
        assert!(sm.object.is_empty());
        assert!(sm.group.is_empty());
        assert!(sm.smooth_group.is_empty());
        assert!(sm.material.is_empty());
    }

    #[test]
    fn tracks_active_context_like_obj_reader() {
        // Simulate the reader walking statements: o -> g -> usemtl -> s.
        let mut active = RWObjSubMesh::new();
        active.object = "wheel".to_string();
        active.group = "rim".to_string();
        active.material = "steel".to_string();
        active.smooth_group = "1".to_string();

        // A new group statement replaces only the Group field; the reader
        // copies the struct when it flushes the sub-mesh.
        let flushed = active.clone();
        active.group = "spokes".to_string();
        assert_eq!(flushed.group, "rim");
        assert_eq!(active.group, "spokes");
        assert_eq!(active.object, flushed.object);
        assert_ne!(active, flushed);
    }
}
