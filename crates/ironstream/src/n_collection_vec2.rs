// FILE: n_collection_vec2.rs
// occt: NCollection_Vec2

/// 2-component vector for XY coordinates or similar.
#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub data: [T; 2],
}

impl<T: Copy + Default> Vec2<T> {
    /// Create zero vector.
    pub fn new() -> Self {
        Vec2 {
            data: [T::default(), T::default()],
        }
    }

    /// Create vector from components.
    pub fn from_xy(x: T, y: T) -> Self {
        Vec2 { data: [x, y] }
    }

    /// Get X component.
    pub fn x(&self) -> T {
        self.data[0]
    }

    /// Get Y component.
    pub fn y(&self) -> T {
        self.data[1]
    }

    /// Set X component.
    pub fn set_x(&mut self, x: T) {
        self.data[0] = x;
    }

    /// Set Y component.
    pub fn set_y(&mut self, y: T) {
        self.data[1] = y;
    }

    /// Length of vector.
    pub fn length(&self) -> f64
    where
        T: Into<f64>,
    {
        let x = self.x().into();
        let y = self.y().into();
        (x * x + y * y).sqrt()
    }

    /// Squared length (faster, no sqrt).
    pub fn length_squared(&self) -> f64
    where
        T: Into<f64>,
    {
        let x = self.x().into();
        let y = self.y().into();
        x * x + y * y
    }
}

impl<T: Default> Default for Vec2<T> {
    fn default() -> Self {
        Vec2 {
            data: [T::default(), T::default()],
        }
    }
}

impl<T: PartialEq + Copy> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Eq + Copy> Eq for Vec2<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vector() {
        let v: Vec2<f64> = Vec2::new();
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
    }

    #[test]
    fn test_from_xy() {
        let v = Vec2::from_xy(3.0, 4.0);
        assert_eq!(v.x(), 3.0);
        assert_eq!(v.y(), 4.0);
    }

    #[test]
    fn test_length() {
        let v = Vec2::from_xy(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_equality() {
        let v1 = Vec2::from_xy(1, 2);
        let v2 = Vec2::from_xy(1, 2);
        assert_eq!(v1, v2);
    }
}
