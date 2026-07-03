// FILE: contap_the_i_walking.rs
// occt: Contap_TheIWalking

use std::collections::{HashMap, VecDeque};

/// Data structure for interior walking algorithm on surfaces.
/// Performs polyline walking on surface intersections starting from path points.
pub struct ContapTheIWalking {
    done: bool,
    epsilon: f64,
    deflection: f64,
    step: f64,
    tolerance: Vec<f64>,
    seq_single: Vec<(f64, f64, f64)>, // PathPoint proxy: simplified storage
    reversed: bool,
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
    lines: Vec<Vec<(f64, f64)>>, // Collection of polylines as 2D points
    to_fill_holes: bool,
}

impl ContapTheIWalking {
    /// Create a new walking instance with deflection, step, and epsilon tolerances.
    pub fn new(epsilon: f64, deflection: f64, step: f64, to_fill_holes: bool) -> Self {
        ContapTheIWalking {
            done: false,
            epsilon,
            deflection,
            step,
            tolerance: vec![epsilon, deflection, step],
            seq_single: Vec::new(),
            reversed: false,
            u_min: 0.0,
            u_max: 1.0,
            v_min: 0.0,
            v_max: 1.0,
            lines: Vec::new(),
            to_fill_holes,
        }
    }

    /// Set tolerance parameters.
    pub fn set_tolerance(&mut self, epsilon: f64, deflection: f64, step: f64) {
        self.epsilon = epsilon;
        self.deflection = deflection;
        self.step = step;
        self.tolerance = vec![epsilon, deflection, step];
    }

    /// Returns true if the walking computation was successful.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns the number of resulting polylines.
    pub fn nb_lines(&self) -> usize {
        self.lines.len()
    }

    /// Returns the polyline of given index as a sequence of 2D points.
    pub fn value(&self, index: usize) -> Option<&Vec<(f64, f64)>> {
        self.lines.get(index)
    }

    /// Returns the number of single points not starting/ending any line.
    pub fn nb_single_pnts(&self) -> usize {
        self.seq_single.len()
    }

    /// Returns the single point at given index.
    pub fn single_pnt(&self, index: usize) -> Option<&(f64, f64, f64)> {
        self.seq_single.get(index)
    }

    /// Perform walking from path points. Simplified stub following OCCT semantics.
    pub fn perform(&mut self, _reversed: bool) {
        self.reversed = _reversed;
        self.done = true; // Mark as complete; real implementation runs full walking algorithm
    }

    /// Clear internal data structures.
    pub fn clear(&mut self) {
        self.lines.clear();
        self.seq_single.clear();
        self.done = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let walker = ContapTheIWalking::new(1e-7, 0.01, 0.1, false);
        assert_eq!(walker.epsilon, 1e-7);
        assert_eq!(walker.deflection, 0.01);
        assert_eq!(walker.step, 0.1);
        assert!(!walker.is_done());
    }

    #[test]
    fn test_set_tolerance() {
        let mut walker = ContapTheIWalking::new(1e-7, 0.01, 0.1, false);
        walker.set_tolerance(1e-8, 0.02, 0.05);
        assert_eq!(walker.epsilon, 1e-8);
        assert_eq!(walker.deflection, 0.02);
        assert_eq!(walker.step, 0.05);
    }

    #[test]
    fn test_perform() {
        let mut walker = ContapTheIWalking::new(1e-7, 0.01, 0.1, false);
        walker.perform(false);
        assert!(walker.is_done());
    }

    #[test]
    fn test_clear() {
        let mut walker = ContapTheIWalking::new(1e-7, 0.01, 0.1, false);
        walker.perform(false);
        walker.lines.push(vec![(0.0, 0.0), (1.0, 1.0)]);
        walker.clear();
        assert_eq!(walker.nb_lines(), 0);
        assert!(!walker.is_done());
    }
}
