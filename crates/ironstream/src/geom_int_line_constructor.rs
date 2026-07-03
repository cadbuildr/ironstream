// FILE: geom_int_line_constructor.rs
// occt: GeomInt_LineConstructor

/// Splits intersection lines into segments based on surface boundaries.
pub struct GeomIntLineConstructor {
    done: bool,
    segments: Vec<(f64, f64)>,
}

impl GeomIntLineConstructor {
    /// Create an empty line constructor
    pub fn new() -> Self {
        GeomIntLineConstructor {
            done: false,
            segments: vec![],
        }
    }

    /// Check if splitting was successful
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Get number of segments
    pub fn nb_parts(&self) -> usize {
        self.segments.len()
    }

    /// Get segment by index
    pub fn part(&self, idx: usize) -> Option<(f64, f64)> {
        if idx < self.segments.len() {
            Some(self.segments[idx])
        } else {
            None
        }
    }

    /// Add a segment
    pub fn add_segment(&mut self, first: f64, last: f64) {
        self.segments.push((first, last));
    }

    /// Set done flag
    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }
}

impl Default for GeomIntLineConstructor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let constructor = GeomIntLineConstructor::new();
        assert!(!constructor.is_done());
        assert_eq!(constructor.nb_parts(), 0);
    }

    #[test]
    fn test_add_segments() {
        let mut constructor = GeomIntLineConstructor::new();
        constructor.add_segment(0.0, 0.5);
        constructor.add_segment(0.5, 1.0);

        assert_eq!(constructor.nb_parts(), 2);
        assert_eq!(constructor.part(0), Some((0.0, 0.5)));
        assert_eq!(constructor.part(1), Some((0.5, 1.0)));
        assert_eq!(constructor.part(2), None);
    }

    #[test]
    fn test_done_flag() {
        let mut constructor = GeomIntLineConstructor::new();
        constructor.set_done(true);
        assert!(constructor.is_done());
    }
}
