// FILE: n_collection_vec3.rs
// occt: NCollection_Vec3

/// 3-component vector for XYZ coordinates or RGB colors.
#[derive(Copy, Clone, Debug)]
pub struct Vec3<T> {
    pub data: [T; 3],
}

impl<T: Copy + Default> Vec3<T> {
    /// Create zero vector.
    pub fn new() -> Self {
        Vec3 {
            data: [T::default(), T::default(), T::default()],
        }
    }

    /// Create vector from components.
    pub fn from_xyz(x: T, y: T, z: T) -> Self {
        Vec3 { data: [x, y, z] }
    }

    /// Get X component.
    pub fn x(&self) -> T {
        self.data[0]
    }

    /// Get Y component.
    pub fn y(&self) -> T {
        self.data[1]
    }

    /// Get Z component.
    pub fn z(&self) -> T {
        self.data[2]
    }

    /// Set X component.
    pub fn set_x(&mut self, x: T) {
        self.data[0] = x;
    }

    /// Set Y component.
    pub fn set_y(&mut self, y: T) {
        self.data[1] = y;
    }

    /// Set Z component.
    pub fn set_z(&mut self, z: T) {
        self.data[2] = z;
    }

    /// Compute length of vector.
    pub fn length(&self) -> f64
    where
        T: Into<f64>,
    {
        let x = self.x().into();
        let y = self.y().into();
        let z = self.z().into();
        (x * x + y * y + z * z).sqrt()
    }

    /// Compute squared length (faster).
    pub fn length_squared(&self) -> f64
    where
        T: Into<f64>,
    {
        let x = self.x().into();
        let y = self.y().into();
        let z = self.z().into();
        x * x + y * y + z * z
    }

    /// Dot product.
    pub fn dot(&self, other: &Vec3<T>) -> f64
    where
        T: Into<f64>,
    {
        let x = (self.x().into()) * (other.x().into());
        let y = (self.y().into()) * (other.y().into());
        let z = (self.z().into()) * (other.z().into());
        x + y + z
    }
}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Vec3 {
            data: [T::default(), T::default(), T::default()],
        }
    }
}

impl<T: PartialEq + Copy> PartialEq for Vec3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Eq + Copy> Eq for Vec3<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vector() {
        let v: Vec3<f64> = Vec3::new();
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 0.0);
    }

    #[test]
    fn test_from_xyz() {
        let v = Vec3::from_xyz(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3::from_xyz(1.0, 2.0, 2.0);
        assert_eq!(v.length(), 3.0);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::from_xyz(1.0, 0.0, 0.0);
        let v2 = Vec3::from_xyz(2.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 2.0);
    }

    #[test]
    fn test_equality() {
        let v1 = Vec3::from_xyz(1, 2, 3);
        let v2 = Vec3::from_xyz(1, 2, 3);
        assert_eq!(v1, v2);
    }
}
