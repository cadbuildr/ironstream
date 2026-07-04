// FILE: xcaf_doc_centroid.rs
// occt: XCAFDoc_Centroid

/// Attribute to store centroid (center of mass) point.
/// This is a specialized attribute that stores the coordinates of a centroid point
/// within the XDE document structure. The centroid is typically a 3D point (x, y, z).
#[derive(Debug, Clone, PartialEq)]
pub struct XCAFDoc_Centroid {
    /// 3D coordinates as (x, y, z)
    point: [f64; 3],
}

impl XCAFDoc_Centroid {
    /// Static GUID for XCAFDoc_Centroid attribute type
    pub const GUID: &'static str = "efd212ec-6dfd-11d4-b9c8-0060b0ee281b";

    /// Creates a new instance with zero coordinates
    pub fn new() -> Self {
        XCAFDoc_Centroid { point: [0.0, 0.0, 0.0] }
    }

    /// Creates a centroid with the given coordinates
    pub fn with_coords(x: f64, y: f64, z: f64) -> Self {
        XCAFDoc_Centroid {
            point: [x, y, z],
        }
    }

    /// Creates a centroid from a coordinate array
    pub fn from_point(point: [f64; 3]) -> Self {
        XCAFDoc_Centroid { point }
    }

    /// Sets the centroid coordinates
    pub fn set_coords(&mut self, x: f64, y: f64, z: f64) {
        self.point = [x, y, z];
    }

    /// Sets the centroid from a point array
    pub fn set_point(&mut self, point: [f64; 3]) {
        self.point = point;
    }

    /// Gets the centroid coordinates as (x, y, z)
    pub fn get_coords(&self) -> (f64, f64, f64) {
        (self.point[0], self.point[1], self.point[2])
    }

    /// Gets the centroid as a point array
    pub fn get_point(&self) -> [f64; 3] {
        self.point
    }
}

impl Default for XCAFDoc_Centroid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centroid_creation() {
        let centroid = XCAFDoc_Centroid::new();
        let (x, y, z) = centroid.get_coords();
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }

    #[test]
    fn test_centroid_with_coords() {
        let centroid = XCAFDoc_Centroid::with_coords(1.0, 2.0, 3.0);
        let (x, y, z) = centroid.get_coords();
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_centroid_set_coords() {
        let mut centroid = XCAFDoc_Centroid::new();
        centroid.set_coords(4.5, 5.5, 6.5);
        assert_eq!(centroid.get_point(), [4.5, 5.5, 6.5]);
    }

    #[test]
    fn test_centroid_from_point() {
        let point = [10.0, 20.0, 30.0];
        let centroid = XCAFDoc_Centroid::from_point(point);
        assert_eq!(centroid.get_point(), point);
    }

    #[test]
    fn test_centroid_equality() {
        let c1 = XCAFDoc_Centroid::with_coords(1.0, 2.0, 3.0);
        let c2 = XCAFDoc_Centroid::with_coords(1.0, 2.0, 3.0);
        let c3 = XCAFDoc_Centroid::with_coords(1.0, 2.0, 4.0);

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }
}
