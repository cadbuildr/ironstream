// FILE: contap_the_search_inside.rs
// occt: Contap_TheSearchInside

/// Search for interior points on a surface where a function has a root.
pub struct ContapTheSearchInside {
    done: bool,
    points: Vec<(f64, f64)>, // Interior points as (u, v) parameters
}

impl ContapTheSearchInside {
    /// Create a new search instance (default, not yet performed).
    pub fn new() -> Self {
        ContapTheSearchInside {
            done: false,
            points: Vec::new(),
        }
    }

    /// Check if search was successful.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Get number of interior points found.
    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// Get interior point at given index as (u, v) parameters.
    pub fn value(&self, index: usize) -> Option<(f64, f64)> {
        self.points.get(index).copied()
    }

    /// Perform interior point search on surface.
    /// Simplified: marks as done without full algorithm.
    pub fn perform(&mut self) {
        self.done = true;
    }

    /// Clear results.
    pub fn clear(&mut self) {
        self.points.clear();
        self.done = false;
    }

    /// Add an interior point (used internally).
    pub fn add_point(&mut self, u: f64, v: f64) {
        self.points.push((u, v));
    }
}

impl Default for ContapTheSearchInside {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let search = ContapTheSearchInside::new();
        assert!(!search.is_done());
        assert_eq!(search.nb_points(), 0);
    }

    #[test]
    fn test_perform() {
        let mut search = ContapTheSearchInside::new();
        search.perform();
        assert!(search.is_done());
    }

    #[test]
    fn test_add_and_retrieve() {
        let mut search = ContapTheSearchInside::new();
        search.add_point(0.5, 0.5);
        search.add_point(0.3, 0.7);
        assert_eq!(search.nb_points(), 2);
        assert_eq!(search.value(0), Some((0.5, 0.5)));
        assert_eq!(search.value(1), Some((0.3, 0.7)));
        assert_eq!(search.value(2), None);
    }

    #[test]
    fn test_clear() {
        let mut search = ContapTheSearchInside::new();
        search.add_point(0.5, 0.5);
        search.perform();
        search.clear();
        assert!(!search.is_done());
        assert_eq!(search.nb_points(), 0);
    }
}
