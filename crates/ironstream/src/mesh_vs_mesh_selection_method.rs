// FILE: mesh_vs_mesh_selection_method.rs
// occt: MeshVS_MeshSelectionMethod

/// Enumeration describing the type of sensitive entity built for mesh selection.
/// This determines what method is used for selecting the whole mesh in mode 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshSelectionMethod {
    /// Precise selection using individual mesh entities
    Precise = 0,

    /// Selection using mesh nodes
    Nodes = 1,

    /// Selection using bounding box
    Box = 2,
}

impl MeshSelectionMethod {
    /// Creates a MeshSelectionMethod from an i32 value
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(MeshSelectionMethod::Precise),
            1 => Some(MeshSelectionMethod::Nodes),
            2 => Some(MeshSelectionMethod::Box),
            _ => None,
        }
    }

    /// Returns the i32 value of the method
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

impl From<i32> for MeshSelectionMethod {
    fn from(value: i32) -> Self {
        MeshSelectionMethod::from_i32(value).unwrap_or(MeshSelectionMethod::Precise)
    }
}

impl From<MeshSelectionMethod> for i32 {
    fn from(method: MeshSelectionMethod) -> Self {
        method.to_i32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_methods() {
        assert_eq!(MeshSelectionMethod::Precise.to_i32(), 0);
        assert_eq!(MeshSelectionMethod::Nodes.to_i32(), 1);
        assert_eq!(MeshSelectionMethod::Box.to_i32(), 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(MeshSelectionMethod::from_i32(0), Some(MeshSelectionMethod::Precise));
        assert_eq!(MeshSelectionMethod::from_i32(1), Some(MeshSelectionMethod::Nodes));
        assert_eq!(MeshSelectionMethod::from_i32(2), Some(MeshSelectionMethod::Box));
        assert_eq!(MeshSelectionMethod::from_i32(3), None);
    }

    #[test]
    fn test_from_into_i32() {
        let method: MeshSelectionMethod = 0i32.into();
        assert_eq!(method, MeshSelectionMethod::Precise);

        let method: MeshSelectionMethod = 1i32.into();
        assert_eq!(method, MeshSelectionMethod::Nodes);

        let method: MeshSelectionMethod = 2i32.into();
        assert_eq!(method, MeshSelectionMethod::Box);

        let value: i32 = MeshSelectionMethod::Precise.into();
        assert_eq!(value, 0);
    }

    #[test]
    fn test_equality() {
        assert_eq!(MeshSelectionMethod::Precise, MeshSelectionMethod::Precise);
        assert_ne!(MeshSelectionMethod::Precise, MeshSelectionMethod::Nodes);
        assert_ne!(MeshSelectionMethod::Nodes, MeshSelectionMethod::Box);
    }

    #[test]
    fn test_copy_clone() {
        let m1 = MeshSelectionMethod::Precise;
        let m2 = m1;
        let m3 = m1.clone();
        assert_eq!(m1, m2);
        assert_eq!(m1, m3);
    }
}
