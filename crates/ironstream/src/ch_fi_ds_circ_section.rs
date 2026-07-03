// FILE: ch_fi_ds_circ_section.rs
// occt: ChFiDS_CircSection

/// A section of a fillet, storing either a circle or a line with parameter bounds.
#[derive(Debug, Clone)]
pub struct ChFiDsCircSection {
    /// Circular curve
    circ: Option<CircleData>,
    /// Linear curve
    lin: Option<LineData>,
    /// First parameter
    param_first: f64,
    /// Last parameter
    param_last: f64,
}

#[derive(Debug, Clone, Copy)]
struct CircleData {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    radius: f64,
}

#[derive(Debug, Clone, Copy)]
struct LineData {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}

impl ChFiDsCircSection {
    /// Create a new empty CircSection
    pub fn new() -> Self {
        Self {
            circ: None,
            lin: None,
            param_first: 0.0,
            param_last: 0.0,
        }
    }

    /// Set the section as a circle with parameter bounds
    pub fn set_circle(&mut self, cx: f64, cy: f64, cz: f64, dx: f64, dy: f64, dz: f64, radius: f64, first: f64, last: f64) {
        self.circ = Some(CircleData { x: cx, y: cy, z: cz, dx, dy, dz, radius });
        self.lin = None;
        self.param_first = first;
        self.param_last = last;
    }

    /// Set the section as a line with parameter bounds
    pub fn set_line(&mut self, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64, first: f64, last: f64) {
        self.lin = Some(LineData { x, y, z, dx, dy, dz });
        self.circ = None;
        self.param_first = first;
        self.param_last = last;
    }

    /// Get the first parameter
    pub fn first_parameter(&self) -> f64 {
        self.param_first
    }

    /// Get the last parameter
    pub fn last_parameter(&self) -> f64 {
        self.param_last
    }

    /// Check if section is a circle
    pub fn is_circle(&self) -> bool {
        self.circ.is_some()
    }

    /// Check if section is a line
    pub fn is_line(&self) -> bool {
        self.lin.is_some()
    }
}

impl Default for ChFiDsCircSection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sec = ChFiDsCircSection::new();
        assert!(!sec.is_circle());
        assert!(!sec.is_line());
        assert_eq!(sec.first_parameter(), 0.0);
        assert_eq!(sec.last_parameter(), 0.0);
    }

    #[test]
    fn test_set_circle() {
        let mut sec = ChFiDsCircSection::new();
        sec.set_circle(1.0, 2.0, 3.0, 0.0, 0.0, 1.0, 5.0, 0.0, 6.28);
        assert!(sec.is_circle());
        assert!(!sec.is_line());
        assert_eq!(sec.first_parameter(), 0.0);
        assert_eq!(sec.last_parameter(), 6.28);
    }

    #[test]
    fn test_set_line() {
        let mut sec = ChFiDsCircSection::new();
        sec.set_line(1.0, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 10.0);
        assert!(!sec.is_circle());
        assert!(sec.is_line());
        assert_eq!(sec.first_parameter(), 0.0);
        assert_eq!(sec.last_parameter(), 10.0);
    }

    #[test]
    fn test_switch_circle_to_line() {
        let mut sec = ChFiDsCircSection::new();
        sec.set_circle(1.0, 2.0, 3.0, 0.0, 0.0, 1.0, 5.0, 0.0, 6.28);
        assert!(sec.is_circle());
        sec.set_line(1.0, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 10.0);
        assert!(!sec.is_circle());
        assert!(sec.is_line());
    }

    #[test]
    fn test_default() {
        let sec = ChFiDsCircSection::default();
        assert!(!sec.is_circle());
        assert!(!sec.is_line());
    }
}
