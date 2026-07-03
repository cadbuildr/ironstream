// FILE: tk_mesh_pch.rs
// occt: TKMesh_pch

//! Precompiled header for TKMesh module.
//! Includes common mesh data types and interfaces.

pub mod mesh_types {
    /// Common mesh-related type definitions
    pub const MODULE_NAME: &str = "TKMesh";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_name() {
        assert_eq!(mesh_types::MODULE_NAME, "TKMesh");
    }
}
