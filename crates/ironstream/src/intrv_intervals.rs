// FILE: intrv_intervals.rs
// occt: Intrv_Intervals

use crate::intrv_interval::IntrvInterval;

/// A sorted sequence of non-overlapping real intervals.
/// Provides set operations: intersect, subtract, unite, exclusive union.
#[derive(Clone, Debug)]
pub struct IntrvIntervals {
    intervals: Vec<IntrvInterval>,
}

impl IntrvIntervals {
    /// Creates a void (empty) sequence of intervals.
    pub fn new() -> Self {
        IntrvIntervals {
            intervals: Vec::new(),
        }
    }

    /// Creates a sequence of one interval.
    pub fn from_interval(int: IntrvInterval) -> Self {
        IntrvIntervals {
            intervals: vec![int],
        }
    }

    /// Intersects the intervals with a single interval.
    pub fn intersect(&mut self, tool: &IntrvInterval) {
        let mut result = Vec::new();
        for interval in &self.intervals {
            if !self.disjoint(interval, tool) {
                let start = interval.start().max(tool.start());
                let end = interval.end().min(tool.end());
                let tol_start = interval.tol_start().max(tool.tol_start());
                let tol_end = interval.tol_end().max(tool.tol_end());
                result.push(IntrvInterval::with_tolerances(start, tol_start, end, tol_end));
            }
        }
        self.intervals = result;
    }

    /// Intersects the intervals with another sequence.
    pub fn intersect_intervals(&mut self, tool: &IntrvIntervals) {
        let mut result = Vec::new();
        for tool_int in &tool.intervals {
            for interval in &self.intervals {
                if !self.disjoint(interval, tool_int) {
                    let start = interval.start().max(tool_int.start());
                    let end = interval.end().min(tool_int.end());
                    let tol_start = interval.tol_start().max(tool_int.tol_start());
                    let tol_end = interval.tol_end().max(tool_int.tol_end());
                    result.push(IntrvInterval::with_tolerances(start, tol_start, end, tol_end));
                }
            }
        }
        self.intervals = result;
    }

    /// Subtracts an interval from this sequence.
    pub fn subtract(&mut self, tool: &IntrvInterval) {
        let mut result = Vec::new();
        for interval in &self.intervals {
            if self.disjoint(interval, tool) {
                result.push(interval.clone());
            } else {
                if interval.start() < tool.start() {
                    result.push(IntrvInterval::with_tolerances(
                        interval.start(),
                        interval.tol_start(),
                        tool.start(),
                        tool.tol_start(),
                    ));
                }
                if interval.end() > tool.end() {
                    result.push(IntrvInterval::with_tolerances(
                        tool.end(),
                        tool.tol_end(),
                        interval.end(),
                        interval.tol_end(),
                    ));
                }
            }
        }
        self.intervals = result;
    }

    /// Subtracts a sequence of intervals.
    pub fn subtract_intervals(&mut self, tool: &IntrvIntervals) {
        for tool_int in &tool.intervals {
            self.subtract(tool_int);
        }
    }

    /// Unites an interval with this sequence.
    pub fn unite(&mut self, tool: &IntrvInterval) {
        let mut result = Vec::new();
        let mut merged = false;
        for interval in &self.intervals {
            if self.adjacent_or_overlap(interval, tool) && !merged {
                let start = interval.start().min(tool.start());
                let end = interval.end().max(tool.end());
                let tol_start = interval.tol_start().min(tool.tol_start());
                let tol_end = interval.tol_end().min(tool.tol_end());
                result.push(IntrvInterval::with_tolerances(start, tol_start, end, tol_end));
                merged = true;
            } else {
                result.push(interval.clone());
            }
        }
        if !merged {
            result.push(tool.clone());
        }
        self.intervals = self.sort_intervals(result);
    }

    /// Unites a sequence of intervals.
    pub fn unite_intervals(&mut self, tool: &IntrvIntervals) {
        for tool_int in &tool.intervals {
            self.unite(tool_int);
        }
    }

    /// Exclusive union: elements in either sequence but not both.
    pub fn x_unite(&mut self, tool: &IntrvInterval) {
        let mut subtracted = self.clone();
        subtracted.subtract(tool);
        let mut tool_only = IntrvIntervals::from_interval(tool.clone());
        tool_only.subtract_intervals(self);
        self.intervals = subtracted.intervals;
        self.unite_intervals(&tool_only);
    }

    /// Exclusive union with another sequence.
    pub fn x_unite_intervals(&mut self, tool: &IntrvIntervals) {
        for tool_int in &tool.intervals {
            self.x_unite(tool_int);
        }
    }

    pub fn nb_intervals(&self) -> usize {
        self.intervals.len()
    }

    pub fn value(&self, index: usize) -> Option<&IntrvInterval> {
        self.intervals.get(index)
    }

    fn disjoint(&self, a: &IntrvInterval, b: &IntrvInterval) -> bool {
        a.end() - (a.tol_end() as f64) < b.start() + (b.tol_start() as f64)
            || b.end() - (b.tol_end() as f64) < a.start() + (a.tol_start() as f64)
    }

    fn adjacent_or_overlap(&self, a: &IntrvInterval, b: &IntrvInterval) -> bool {
        !self.disjoint(a, b)
            || (a.end() - (a.tol_end() as f64) - (b.start() + (b.tol_start() as f64))).abs() < 1e-10
            || (b.end() - (b.tol_end() as f64) - (a.start() + (a.tol_start() as f64))).abs() < 1e-10
    }

    fn sort_intervals(&self, mut intervals: Vec<IntrvInterval>) -> Vec<IntrvInterval> {
        intervals.sort_by(|a, b| a.start().partial_cmp(&b.start()).unwrap_or(std::cmp::Ordering::Equal));
        intervals
    }
}

impl Default for IntrvIntervals {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_intervals() {
        let i = IntrvIntervals::new();
        assert_eq!(i.nb_intervals(), 0);
    }

    #[test]
    fn test_from_interval() {
        let int = IntrvInterval::from_bounds(1.0, 3.0);
        let i = IntrvIntervals::from_interval(int);
        assert_eq!(i.nb_intervals(), 1);
    }

    #[test]
    fn test_unite() {
        let mut i = IntrvIntervals::from_interval(IntrvInterval::from_bounds(1.0, 3.0));
        i.unite(&IntrvInterval::from_bounds(2.0, 5.0));
        assert_eq!(i.nb_intervals(), 1);
        let result = i.value(0).unwrap();
        assert_eq!(result.start(), 1.0);
        assert_eq!(result.end(), 5.0);
    }

    #[test]
    fn test_intersect() {
        let mut i = IntrvIntervals::from_interval(IntrvInterval::from_bounds(1.0, 5.0));
        i.intersect(&IntrvInterval::from_bounds(2.0, 4.0));
        assert_eq!(i.nb_intervals(), 1);
        let result = i.value(0).unwrap();
        assert_eq!(result.start(), 2.0);
        assert_eq!(result.end(), 4.0);
    }

    #[test]
    fn test_subtract() {
        let mut i = IntrvIntervals::from_interval(IntrvInterval::from_bounds(1.0, 5.0));
        i.subtract(&IntrvInterval::from_bounds(2.0, 3.0));
        assert_eq!(i.nb_intervals(), 2);
    }
}
