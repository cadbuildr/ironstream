// FILE: int_polyh_section_line.rs
// occt: IntPolyh_SectionLine

//! Section line between two polyhedra surfaces.

/// Section line segment
#[derive(Clone)]
pub struct SectionPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u1: f64,
    pub v1: f64,
    pub u2: f64,
    pub v2: f64,
}

/// Section line as sequence of points
pub struct IntPolyhSectionLine {
    points: Vec<SectionPoint>,
}

impl IntPolyhSectionLine {
    /// Creates empty section line
    pub fn new() -> Self {
        IntPolyhSectionLine {
            points: Vec::new(),
        }
    }

    /// Adds point to section line
    pub fn add_point(&mut self, point: SectionPoint) {
        self.points.push(point);
    }

    /// Returns number of points
    pub fn nb_points(&self) -> i32 {
        self.points.len() as i32
    }

    /// Returns point at index
    pub fn point(&self, index: i32) -> Option<SectionPoint> {
        self.points.get(index as usize).cloned()
    }

    /// Returns section line length
    pub fn length(&self) -> f64 {
        let mut len = 0.0;
        for i in 0..self.points.len().saturating_sub(1) {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            let dz = p2.z - p1.z;
            len += (dx * dx + dy * dy + dz * dz).sqrt();
        }
        len
    }

    /// Clears all points
    pub fn clear(&mut self) {
        self.points.clear();
    }
}

impl Default for IntPolyhSectionLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_line_new() {
        let line = IntPolyhSectionLine::new();
        assert_eq!(line.nb_points(), 0);
    }

    #[test]
    fn test_section_line_add_point() {
        let mut line = IntPolyhSectionLine::new();
        let point = SectionPoint {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            u1: 0.5,
            v1: 0.5,
            u2: 0.5,
            v2: 0.5,
        };
        line.add_point(point);
        assert_eq!(line.nb_points(), 1);
    }

    #[test]
    fn test_section_line_length() {
        let mut line = IntPolyhSectionLine::new();
        line.add_point(SectionPoint {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            u1: 0.0,
            v1: 0.0,
            u2: 0.0,
            v2: 0.0,
        });
        line.add_point(SectionPoint {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            u1: 1.0,
            v1: 0.0,
            u2: 0.0,
            v2: 0.0,
        });
        assert!((line.length() - 1.0).abs() < 1e-10);
    }
}
