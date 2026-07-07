// FILE: iges_solid_block.rs
// occt: IGESSolid_Block

//! Block entity: a rectangular parallelepiped (IGES Type 150, Form 0).
//!
//! Defined by one corner at (X1, Y1, Z1) and three edges along local +X, +Y, +Z axes.
//! The block is axis-aligned in the local coordinate system.

/// Three-dimensional vector with X, Y, Z components
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3D { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_x() -> Self {
        Vec3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_z() -> Self {
        Vec3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    /// Cross product: self ^ other
    pub fn cross(&self, other: &Vec3D) -> Vec3D {
        Vec3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Normalized vector (unit direction)
    pub fn normalized(&self) -> Vec3D {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 0.0 {
            Vec3D {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }

    /// Magnitude/length of vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

/// Three-dimensional point
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn from_vec(v: Vec3D) -> Self {
        Point3D {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    /// Apply affine transformation: new_point = point + offset
    pub fn translate(&self, offset: &Vec3D) -> Point3D {
        Point3D {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}

/// Transformation matrix (simplified 3x4 transformation)
#[derive(Clone, Debug)]
pub struct GTrsf {
    matrix: [[f64; 4]; 3],
}

impl GTrsf {
    pub fn identity() -> Self {
        GTrsf {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        }
    }

    /// Transform a vector (ignoring translation)
    pub fn transform_direction(&self, vec: &Vec3D) -> Vec3D {
        Vec3D {
            x: self.matrix[0][0] * vec.x
                + self.matrix[0][1] * vec.y
                + self.matrix[0][2] * vec.z,
            y: self.matrix[1][0] * vec.x
                + self.matrix[1][1] * vec.y
                + self.matrix[1][2] * vec.z,
            z: self.matrix[2][0] * vec.x
                + self.matrix[2][1] * vec.y
                + self.matrix[2][2] * vec.z,
        }
    }

    /// Transform a point (with translation)
    pub fn transform_point(&self, pt: &Point3D) -> Point3D {
        Point3D {
            x: self.matrix[0][0] * pt.x
                + self.matrix[0][1] * pt.y
                + self.matrix[0][2] * pt.z
                + self.matrix[0][3],
            y: self.matrix[1][0] * pt.x
                + self.matrix[1][1] * pt.y
                + self.matrix[1][2] * pt.z
                + self.matrix[1][3],
            z: self.matrix[2][0] * pt.x
                + self.matrix[2][1] * pt.y
                + self.matrix[2][2] * pt.z
                + self.matrix[2][3],
        }
    }

    /// Set translation part to zero
    pub fn set_translation_zero(&mut self) {
        self.matrix[0][3] = 0.0;
        self.matrix[1][3] = 0.0;
        self.matrix[2][3] = 0.0;
    }
}

/// IGES Entity base
pub struct IGESEntity {
    type_number: i32,
    form_number: i32,
    has_transformation: bool,
    transformation: Option<GTrsf>,
}

impl IGESEntity {
    pub fn new() -> Self {
        IGESEntity {
            type_number: 0,
            form_number: 0,
            has_transformation: false,
            transformation: None,
        }
    }

    pub fn init_type_and_form(&mut self, type_num: i32, form_num: i32) {
        self.type_number = type_num;
        self.form_number = form_num;
    }

    pub fn has_transformation(&self) -> bool {
        self.has_transformation
    }

    pub fn transformation(&self) -> Option<&GTrsf> {
        self.transformation.as_ref()
    }

    pub fn set_transformation(&mut self, transf: GTrsf) {
        self.has_transformation = true;
        self.transformation = Some(transf);
    }

    pub fn type_number(&self) -> i32 {
        self.type_number
    }

    pub fn form_number(&self) -> i32 {
        self.form_number
    }
}

/// Block entity: rectangular parallelepiped
pub struct IGESSolidBlock {
    entity: IGESEntity,
    size: Vec3D,
    corner: Vec3D,
    x_axis: Vec3D,
    z_axis: Vec3D,
}

impl IGESSolidBlock {
    /// Creates a new block with default values
    pub fn new() -> Self {
        IGESSolidBlock {
            entity: IGESEntity::new(),
            size: Vec3D::zero(),
            corner: Vec3D::zero(),
            x_axis: Vec3D::unit_x(),
            z_axis: Vec3D::unit_z(),
        }
    }

    /// Initializes the block with size, corner, and axis information
    pub fn init(&mut self, size: Vec3D, corner: Vec3D, x_axis: Vec3D, z_axis: Vec3D) {
        self.size = size;
        self.corner = corner;
        self.x_axis = x_axis;
        self.z_axis = z_axis;
        self.entity.init_type_and_form(150, 0);
    }

    /// Returns the size of the block
    pub fn size(&self) -> Vec3D {
        self.size
    }

    /// Returns the length along local X-direction
    pub fn x_length(&self) -> f64 {
        self.size.x
    }

    /// Returns the length along local Y-direction
    pub fn y_length(&self) -> f64 {
        self.size.y
    }

    /// Returns the length along local Z-direction
    pub fn z_length(&self) -> f64 {
        self.size.z
    }

    /// Returns the corner point
    pub fn corner(&self) -> Point3D {
        Point3D::from_vec(self.corner)
    }

    /// Returns the corner point after transformation
    pub fn transformed_corner(&self) -> Point3D {
        if !self.entity.has_transformation() {
            Point3D::from_vec(self.corner)
        } else if let Some(transf) = &self.entity.transformation {
            transf.transform_point(&Point3D::from_vec(self.corner))
        } else {
            Point3D::from_vec(self.corner)
        }
    }

    /// Returns the X-axis direction
    pub fn x_axis(&self) -> Vec3D {
        self.x_axis.normalized()
    }

    /// Returns the X-axis direction after transformation
    pub fn transformed_x_axis(&self) -> Vec3D {
        if !self.entity.has_transformation() {
            self.x_axis.normalized()
        } else if let Some(transf) = &self.entity.transformation {
            transf.transform_direction(&self.x_axis).normalized()
        } else {
            self.x_axis.normalized()
        }
    }

    /// Returns the Y-axis direction (cross product of X and Z,
    /// as in OCCT IGESSolid_Block::YAxis: theXAxis ^ theZAxis)
    pub fn y_axis(&self) -> Vec3D {
        self.x_axis.cross(&self.z_axis).normalized()
    }

    /// Returns the Y-axis direction after transformation
    pub fn transformed_y_axis(&self) -> Vec3D {
        if !self.entity.has_transformation() {
            self.x_axis.cross(&self.z_axis).normalized()
        } else if let Some(transf) = &self.entity.transformation {
            let y = self.x_axis.cross(&self.z_axis);
            transf.transform_direction(&y).normalized()
        } else {
            self.x_axis.cross(&self.z_axis).normalized()
        }
    }

    /// Returns the Z-axis direction
    pub fn z_axis(&self) -> Vec3D {
        self.z_axis.normalized()
    }

    /// Returns the Z-axis direction after transformation
    pub fn transformed_z_axis(&self) -> Vec3D {
        if !self.entity.has_transformation() {
            self.z_axis.normalized()
        } else if let Some(transf) = &self.entity.transformation {
            transf.transform_direction(&self.z_axis).normalized()
        } else {
            self.z_axis.normalized()
        }
    }

    pub fn set_transformation(&mut self, transf: GTrsf) {
        self.entity.set_transformation(transf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3d_creation() {
        let v = Vec3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vec3d_zero() {
        let v = Vec3D::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_vec3d_unit_x() {
        let v = Vec3D::unit_x();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_vec3d_unit_z() {
        let v = Vec3D::unit_z();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn test_vec3d_magnitude() {
        let v = Vec3D::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3d_cross_product() {
        let x = Vec3D::unit_x();
        let z = Vec3D::unit_z();
        let y = x.cross(&z);

        assert!((y.y - (-1.0)).abs() < 1e-10);
        assert!(y.x.abs() < 1e-10);
        assert!(y.z.abs() < 1e-10);
    }

    #[test]
    fn test_vec3d_normalized() {
        let v = Vec3D::new(3.0, 4.0, 0.0);
        let n = v.normalized();

        assert!((n.x - 0.6).abs() < 1e-10);
        assert!((n.y - 0.8).abs() < 1e-10);
        assert!(n.z.abs() < 1e-10);
    }

    #[test]
    fn test_point3d_creation() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_point3d_from_vec() {
        let v = Vec3D::new(1.0, 2.0, 3.0);
        let p = Point3D::from_vec(v);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_block_creation() {
        let block = IGESSolidBlock::new();
        assert_eq!(block.size(), Vec3D::zero());
    }

    #[test]
    fn test_block_init() {
        let mut block = IGESSolidBlock::new();
        let size = Vec3D::new(2.0, 3.0, 4.0);
        let corner = Vec3D::new(0.0, 0.0, 0.0);
        let x_axis = Vec3D::unit_x();
        let z_axis = Vec3D::unit_z();

        block.init(size, corner, x_axis, z_axis);

        assert_eq!(block.size(), size);
        assert_eq!(block.x_length(), 2.0);
        assert_eq!(block.y_length(), 3.0);
        assert_eq!(block.z_length(), 4.0);
    }

    #[test]
    fn test_block_corner() {
        let mut block = IGESSolidBlock::new();
        let size = Vec3D::new(1.0, 1.0, 1.0);
        let corner = Vec3D::new(1.0, 2.0, 3.0);

        block.init(size, corner, Vec3D::unit_x(), Vec3D::unit_z());

        let c = block.corner();
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 2.0);
        assert_eq!(c.z, 3.0);
    }

    #[test]
    fn test_block_axes() {
        let mut block = IGESSolidBlock::new();
        block.init(
            Vec3D::new(1.0, 1.0, 1.0),
            Vec3D::zero(),
            Vec3D::unit_x(),
            Vec3D::unit_z(),
        );

        let x = block.x_axis();
        let z = block.z_axis();
        let y = block.y_axis();

        assert!((x.x - 1.0).abs() < 1e-10);
        assert!((z.z - 1.0).abs() < 1e-10);
        assert!(y.y.abs() < 1e-10 || (y.y + 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_block_transformed_corner_no_transform() {
        let mut block = IGESSolidBlock::new();
        let corner_vec = Vec3D::new(1.0, 2.0, 3.0);
        block.init(
            Vec3D::new(1.0, 1.0, 1.0),
            corner_vec,
            Vec3D::unit_x(),
            Vec3D::unit_z(),
        );

        let tc = block.transformed_corner();
        assert_eq!(tc.x, 1.0);
        assert_eq!(tc.y, 2.0);
        assert_eq!(tc.z, 3.0);
    }

    #[test]
    fn test_gtrfs_identity() {
        let t = GTrsf::identity();
        let v = Vec3D::new(1.0, 2.0, 3.0);
        let tv = t.transform_direction(&v);

        assert!((tv.x - 1.0).abs() < 1e-10);
        assert!((tv.y - 2.0).abs() < 1e-10);
        assert!((tv.z - 3.0).abs() < 1e-10);
    }
}
