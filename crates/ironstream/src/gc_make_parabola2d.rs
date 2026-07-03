// FILE: gc_make_parabola2d.rs
// occt: GC_MakeParabola2d

//! Construction algorithms for 2D parabolas.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MakeStatus {
    Ok,
    NegativeFocus,
    InvalidAxis,
}

/// A 2D parabola defined by focus and directrix.
#[derive(Clone, Debug)]
pub struct Parabola2d {
    /// Focus point (x, y)
    pub focus: (f64, f64),
    /// Directrix direction (normalized)
    pub directrix_dir: (f64, f64),
    /// Distance from vertex to focus
    pub focal_param: f64,
}

impl Parabola2d {
    /// Create a parabola from focus and focal parameter.
    pub fn new(focus: (f64, f64), directrix_dir: (f64, f64), focal_param: f64) -> Option<Self> {
        if focal_param < 0.0 {
            return None;
        }

        let (dx, dy) = directrix_dir;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1e-15 {
            return None;
        }

        Some(Parabola2d {
            focus,
            directrix_dir: (dx / len, dy / len),
            focal_param,
        })
    }

    /// Get vertex point (midway between focus and directrix).
    pub fn vertex(&self) -> (f64, f64) {
        let offset = self.focal_param / 2.0;
        (
            self.focus.0 - offset * self.directrix_dir.0,
            self.focus.1 - offset * self.directrix_dir.1,
        )
    }

    /// Evaluate parabola at parameter t.
    /// Using standard form: y^2 = 4*p*x
    pub fn eval(&self, t: f64) -> (f64, f64) {
        let x = t * t / (4.0 * self.focal_param);
        let y = t;

        // Rotate to match directrix
        let (dx, dy) = self.directrix_dir;
        let px = x * dx - y * dy;
        let py = x * dy + y * dx;

        // Translate by vertex
        let v = self.vertex();
        (v.0 + px, v.1 + py)
    }
}

/// Builder for 2D parabolas.
pub struct MakeParabola2d {
    parabola: Option<Parabola2d>,
    status: MakeStatus,
}

impl MakeParabola2d {
    /// Create from focus and focal parameter.
    pub fn new(focus: (f64, f64), directrix_dir: (f64, f64), focal_param: f64) -> Self {
        match Parabola2d::new(focus, directrix_dir, focal_param) {
            Some(parabola) => MakeParabola2d {
                parabola: Some(parabola),
                status: MakeStatus::Ok,
            },
            None => MakeParabola2d {
                parabola: None,
                status: MakeStatus::NegativeFocus,
            },
        }
    }

    /// Get the constructed parabola.
    pub fn value(&self) -> Option<&Parabola2d> {
        self.parabola.as_ref()
    }

    /// Get status.
    pub fn status(&self) -> MakeStatus {
        self.status
    }

    /// Check if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.status == MakeStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parabola_create() {
        let para = Parabola2d::new((1.0, 0.0), (1.0, 0.0), 1.0);
        assert!(para.is_some());
    }

    #[test]
    fn test_parabola_negative_focal() {
        let para = Parabola2d::new((1.0, 0.0), (1.0, 0.0), -1.0);
        assert!(para.is_none());
    }

    #[test]
    fn test_make_parabola() {
        let maker = MakeParabola2d::new((1.0, 0.0), (1.0, 0.0), 1.0);
        assert!(maker.is_done());
        assert!(maker.value().is_some());
    }

    #[test]
    fn test_parabola_vertex() {
        let para = Parabola2d::new((1.0, 0.0), (1.0, 0.0), 2.0).unwrap();
        let v = para.vertex();
        assert!((v.0 - 0.0).abs() < 1e-9);
    }
}
