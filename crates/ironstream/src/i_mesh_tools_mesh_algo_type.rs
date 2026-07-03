// FILE: i_mesh_tools_mesh_algo_type.rs
// occt: IMeshTools_MeshAlgoType

/// Enumerates built-in meshing algorithms factories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IMeshToolsMeshAlgoType {
    /// Use global default (Watson or CSF_MeshAlgo).
    Default = -1,
    /// Generate 2D Delaunay triangulation based on Watson algorithm.
    Watson = 0,
    /// Generate 2D Delaunay triangulation based on Delabella algorithm.
    Delabella = 1,
}

impl IMeshToolsMeshAlgoType {
    /// Convert from i32 value.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            -1 => Some(IMeshToolsMeshAlgoType::Default),
            0 => Some(IMeshToolsMeshAlgoType::Watson),
            1 => Some(IMeshToolsMeshAlgoType::Delabella),
            _ => None,
        }
    }

    /// Convert to i32 value.
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(
            IMeshToolsMeshAlgoType::from_i32(-1),
            Some(IMeshToolsMeshAlgoType::Default)
        );
        assert_eq!(
            IMeshToolsMeshAlgoType::from_i32(0),
            Some(IMeshToolsMeshAlgoType::Watson)
        );
        assert_eq!(
            IMeshToolsMeshAlgoType::from_i32(1),
            Some(IMeshToolsMeshAlgoType::Delabella)
        );
        assert_eq!(IMeshToolsMeshAlgoType::from_i32(99), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(IMeshToolsMeshAlgoType::Default.to_i32(), -1);
        assert_eq!(IMeshToolsMeshAlgoType::Watson.to_i32(), 0);
        assert_eq!(IMeshToolsMeshAlgoType::Delabella.to_i32(), 1);
    }
}
