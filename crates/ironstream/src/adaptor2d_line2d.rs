// FILE: adaptor2d_line2d.rs
// occt: Adaptor2d_Line2d

/// A 2D line adaptor for curve operations
#[derive(Debug, Clone)]
pub struct Adaptor2dLine2d {
    x0: f64,
    y0: f64,
    dx: f64,
    dy: f64,
    u_first: f64,
    u_last: f64,
}

impl Adaptor2dLine2d {
    /// Create an empty line adaptor
    pub fn new() -> Self {
        Adaptor2dLine2d {
            x0: 0.0,
            y0: 0.0,
            dx: 1.0,
            dy: 0.0,
            u_first: 0.0,
            u_last: 1.0,
        }
    }

    /// Create with line parameters
    pub fn with_line(px: f64, py: f64, dx: f64, dy: f64, u_first: f64, u_last: f64) -> Self {
        Adaptor2dLine2d {
            x0: px,
            y0: py,
            dx,
            dy,
            u_first,
            u_last,
        }
    }

    /// Load line data
    pub fn load(&mut self, px: f64, py: f64, dx: f64, dy: f64) {
        self.x0 = px;
        self.y0 = py;
        self.dx = dx;
        self.dy = dy;
    }

    /// Load with parameter range
    pub fn load_range(&mut self, u_first: f64, u_last: f64) {
        self.u_first = u_first;
        self.u_last = u_last;
    }

    /// Get first parameter
    pub fn first_parameter(&self) -> f64 {
        self.u_first
    }

    /// Get last parameter
    pub fn last_parameter(&self) -> f64 {
        self.u_last
    }

    /// Get continuity (always C_Infinity for lines)
    pub fn continuity(&self) -> &'static str {
        "C_Infinity"
    }

    /// Evaluate point at parameter
    pub fn value(&self, t: f64) -> (f64, f64) {
        (
            self.x0 + t * self.dx,
            self.y0 + t * self.dy,
        )
    }

    /// Get the direction vector
    pub fn direction(&self) -> (f64, f64) {
        (self.dx, self.dy)
    }

    /// Get the starting point
    pub fn start_point(&self) -> (f64, f64) {
        (self.x0, self.y0)
    }
}

impl Default for Adaptor2dLine2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line2d_new() {
        let line = Adaptor2dLine2d::new();
        assert_eq!(line.first_parameter(), 0.0);
        assert_eq!(line.last_parameter(), 1.0);
    }

    #[test]
    fn test_line2d_with_line() {
        let line = Adaptor2dLine2d::with_line(1.0, 2.0, 1.0, 0.0, 0.0, 10.0);
        assert_eq!(line.start_point(), (1.0, 2.0));
        assert_eq!(line.direction(), (1.0, 0.0));
    }

    #[test]
    fn test_line2d_value() {
        let line = Adaptor2dLine2d::with_line(0.0, 0.0, 1.0, 1.0, 0.0, 1.0);
        let (x, y) = line.value(0.5);
        assert!((x - 0.5).abs() < 0.0001);
        assert!((y - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_line2d_continuity() {
        let line = Adaptor2dLine2d::new();
        assert_eq!(line.continuity(), "C_Infinity");
    }

    #[test]
    fn test_line2d_load_range() {
        let mut line = Adaptor2dLine2d::new();
        line.load_range(1.0, 5.0);
        assert_eq!(line.first_parameter(), 1.0);
        assert_eq!(line.last_parameter(), 5.0);
    }
}
