// FILE: mesh_vs_builder_priority.rs
// occt: MeshVS_BuilderPriority

/// Priority levels for mesh visualization builders.
/// Used to control the rendering order of mesh presentation components.
pub type MeshVsBuilderPriority = i32;

/// Standard mesh visualization builder priority constants.
pub mod mesh_vs_builder_priority {
    use super::MeshVsBuilderPriority;

    /// Base mesh rendering priority
    pub const MESH: MeshVsBuilderPriority = 5;

    /// Nodal color rendering priority
    pub const NODAL_COLOR: MeshVsBuilderPriority = 10;

    /// Element color rendering priority
    pub const ELEM_COLOR: MeshVsBuilderPriority = 15;

    /// Text rendering priority
    pub const TEXT: MeshVsBuilderPriority = 20;

    /// Vector rendering priority
    pub const VECTOR: MeshVsBuilderPriority = 25;

    /// User-defined rendering priority
    pub const USER: MeshVsBuilderPriority = 30;

    /// Default rendering priority (same as USER)
    pub const DEFAULT: MeshVsBuilderPriority = USER;
}

/// Builder priority helpers
pub struct MeshVsBuilderPriorityHelper;

impl MeshVsBuilderPriorityHelper {
    /// Get the name of a priority level
    pub fn name(priority: MeshVsBuilderPriority) -> &'static str {
        match priority {
            mesh_vs_builder_priority::MESH => "Mesh",
            mesh_vs_builder_priority::NODAL_COLOR => "NodalColor",
            mesh_vs_builder_priority::ELEM_COLOR => "ElemColor",
            mesh_vs_builder_priority::TEXT => "Text",
            mesh_vs_builder_priority::VECTOR => "Vector",
            // DEFAULT is an alias of USER (MeshVS_BP_Default = MeshVS_BP_User),
            // so the same value maps to "User".
            mesh_vs_builder_priority::USER => "User",
            _ => "Unknown",
        }
    }

    /// Check if a priority is one of the standard values
    pub fn is_standard(priority: MeshVsBuilderPriority) -> bool {
        matches!(
            priority,
            mesh_vs_builder_priority::MESH
                | mesh_vs_builder_priority::NODAL_COLOR
                | mesh_vs_builder_priority::ELEM_COLOR
                | mesh_vs_builder_priority::TEXT
                | mesh_vs_builder_priority::VECTOR
                | mesh_vs_builder_priority::USER
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::mesh_vs_builder_priority::*;

    #[test]
    fn test_priority_values() {
        assert_eq!(MESH, 5);
        assert_eq!(NODAL_COLOR, 10);
        assert_eq!(ELEM_COLOR, 15);
        assert_eq!(TEXT, 20);
        assert_eq!(VECTOR, 25);
        assert_eq!(USER, 30);
        assert_eq!(DEFAULT, 30);
    }

    #[test]
    fn test_default_equals_user() {
        assert_eq!(DEFAULT, USER);
    }

    #[test]
    fn test_priority_ordering() {
        // Priorities should be in ascending order
        assert!(MESH < NODAL_COLOR);
        assert!(NODAL_COLOR < ELEM_COLOR);
        assert!(ELEM_COLOR < TEXT);
        assert!(TEXT < VECTOR);
        assert!(VECTOR < USER);
    }

    #[test]
    fn test_helper_names() {
        assert_eq!(MeshVsBuilderPriorityHelper::name(MESH), "Mesh");
        assert_eq!(MeshVsBuilderPriorityHelper::name(NODAL_COLOR), "NodalColor");
        assert_eq!(MeshVsBuilderPriorityHelper::name(ELEM_COLOR), "ElemColor");
        assert_eq!(MeshVsBuilderPriorityHelper::name(TEXT), "Text");
        assert_eq!(MeshVsBuilderPriorityHelper::name(VECTOR), "Vector");
        assert_eq!(MeshVsBuilderPriorityHelper::name(USER), "User");
        // DEFAULT is the same value as USER, so it carries the same name.
        assert_eq!(MeshVsBuilderPriorityHelper::name(DEFAULT), "User");
        assert_eq!(MeshVsBuilderPriorityHelper::name(999), "Unknown");
    }

    #[test]
    fn test_is_standard() {
        assert!(MeshVsBuilderPriorityHelper::is_standard(MESH));
        assert!(MeshVsBuilderPriorityHelper::is_standard(NODAL_COLOR));
        assert!(MeshVsBuilderPriorityHelper::is_standard(ELEM_COLOR));
        assert!(MeshVsBuilderPriorityHelper::is_standard(TEXT));
        assert!(MeshVsBuilderPriorityHelper::is_standard(VECTOR));
        assert!(MeshVsBuilderPriorityHelper::is_standard(USER));
        assert!(!MeshVsBuilderPriorityHelper::is_standard(999));
    }

    #[test]
    fn test_custom_priority() {
        let custom_priority: MeshVsBuilderPriority = 42;
        assert!(!MeshVsBuilderPriorityHelper::is_standard(custom_priority));
        assert_eq!(MeshVsBuilderPriorityHelper::name(custom_priority), "Unknown");
    }
}
