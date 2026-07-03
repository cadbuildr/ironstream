// FILE: gcc_ana_pnt2d_bisec.rs
// occt: GccAna_Pnt2dBisec

/// Bisecting line between two 2D points.
pub struct GccAnaPnt2dBisec {
    well_done: bool,
    has_solution: bool,
    line_x0: f64,
    line_y0: f64,
    line_dx: f64,
    line_dy: f64,
}

impl GccAnaPnt2dBisec {
    /// Constructs a bisecting line between two points.
    pub fn new(p1_x: f64, p1_y: f64, p2_x: f64, p2_y: f64) -> Self {
        let dx = p2_x - p1_x;
        let dy = p2_y - p1_y;

        let has_sol = dx * dx + dy * dy > 1e-20;

        let (center_x, center_y) = ((p1_x + p2_x) / 2.0, (p1_y + p2_y) / 2.0);
        let (line_dx, line_dy) = if has_sol {
            // Perpendicular to the segment
            (-dy, dx)
        } else {
            (1.0, 0.0)
        };

        // Normalize
        let len = (line_dx * line_dx + line_dy * line_dy).sqrt();
        let line_dx_n = if len > 1e-15 { line_dx / len } else { 1.0 };
        let line_dy_n = if len > 1e-15 { line_dy / len } else { 0.0 };

        GccAnaPnt2dBisec {
            well_done: true,
            has_solution: has_sol,
            line_x0: center_x,
            line_y0: center_y,
            line_dx: line_dx_n,
            line_dy: line_dy_n,
        }
    }

    pub fn is_done(&self) -> bool {
        self.well_done
    }

    pub fn has_solution(&self) -> bool {
        self.has_solution
    }

    pub fn this_solution(&self) -> Option<(f64, f64, f64, f64)> {
        if !self.has_solution {
            return None;
        }
        Some((self.line_x0, self.line_y0, self.line_dx, self.line_dy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_different_points() {
        let bisec = GccAnaPnt2dBisec::new(0.0, 0.0, 4.0, 0.0);
        assert!(bisec.is_done());
        assert!(bisec.has_solution());

        if let Some((x0, y0, dx, dy)) = bisec.this_solution() {
            assert!((x0 - 2.0).abs() < 1e-10);
            assert!((y0 - 0.0).abs() < 1e-10);
            assert!(dx * dx + dy * dy > 0.99);
        } else {
            panic!("Expected solution");
        }
    }

    #[test]
    fn test_same_point() {
        let bisec = GccAnaPnt2dBisec::new(1.0, 1.0, 1.0, 1.0);
        assert!(bisec.is_done());
        assert!(!bisec.has_solution());
        assert!(bisec.this_solution().is_none());
    }

    #[test]
    fn test_vertical_segment() {
        let bisec = GccAnaPnt2dBisec::new(0.0, 0.0, 0.0, 2.0);
        assert!(bisec.is_done());
        assert!(bisec.has_solution());

        if let Some((x0, y0, dx, dy)) = bisec.this_solution() {
            assert!((x0 - 0.0).abs() < 1e-10);
            assert!((y0 - 1.0).abs() < 1e-10);
        }
    }
}
