// FILE: select3_d_sensitive_triangle.rs
// occt: Select3D_SensitiveTriangle

/// Represents the type of sensitivity for geometric entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Select3DTypeOfSensitivity {
    /// Sensitivity to boundary (edges)
    Boundary = 0,
    /// Sensitivity to interior (faces)
    Interior = 1,
}

/// A sensitive triangle entity for selection of triangular faces.
pub struct Select3DSensitiveTriangle {
    owner_id: Option<()>, // TODO: real owner type
    points: [(f64, f64, f64); 3], // p0, p1, p2
    centroid: (f64, f64, f64),
    sensitivity_type: Select3DTypeOfSensitivity,
}

impl Select3DSensitiveTriangle {
    /// Creates a new sensitive triangle from three points.
    pub fn new(
        owner_id: Option<()>,
        p0: (f64, f64, f64),
        p1: (f64, f64, f64),
        p2: (f64, f64, f64),
        sensitivity_type: Select3DTypeOfSensitivity,
    ) -> Self {
        // Calculate centroid
        let centroid = (
            (p0.0 + p1.0 + p2.0) / 3.0,
            (p0.1 + p1.1 + p2.1) / 3.0,
            (p0.2 + p1.2 + p2.2) / 3.0,
        );

        Select3DSensitiveTriangle {
            owner_id,
            points: [p0, p1, p2],
            centroid,
            sensitivity_type,
        }
    }

    /// Returns the three points of the triangle.
    pub fn points_3d(&self) -> [(f64, f64, f64); 3] {
        self.points
    }

    /// Returns the center (centroid) of the triangle.
    pub fn center_3d(&self) -> (f64, f64, f64) {
        self.centroid
    }

    /// Returns the center of geometry (same as center_3d for triangle).
    pub fn center_of_geometry(&self) -> (f64, f64, f64) {
        self.centroid
    }

    /// Returns the number of sub-elements (always 3 for triangle).
    pub fn nb_sub_elements(&self) -> usize {
        3
    }

    /// Returns the sensitivity type.
    pub fn sensitivity_type(&self) -> Select3DTypeOfSensitivity {
        self.sensitivity_type
    }

    /// BVH tree is not needed for simple triangles.
    pub fn to_build_bvh(&self) -> bool {
        false
    }

    /// Returns the owner ID.
    pub fn owner_id(&self) -> Option<&()> {
        self.owner_id.as_ref()
    }

    /// Sets the owner ID.
    pub fn set_owner_id(&mut self, owner_id: Option<()>) {
        self.owner_id = owner_id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_triangle() {
        let p0 = (0.0, 0.0, 0.0);
        let p1 = (1.0, 0.0, 0.0);
        let p2 = (0.0, 1.0, 0.0);

        let tri =
            Select3DSensitiveTriangle::new(None, p0, p1, p2, Select3DTypeOfSensitivity::Interior);

        assert_eq!(tri.points_3d(), [p0, p1, p2]);
        assert_eq!(tri.nb_sub_elements(), 3);
    }

    #[test]
    fn test_centroid_calculation() {
        let p0 = (0.0, 0.0, 0.0);
        let p1 = (3.0, 0.0, 0.0);
        let p2 = (0.0, 3.0, 0.0);

        let tri =
            Select3DSensitiveTriangle::new(None, p0, p1, p2, Select3DTypeOfSensitivity::Interior);

        let (cx, cy, cz) = tri.center_3d();
        assert!((cx - 1.0).abs() < 1e-10);
        assert!((cy - 1.0).abs() < 1e-10);
        assert!((cz - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_center_of_geometry() {
        let p0 = (1.0, 2.0, 3.0);
        let p1 = (4.0, 5.0, 6.0);
        let p2 = (7.0, 8.0, 9.0);

        let tri =
            Select3DSensitiveTriangle::new(None, p0, p1, p2, Select3DTypeOfSensitivity::Interior);

        let cog = tri.center_of_geometry();
        assert_eq!(cog, tri.center_3d());
    }

    #[test]
    fn test_sensitivity_type() {
        let tri = Select3DSensitiveTriangle::new(
            None,
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            Select3DTypeOfSensitivity::Boundary,
        );

        assert_eq!(tri.sensitivity_type(), Select3DTypeOfSensitivity::Boundary);
    }

    #[test]
    fn test_to_build_bvh() {
        let tri = Select3DSensitiveTriangle::new(
            None,
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            Select3DTypeOfSensitivity::Interior,
        );

        assert!(!tri.to_build_bvh());
    }

    #[test]
    fn test_owner_id() {
        let mut tri = Select3DSensitiveTriangle::new(
            None,
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            Select3DTypeOfSensitivity::Interior,
        );

        assert!(tri.owner_id().is_none());

        tri.set_owner_id(Some(()));
        assert!(tri.owner_id().is_some());
    }
}
