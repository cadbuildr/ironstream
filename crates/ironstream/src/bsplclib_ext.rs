// FILE: bsplclib_ext.rs
// occt-ref: BSplCLib_BuildCache, BSplCLib_KnotAnalyzer, BSplCLib_IntervalGenerator

/// Cache for B-spline curve evaluation.
// occt-ref: BSplCLib_BuildCache
#[derive(Clone, Debug)]
pub struct BSplClibBuildCache {
    pub degree: usize,
    pub cached_params: Vec<f64>,
    pub cached_values: Vec<f64>,
}

impl BSplClibBuildCache {
    pub fn new(degree: usize) -> Self {
        Self {
            degree: degree.max(1),
            cached_params: Vec::new(),
            cached_values: Vec::new(),
        }
    }

    pub fn cache_value(&mut self, param: f64, value: f64) {
        self.cached_params.push(param);
        self.cached_values.push(value);
    }

    pub fn lookup(&self, param: f64) -> Option<f64> {
        self.cached_params
            .iter()
            .position(|&p| (p - param).abs() < 1e-10)
            .and_then(|idx| self.cached_values.get(idx).copied())
    }

    pub fn clear_cache(&mut self) {
        self.cached_params.clear();
        self.cached_values.clear();
    }
}

impl Default for BSplClibBuildCache {
    fn default() -> Self {
        Self::new(3)
    }
}

/// Knot vector analyzer.
// occt-ref: BSplCLib_KnotAnalyzer
#[derive(Clone, Debug)]
pub struct BSplClibKnotAnalyzer {
    pub knots: Vec<f64>,
    pub multiplicities: Vec<usize>,
}

impl BSplClibKnotAnalyzer {
    pub fn new() -> Self {
        Self {
            knots: Vec::new(),
            multiplicities: Vec::new(),
        }
    }

    pub fn add_knot(&mut self, knot: f64, multiplicity: usize) {
        self.knots.push(knot);
        self.multiplicities.push(multiplicity.max(1));
    }

    pub fn is_uniform(&self) -> bool {
        if self.knots.len() < 2 {
            return true;
        }
        let spacing = self.knots[1] - self.knots[0];
        self.knots
            .windows(2)
            .all(|w| (w[1] - w[0] - spacing).abs() < 1e-10)
    }

    pub fn total_multiplicity(&self) -> usize {
        self.multiplicities.iter().sum()
    }
}

impl Default for BSplClibKnotAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate evaluation intervals for a B-spline curve.
// occt-ref: BSplCLib_IntervalGenerator
#[derive(Clone, Debug)]
pub struct BSplClibIntervalGenerator {
    pub start_param: f64,
    pub end_param: f64,
    pub num_intervals: usize,
}

impl BSplClibIntervalGenerator {
    pub fn new(start: f64, end: f64, num_intervals: usize) -> Self {
        Self {
            start_param: start,
            end_param: end,
            num_intervals: num_intervals.max(1),
        }
    }

    pub fn generate_intervals(&self) -> Vec<f64> {
        let mut intervals = Vec::new();
        let step = (self.end_param - self.start_param) / self.num_intervals as f64;
        for i in 0..=self.num_intervals {
            intervals.push(self.start_param + step * i as f64);
        }
        intervals
    }

    pub fn interval_width(&self) -> f64 {
        (self.end_param - self.start_param) / self.num_intervals as f64
    }
}

impl Default for BSplClibIntervalGenerator {
    fn default() -> Self {
        Self::new(0.0, 1.0, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsplclib_build_cache() {
        let mut bc = BSplClibBuildCache::new(3);
        bc.cache_value(0.0, 1.0);
        bc.cache_value(0.5, 2.0);
        assert_eq!(bc.lookup(0.0), Some(1.0));
        assert_eq!(bc.lookup(0.5), Some(2.0));
    }

    #[test]
    fn bsplclib_knot_analyzer() {
        let mut ka = BSplClibKnotAnalyzer::new();
        ka.add_knot(0.0, 2);
        ka.add_knot(1.0, 1);
        ka.add_knot(2.0, 2);
        assert_eq!(ka.total_multiplicity(), 5);
    }

    #[test]
    fn bsplclib_uniform_knots() {
        let mut ka = BSplClibKnotAnalyzer::new();
        ka.add_knot(0.0, 1);
        ka.add_knot(0.5, 1);
        ka.add_knot(1.0, 1);
        assert!(ka.is_uniform());
    }

    #[test]
    fn bsplclib_interval_generator() {
        let ig = BSplClibIntervalGenerator::new(0.0, 1.0, 10);
        let intervals = ig.generate_intervals();
        assert_eq!(intervals.len(), 11);
        assert!(intervals[0] == 0.0);
        assert!(intervals[10] == 1.0);
    }

    #[test]
    fn bsplclib_interval_width() {
        let ig = BSplClibIntervalGenerator::new(0.0, 2.0, 10);
        assert!((ig.interval_width() - 0.2).abs() < 0.01);
    }
}
