// FILE: iges_solid_torus.rs
// occt: IGESSolid_Torus

/// Defines Torus, Type <160> Form Number <0>
/// A Torus is a solid formed by revolving a circular disc
/// about a specified coplanar axis.
#[derive(Clone, Debug)]
pub struct IgesSolidTorus {
    major_radius: f64,
    disc_radius: f64,
    axis_point: [f64; 3],
    axis: [f64; 3],
}

impl IgesSolidTorus {
    /// Creates a new Torus with default values
    pub fn new() -> Self {
        Self {
            major_radius: 0.0,
            disc_radius: 0.0,
            axis_point: [0.0, 0.0, 0.0],
            axis: [0.0, 0.0, 1.0],
        }
    }

    /// This method is used to set the fields of the class Torus
    /// - r1      : distance from center of torus to center of circular disc to be revolved
    /// - r2      : radius of circular disc
    /// - point   : center point coordinates (default [0,0,0])
    /// - axis    : unit vector in axis direction (default [0,0,1])
    pub fn init(
        &mut self,
        r1: f64,
        r2: f64,
        point: [f64; 3],
        axis: [f64; 3],
    ) {
        self.major_radius = r1;
        self.disc_radius = r2;
        self.axis_point = point;
        // Normalize axis if needed
        let magnitude_sq = axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2];
        if magnitude_sq > 0.0 && (magnitude_sq - 1.0).abs() > 1.0e-5 {
            let magnitude = magnitude_sq.sqrt();
            self.axis = [
                axis[0] / magnitude,
                axis[1] / magnitude,
                axis[2] / magnitude,
            ];
        } else {
            self.axis = axis;
        }
    }

    /// Returns the distance from the center of torus to the center of
    /// the disc to be revolved
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// Returns the radius of the disc to be revolved
    pub fn disc_radius(&self) -> f64 {
        self.disc_radius
    }

    /// Returns the center of torus
    pub fn axis_point(&self) -> [f64; 3] {
        self.axis_point
    }

    /// Returns direction of the axis (unit vector)
    pub fn axis(&self) -> [f64; 3] {
        self.axis
    }

    /// Returns the center of torus after applying TransformationMatrix
    /// (stub: returns the same as axis_point for now)
    pub fn transformed_axis_point(&self) -> [f64; 3] {
        self.axis_point
    }

    /// Returns direction of the axis after applying TransformationMatrix
    /// (stub: returns the same as axis for now)
    pub fn transformed_axis(&self) -> [f64; 3] {
        self.axis
    }
}

impl Default for IgesSolidTorus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let torus = IgesSolidTorus::new();
        assert_eq!(torus.major_radius(), 0.0);
        assert_eq!(torus.disc_radius(), 0.0);
        assert_eq!(torus.axis_point(), [0.0, 0.0, 0.0]);
        assert_eq!(torus.axis(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_init_with_values() {
        let mut torus = IgesSolidTorus::new();
        torus.init(5.0, 2.0, [1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);

        assert_eq!(torus.major_radius(), 5.0);
        assert_eq!(torus.disc_radius(), 2.0);
        assert_eq!(torus.axis_point(), [1.0, 2.0, 3.0]);
        assert_eq!(torus.axis(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_axis_normalization() {
        let mut torus = IgesSolidTorus::new();
        torus.init(5.0, 2.0, [0.0, 0.0, 0.0], [0.0, 0.0, 2.0]);

        let axis = torus.axis();
        // Should be normalized to [0, 0, 1]
        assert!((axis[2] - 1.0).abs() < 1.0e-5);
    }

    #[test]
    fn test_axis_not_normalized_if_close_to_unit() {
        let mut torus = IgesSolidTorus::new();
        torus.init(5.0, 2.0, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);

        let axis = torus.axis();
        assert_eq!(axis, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_transformed_returns_same_as_untransformed() {
        let mut torus = IgesSolidTorus::new();
        torus.init(5.0, 2.0, [1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);

        assert_eq!(torus.axis_point(), torus.transformed_axis_point());
        assert_eq!(torus.axis(), torus.transformed_axis());
    }

    #[test]
    fn test_clone() {
        let mut torus = IgesSolidTorus::new();
        torus.init(5.0, 2.0, [1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);

        let cloned = torus.clone();
        assert_eq!(cloned.major_radius(), 5.0);
        assert_eq!(cloned.disc_radius(), 2.0);
        assert_eq!(cloned.axis_point(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_radii_stored_correctly() {
        let mut torus = IgesSolidTorus::new();
        torus.init(10.5, 3.2, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);

        assert_eq!(torus.major_radius(), 10.5);
        assert_eq!(torus.disc_radius(), 3.2);
    }

    #[test]
    fn test_point_coordinate() {
        let mut torus = IgesSolidTorus::new();
        torus.init(5.0, 2.0, [4.5, 5.5, 6.5], [0.0, 0.0, 1.0]);

        let point = torus.axis_point();
        assert_eq!(point[0], 4.5);
        assert_eq!(point[1], 5.5);
        assert_eq!(point[2], 6.5);
    }
}
