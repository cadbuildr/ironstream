// FILE: draw_marker3_d.rs
// occt: Draw_Marker3D

//! A drawable 3D marker for the Draw application.

/// Represents a 3D marker drawable
pub struct DrawMarker3D {
    /// Position X coordinate
    x: f64,
    /// Position Y coordinate
    y: f64,
    /// Position Z coordinate
    z: f64,
    /// Marker shape
    shape: u32,
    /// Marker color
    color: u32,
    /// Marker integer size
    i_size: i32,
    /// Marker real size
    r_size: f64,
    /// Is real size used
    is_r_size: bool,
}

impl DrawMarker3D {
    /// Create a new 3D marker with integer size
    pub fn new(x: f64, y: f64, z: f64, shape: u32, color: u32, size: i32) -> Self {
        DrawMarker3D {
            x,
            y,
            z,
            shape,
            color,
            i_size: size,
            r_size: 0.0,
            is_r_size: false,
        }
    }

    /// Create a new 3D marker with real size
    pub fn new_with_real_size(x: f64, y: f64, z: f64, shape: u32, color: u32, size: f64) -> Self {
        DrawMarker3D {
            x,
            y,
            z,
            shape,
            color,
            i_size: 0,
            r_size: size,
            is_r_size: true,
        }
    }

    /// Get the position
    pub fn position(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Set the position
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Get the marker shape
    pub fn shape(&self) -> u32 {
        self.shape
    }

    /// Get the color
    pub fn color(&self) -> u32 {
        self.color
    }

    /// Get the integer size
    pub fn size(&self) -> i32 {
        self.i_size
    }

    /// Get the real size
    pub fn real_size(&self) -> f64 {
        self.r_size
    }

    /// Check if real size is used
    pub fn is_real_size(&self) -> bool {
        self.is_r_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker3d_int_size() {
        let marker = DrawMarker3D::new(1.0, 2.0, 3.0, 0, 0xFF0000, 5);
        assert_eq!(marker.position(), (1.0, 2.0, 3.0));
        assert_eq!(marker.size(), 5);
        assert!(!marker.is_real_size());
    }

    #[test]
    fn test_marker3d_real_size() {
        let marker = DrawMarker3D::new_with_real_size(10.0, 20.0, 30.0, 4, 0x00FF00, 7.5);
        assert_eq!(marker.position(), (10.0, 20.0, 30.0));
        assert_eq!(marker.real_size(), 7.5);
        assert!(marker.is_real_size());
    }

    #[test]
    fn test_marker3d_position_change() {
        let mut marker = DrawMarker3D::new(0.0, 0.0, 0.0, 0, 0xFFFFFF, 3);
        marker.set_position(5.0, 10.0, 15.0);
        assert_eq!(marker.position(), (5.0, 10.0, 15.0));
    }
}
