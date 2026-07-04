// FILE: iges_solid_right_angular_wedge.rs
// occt: IGESSolid_RightAngularWedge

/// Represents a Right Angular Wedge (Type 152, Form 0 in IGESSolid).
/// A right angular wedge is a triangular/trapezoidal prism defined by
/// dimensions along local X, Y, Z axes with a corner point and axis directions.
#[derive(Debug, Clone)]
pub struct IGESSolidRightAngularWedge {
    /// Dimensions [X_big, Y, Z]
    size: [f64; 3],
    /// Smaller length along X-direction at Y=Y_length
    x_small_length: f64,
    /// Corner point coordinates
    corner: [f64; 3],
    /// Unit vector defining local X-axis
    x_axis: [f64; 3],
    /// Unit vector defining local Z-axis
    z_axis: [f64; 3],
}

impl IGESSolidRightAngularWedge {
    /// Creates a new RightAngularWedge with default values.
    pub fn new() -> Self {
        Self {
            size: [0.0, 0.0, 0.0],
            x_small_length: 0.0,
            corner: [0.0, 0.0, 0.0],
            x_axis: [1.0, 0.0, 0.0],
            z_axis: [0.0, 0.0, 1.0],
        }
    }

    /// Initializes the fields of the RightAngularWedge.
    /// - size: dimensions [X_big, Y, Z]
    /// - low_x: the length at the smaller X-side
    /// - corner: corner point coordinates (default: [0,0,0])
    /// - x_axis: unit vector defining local X-axis (default: [1,0,0])
    /// - z_axis: unit vector defining local Z-axis (default: [0,0,1])
    pub fn init(
        &mut self,
        size: [f64; 3],
        low_x: f64,
        corner: [f64; 3],
        x_axis: [f64; 3],
        z_axis: [f64; 3],
    ) {
        self.size = size;
        self.x_small_length = low_x;
        self.corner = corner;
        self.x_axis = x_axis;
        self.z_axis = z_axis;
    }

    /// Returns the size (dimensions) as [X, Y, Z].
    pub fn size(&self) -> [f64; 3] {
        self.size
    }

    /// Returns the big (longer) length along the local X-axis.
    pub fn x_big_length(&self) -> f64 {
        self.size[0]
    }

    /// Returns the smaller length along the local X-direction.
    pub fn x_small_length(&self) -> f64 {
        self.x_small_length
    }

    /// Returns the length along the local Y-axis.
    pub fn y_length(&self) -> f64 {
        self.size[1]
    }

    /// Returns the length along the local Z-axis.
    pub fn z_length(&self) -> f64 {
        self.size[2]
    }

    /// Returns the corner point coordinates.
    pub fn corner(&self) -> [f64; 3] {
        self.corner
    }

    /// Returns the direction defining the local X-axis.
    pub fn x_axis(&self) -> [f64; 3] {
        self.x_axis
    }

    /// Returns the direction defining the local Y-axis.
    /// Computed as the cross product of Z-axis and X-axis.
    pub fn y_axis(&self) -> [f64; 3] {
        cross_product(&self.z_axis, &self.x_axis)
    }

    /// Returns the direction defining the local Z-axis.
    pub fn z_axis(&self) -> [f64; 3] {
        self.z_axis
    }
}

impl Default for IGESSolidRightAngularWedge {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the cross product of two 3D vectors.
fn cross_product(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creation() {
        let wedge = IGESSolidRightAngularWedge::new();
        assert_eq!(wedge.size(), [0.0, 0.0, 0.0]);
        assert_eq!(wedge.corner(), [0.0, 0.0, 0.0]);
        assert_eq!(wedge.x_axis(), [1.0, 0.0, 0.0]);
        assert_eq!(wedge.z_axis(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_init() {
        let mut wedge = IGESSolidRightAngularWedge::new();
        wedge.init(
            [10.0, 5.0, 8.0],
            3.0,
            [1.0, 2.0, 3.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
        );

        assert_eq!(wedge.x_big_length(), 10.0);
        assert_eq!(wedge.y_length(), 5.0);
        assert_eq!(wedge.z_length(), 8.0);
        assert_eq!(wedge.x_small_length(), 3.0);
        assert_eq!(wedge.corner(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_y_axis_calculation() {
        let mut wedge = IGESSolidRightAngularWedge::new();
        wedge.init(
            [1.0, 1.0, 1.0],
            0.5,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
        );

        let y_axis = wedge.y_axis();
        // Cross product of [0,0,1] × [1,0,0] = [0,1,0]
        assert_eq!(y_axis[0], 0.0);
        assert_eq!(y_axis[1], 1.0);
        assert_eq!(y_axis[2], 0.0);
    }

    #[test]
    fn test_cross_product() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let result = cross_product(&a, &b);
        assert_eq!(result, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_cross_product_zero() {
        let a = [1.0, 0.0, 0.0];
        let result = cross_product(&a, &a);
        assert_eq!(result, [0.0, 0.0, 0.0]);
    }
}
