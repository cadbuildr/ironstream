// FILE: select3_d_sensitive_poly.rs
// occt: Select3D_SensitivePoly

/// Represents the type of sensitivity for geometric entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Select3DTypeOfSensitivity {
    /// Sensitivity to boundary (edges)
    Boundary = 0,
    /// Sensitivity to interior (faces)
    Interior = 1,
}

/// A sensitive polygon entity for selection of faces and curves.
pub struct Select3DSensitivePoly {
    owner_id: Option<()>, // TODO: real owner type
    points: Vec<(f64, f64, f64)>, // TODO: replace with real gp_Pnt
    sensitivity_type: Select3DTypeOfSensitivity,
    center_of_geometry: (f64, f64, f64), // TODO: replace with real gp_Pnt
    bnd_box: Option<()>,           // TODO: replace with real Select3D_BndBox3d
    is_computed: bool,
}

impl Select3DSensitivePoly {
    /// Creates a new sensitive polygon from points.
    pub fn new(
        owner_id: Option<()>,
        points: Vec<(f64, f64, f64)>,
        bvh_enabled: bool,
    ) -> Self {
        let center = if !points.is_empty() {
            let mut cx = 0.0;
            let mut cy = 0.0;
            let mut cz = 0.0;
            for (x, y, z) in &points {
                cx += x;
                cy += y;
                cz += z;
            }
            let n = points.len() as f64;
            (cx / n, cy / n, cz / n)
        } else {
            (0.0, 0.0, 0.0)
        };

        Select3DSensitivePoly {
            owner_id,
            points,
            sensitivity_type: Select3DTypeOfSensitivity::Interior,
            center_of_geometry: center,
            bnd_box: None,
            is_computed: false,
        }
    }

    /// Returns the number of points in the polygon.
    pub fn nb_sub_elements(&self) -> usize {
        // Number of segments is equal to number of points for a closed polygon
        self.points.len()
    }

    /// Returns the center of geometry.
    pub fn center_of_geometry(&self) -> (f64, f64, f64) {
        self.center_of_geometry
    }

    /// Returns the sensitivity type.
    pub fn sensitivity_type(&self) -> Select3DTypeOfSensitivity {
        self.sensitivity_type
    }

    /// Sets the sensitivity type.
    pub fn set_sensitivity_type(&mut self, sens_type: Select3DTypeOfSensitivity) {
        self.sensitivity_type = sens_type;
    }

    /// Returns the point at the given index (0-based).
    pub fn get_point_3d(&self, idx: usize) -> Option<(f64, f64, f64)> {
        self.points.get(idx).copied()
    }

    /// Returns the array bounds [0, size-1].
    pub fn array_bounds(&self) -> (usize, usize) {
        if self.points.is_empty() {
            (0, 0)
        } else {
            (0, self.points.len() - 1)
        }
    }

    /// Returns whether the polygon has been computed/initialized.
    pub fn is_computed(&self) -> bool {
        self.is_computed
    }

    /// Marks the polygon as computed.
    pub fn set_computed(&mut self, computed: bool) {
        self.is_computed = computed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_poly() {
        let points = vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0)];
        let poly = Select3DSensitivePoly::new(None, points, true);

        assert_eq!(poly.nb_sub_elements(), 3);
    }

    #[test]
    fn test_center_of_geometry() {
        let points = vec![(0.0, 0.0, 0.0), (2.0, 0.0, 0.0), (2.0, 2.0, 0.0)];
        let poly = Select3DSensitivePoly::new(None, points, true);

        let (cx, cy, cz) = poly.center_of_geometry();
        assert!((cx - 4.0 / 3.0).abs() < 1e-10);
        assert!((cy - 2.0 / 3.0).abs() < 1e-10);
        assert!((cz - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_sensitivity_type() {
        let mut poly = Select3DSensitivePoly::new(None, vec![], true);

        assert_eq!(poly.sensitivity_type(), Select3DTypeOfSensitivity::Interior);

        poly.set_sensitivity_type(Select3DTypeOfSensitivity::Boundary);
        assert_eq!(poly.sensitivity_type(), Select3DTypeOfSensitivity::Boundary);
    }

    #[test]
    fn test_get_point_3d() {
        let points = vec![(1.0, 2.0, 3.0), (4.0, 5.0, 6.0)];
        let poly = Select3DSensitivePoly::new(None, points, true);

        assert_eq!(poly.get_point_3d(0), Some((1.0, 2.0, 3.0)));
        assert_eq!(poly.get_point_3d(1), Some((4.0, 5.0, 6.0)));
        assert_eq!(poly.get_point_3d(2), None);
    }

    #[test]
    fn test_array_bounds() {
        let points = vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (2.0, 2.0, 2.0)];
        let poly = Select3DSensitivePoly::new(None, points, true);

        let (low, high) = poly.array_bounds();
        assert_eq!(low, 0);
        assert_eq!(high, 2);
    }

    #[test]
    fn test_is_computed() {
        let mut poly = Select3DSensitivePoly::new(None, vec![], true);
        assert!(!poly.is_computed());

        poly.set_computed(true);
        assert!(poly.is_computed());
    }
}
