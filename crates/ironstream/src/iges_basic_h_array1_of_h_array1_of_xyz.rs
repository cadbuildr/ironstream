// FILE: iges_basic_h_array1_of_h_array1_of_xyz.rs
// occt: IGESBasic_HArray1OfHArray1OfXYZ

/// 3D point representation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpXYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GpXYZ {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Handle Array of Handle Array of XYZ (3D points).
pub struct IgesBasicHArray1OfHArray1OfXYZ {
    low: i32,
    up: i32,
    values: Vec<Vec<GpXYZ>>,
}

impl IgesBasicHArray1OfHArray1OfXYZ {
    /// Create a new array with bounds [low, up].
    pub fn new(low: i32, up: i32) -> Self {
        let len = (up - low + 1) as usize;
        Self {
            low,
            up,
            values: vec![Vec::new(); len],
        }
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> i32 {
        self.low
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> i32 {
        self.up
    }

    /// Returns the length of the array.
    pub fn length(&self) -> i32 {
        self.up - self.low + 1
    }

    /// Set value at position num.
    pub fn set_value(&mut self, num: i32, val: Vec<GpXYZ>) {
        if num >= self.low && num <= self.up {
            let idx = (num - self.low) as usize;
            self.values[idx] = val;
        }
    }

    /// Get value at position num.
    pub fn value(&self, num: i32) -> Option<Vec<GpXYZ>> {
        if num >= self.low && num <= self.up {
            let idx = (num - self.low) as usize;
            Some(self.values[idx].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr = IgesBasicHArray1OfHArray1OfXYZ::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_set_and_get_value() {
        let mut arr = IgesBasicHArray1OfHArray1OfXYZ::new(1, 3);
        let data = vec![GpXYZ::new(1.0, 2.0, 3.0), GpXYZ::new(4.0, 5.0, 6.0)];
        arr.set_value(1, data.clone());
        assert_eq!(arr.value(1), Some(data));
    }

    #[test]
    fn test_bounds() {
        let arr = IgesBasicHArray1OfHArray1OfXYZ::new(0, 9);
        assert_eq!(arr.lower(), 0);
        assert_eq!(arr.upper(), 9);
        assert_eq!(arr.length(), 10);
    }

    #[test]
    fn test_negative_bounds() {
        let arr = IgesBasicHArray1OfHArray1OfXYZ::new(-5, 5);
        assert_eq!(arr.lower(), -5);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 11);
    }

    #[test]
    fn test_out_of_bounds() {
        let arr = IgesBasicHArray1OfHArray1OfXYZ::new(1, 3);
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(4), None);
    }

    #[test]
    fn test_multiple_values() {
        let mut arr = IgesBasicHArray1OfHArray1OfXYZ::new(1, 3);
        arr.set_value(1, vec![GpXYZ::new(1.0, 2.0, 3.0)]);
        arr.set_value(2, vec![GpXYZ::new(4.0, 5.0, 6.0), GpXYZ::new(7.0, 8.0, 9.0)]);
        arr.set_value(3, vec![GpXYZ::new(10.0, 11.0, 12.0)]);
        assert_eq!(arr.value(1), Some(vec![GpXYZ::new(1.0, 2.0, 3.0)]));
        assert_eq!(arr.value(2), Some(vec![GpXYZ::new(4.0, 5.0, 6.0), GpXYZ::new(7.0, 8.0, 9.0)]));
    }
}
