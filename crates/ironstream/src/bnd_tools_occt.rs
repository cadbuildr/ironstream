// FILE: bnd_tools_occt.rs

// occt: Bnd_Tools
//
// Static utility methods for converting between Bnd_Box formats and BVH_Box formats.
// This provides bridge functionality for bounding box interoperability.

/// Represents a 2D bounding box with min and max vectors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BvhBox2d {
    pub min: [f64; 2],
    pub max: [f64; 2],
}

impl BvhBox2d {
    pub fn new(min: [f64; 2], max: [f64; 2]) -> Self {
        BvhBox2d { min, max }
    }
}

/// Represents a 3D bounding box with min and max vectors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BvhBox3d {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl BvhBox3d {
    pub fn new(min: [f64; 3], max: [f64; 3]) -> Self {
        BvhBox3d { min, max }
    }
}

/// Represents a 2D bounding box with min/max corner coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BndBox2d {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
}

impl BndBox2d {
    pub fn new(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Self {
        BndBox2d {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }

    pub fn get(&self) -> (f64, f64, f64, f64) {
        (self.x_min, self.y_min, self.x_max, self.y_max)
    }
}

/// Represents a 3D bounding box with min/max corner coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BndBox3d {
    x_min: f64,
    y_min: f64,
    z_min: f64,
    x_max: f64,
    y_max: f64,
    z_max: f64,
}

impl BndBox3d {
    pub fn new(x_min: f64, y_min: f64, z_min: f64, x_max: f64, y_max: f64, z_max: f64) -> Self {
        BndBox3d {
            x_min,
            y_min,
            z_min,
            x_max,
            y_max,
            z_max,
        }
    }

    pub fn get(&self) -> (f64, f64, f64, f64, f64, f64) {
        (
            self.x_min, self.y_min, self.z_min, self.x_max, self.y_max, self.z_max,
        )
    }
}

/// Bnd_Tools - Static utility class for bounding box conversions.
pub struct BndTools;

impl BndTools {
    /// Converts a Bnd_Box2d to BVH_Box<double, 2>.
    ///
    /// Extracts the min and max coordinates from the 2D bounding box
    /// and constructs a BVH_Box with corresponding vectors.
    pub fn bnd_2_bvh_2d(the_box: &BndBox2d) -> BvhBox2d {
        let (x_min, y_min, x_max, y_max) = the_box.get();
        BvhBox2d::new([x_min, y_min], [x_max, y_max])
    }

    /// Converts a Bnd_Box to BVH_Box<double, 3>.
    ///
    /// Extracts the min and max coordinates from the 3D bounding box
    /// and constructs a BVH_Box with corresponding vectors.
    pub fn bnd_2_bvh_3d(the_box: &BndBox3d) -> BvhBox3d {
        let (x_min, y_min, z_min, x_max, y_max, z_max) = the_box.get();
        BvhBox3d::new([x_min, y_min, z_min], [x_max, y_max, z_max])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bnd_2d_to_bvh_2d() {
        let bnd_box = BndBox2d::new(1.0, 2.0, 3.0, 4.0);
        let bvh_box = BndTools::bnd_2_bvh_2d(&bnd_box);

        assert_eq!(bvh_box.min, [1.0, 2.0]);
        assert_eq!(bvh_box.max, [3.0, 4.0]);
    }

    #[test]
    fn test_bnd_2d_to_bvh_2d_negative_coords() {
        let bnd_box = BndBox2d::new(-5.0, -10.0, 5.0, 10.0);
        let bvh_box = BndTools::bnd_2_bvh_2d(&bnd_box);

        assert_eq!(bvh_box.min, [-5.0, -10.0]);
        assert_eq!(bvh_box.max, [5.0, 10.0]);
    }

    #[test]
    fn test_bnd_3d_to_bvh_3d() {
        let bnd_box = BndBox3d::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let bvh_box = BndTools::bnd_2_bvh_3d(&bnd_box);

        assert_eq!(bvh_box.min, [1.0, 2.0, 3.0]);
        assert_eq!(bvh_box.max, [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_bnd_3d_to_bvh_3d_negative_coords() {
        let bnd_box = BndBox3d::new(-1.0, -2.0, -3.0, 1.0, 2.0, 3.0);
        let bvh_box = BndTools::bnd_2_bvh_3d(&bnd_box);

        assert_eq!(bvh_box.min, [-1.0, -2.0, -3.0]);
        assert_eq!(bvh_box.max, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_bvh_box_2d_construction() {
        let bvh_box = BvhBox2d::new([0.0, 0.0], [10.0, 10.0]);
        assert_eq!(bvh_box.min[0], 0.0);
        assert_eq!(bvh_box.min[1], 0.0);
        assert_eq!(bvh_box.max[0], 10.0);
        assert_eq!(bvh_box.max[1], 10.0);
    }

    #[test]
    fn test_bvh_box_3d_construction() {
        let bvh_box = BvhBox3d::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
        assert_eq!(bvh_box.min[0], 1.0);
        assert_eq!(bvh_box.min[1], 2.0);
        assert_eq!(bvh_box.min[2], 3.0);
        assert_eq!(bvh_box.max[0], 4.0);
        assert_eq!(bvh_box.max[1], 5.0);
        assert_eq!(bvh_box.max[2], 6.0);
    }
}
