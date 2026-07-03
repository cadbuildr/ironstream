// FILE: b_rep_mesh_degree_of_freedom.rs
// occt: BRepMesh_DegreeOfFreedom

/// Enumeration representing the degree of freedom of a mesh node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DegreeOfFreedom {
    /// Node is free to move in any direction
    Free = 0,
    /// Node is constrained within volume
    InVolume = 1,
    /// Node is constrained on a surface
    OnSurface = 2,
    /// Node is constrained on a curve
    OnCurve = 3,
    /// Node is fixed and cannot move
    Fixed = 4,
    /// Node is on the frontier/boundary
    Frontier = 5,
    /// Node has been deleted
    Deleted = 6,
}

impl DegreeOfFreedom {
    /// Returns the integer value of the enum variant.
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    /// Creates from integer value.
    pub fn from_i32(value: i32) -> Result<Self, String> {
        match value {
            0 => Ok(DegreeOfFreedom::Free),
            1 => Ok(DegreeOfFreedom::InVolume),
            2 => Ok(DegreeOfFreedom::OnSurface),
            3 => Ok(DegreeOfFreedom::OnCurve),
            4 => Ok(DegreeOfFreedom::Fixed),
            5 => Ok(DegreeOfFreedom::Frontier),
            6 => Ok(DegreeOfFreedom::Deleted),
            _ => Err(format!("Invalid DegreeOfFreedom value: {}", value)),
        }
    }
}

impl Default for DegreeOfFreedom {
    fn default() -> Self {
        DegreeOfFreedom::Free
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_of_freedom_variants() {
        assert_eq!(DegreeOfFreedom::Free.as_i32(), 0);
        assert_eq!(DegreeOfFreedom::InVolume.as_i32(), 1);
        assert_eq!(DegreeOfFreedom::OnSurface.as_i32(), 2);
        assert_eq!(DegreeOfFreedom::OnCurve.as_i32(), 3);
        assert_eq!(DegreeOfFreedom::Fixed.as_i32(), 4);
        assert_eq!(DegreeOfFreedom::Frontier.as_i32(), 5);
        assert_eq!(DegreeOfFreedom::Deleted.as_i32(), 6);
    }

    #[test]
    fn test_degree_of_freedom_from_i32() {
        assert_eq!(
            DegreeOfFreedom::from_i32(0).unwrap(),
            DegreeOfFreedom::Free
        );
        assert_eq!(
            DegreeOfFreedom::from_i32(2).unwrap(),
            DegreeOfFreedom::OnSurface
        );
        assert_eq!(
            DegreeOfFreedom::from_i32(4).unwrap(),
            DegreeOfFreedom::Fixed
        );
        assert!(DegreeOfFreedom::from_i32(99).is_err());
    }

    #[test]
    fn test_degree_of_freedom_default() {
        assert_eq!(DegreeOfFreedom::default(), DegreeOfFreedom::Free);
    }

    #[test]
    fn test_degree_of_freedom_equality() {
        let d1 = DegreeOfFreedom::OnSurface;
        let d2 = DegreeOfFreedom::OnSurface;
        assert_eq!(d1, d2);
    }
}
