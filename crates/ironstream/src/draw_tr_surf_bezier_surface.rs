// FILE: draw_tr_surf_bezier_surface.rs
// occt: DrawTrSurf_BezierSurface

//! A drawable Bezier surface for the Draw interface.

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct BezierSurface {
    pub control_points: Vec<Vec<Point>>,
}

impl BezierSurface {
    pub fn new(control_points: Vec<Vec<Point>>) -> Self {
        Self { control_points }
    }

    pub fn u_degree(&self) -> usize {
        if self.control_points.is_empty() {
            0
        } else {
            self.control_points.len() - 1
        }
    }

    pub fn v_degree(&self) -> usize {
        if self.control_points.is_empty() || self.control_points[0].is_empty() {
            0
        } else {
            self.control_points[0].len() - 1
        }
    }
}

impl Default for BezierSurface {
    fn default() -> Self {
        Self {
            control_points: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DrawTrSurfBezierSurface {
    surface: BezierSurface,
    show_poles: bool,
}

impl DrawTrSurfBezierSurface {
    pub fn new(surface: BezierSurface) -> Self {
        Self {
            surface,
            show_poles: true,
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn show_poles(&mut self) {
        self.show_poles = true;
    }

    pub fn clear_poles(&mut self) {
        self.show_poles = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let surface = BezierSurface::default();
        let drawable = DrawTrSurfBezierSurface::new(surface);
        assert!(drawable.show_poles);
    }

    #[test]
    fn test_visibility() {
        let surface = BezierSurface::default();
        let mut drawable = DrawTrSurfBezierSurface::new(surface);

        drawable.clear_poles();
        assert!(!drawable.show_poles);

        drawable.show_poles();
        assert!(drawable.show_poles);
    }
}
