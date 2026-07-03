// FILE: extrema_pc_grid_evaluator.rs
// occt: ExtremaPC_GridEvaluator

/// Grid-based evaluator for point-curve extrema.
pub struct ExtremaPcGridEvaluator {
    grid_size: i32,
    min_param: f64,
    max_param: f64,
    values: Vec<f64>,
}

impl ExtremaPcGridEvaluator {
    /// Creates a new grid evaluator.
    pub fn new(grid_size: i32) -> Self {
        ExtremaPcGridEvaluator {
            grid_size,
            min_param: 0.0,
            max_param: 1.0,
            values: vec![0.0; grid_size as usize],
        }
    }

    /// Sets the parameter range.
    pub fn set_range(&mut self, min_param: f64, max_param: f64) {
        self.min_param = min_param;
        self.max_param = max_param;
    }

    /// Returns the grid size.
    pub fn grid_size(&self) -> i32 {
        self.grid_size
    }

    /// Returns parameter at grid point i.
    pub fn param_at(&self, i: i32) -> f64 {
        if i < 0 || i >= self.grid_size {
            self.min_param
        } else {
            let frac = i as f64 / (self.grid_size - 1) as f64;
            self.min_param + frac * (self.max_param - self.min_param)
        }
    }

    /// Sets value at grid point i.
    pub fn set_value(&mut self, i: i32, value: f64) {
        if i >= 0 && (i as usize) < self.values.len() {
            self.values[i as usize] = value;
        }
    }

    /// Gets value at grid point i.
    pub fn value(&self, i: i32) -> Option<f64> {
        if i >= 0 && (i as usize) < self.values.len() {
            Some(self.values[i as usize])
        } else {
            None
        }
    }

    /// Finds minimum value in grid.
    pub fn min_value(&self) -> f64 {
        self.values.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Finds index of minimum value.
    pub fn min_index(&self) -> Option<i32> {
        self.values
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i as i32)
    }
}

impl Default for ExtremaPcGridEvaluator {
    fn default() -> Self {
        Self::new(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let eval = ExtremaPcGridEvaluator::new(5);
        assert_eq!(eval.grid_size(), 5);
    }

    #[test]
    fn test_param_at() {
        let eval = ExtremaPcGridEvaluator::new(3);
        assert_eq!(eval.param_at(0), 0.0);
        assert_eq!(eval.param_at(2), 1.0);
        assert_eq!(eval.param_at(1), 0.5);
    }

    #[test]
    fn test_set_and_get_value() {
        let mut eval = ExtremaPcGridEvaluator::new(3);
        eval.set_value(0, 1.0);
        eval.set_value(1, 2.5);
        eval.set_value(2, 0.5);
        assert_eq!(eval.value(0), Some(1.0));
        assert_eq!(eval.value(1), Some(2.5));
        assert_eq!(eval.value(2), Some(0.5));
    }

    #[test]
    fn test_min_value() {
        let mut eval = ExtremaPcGridEvaluator::new(4);
        eval.set_value(0, 5.0);
        eval.set_value(1, 1.5);
        eval.set_value(2, 3.0);
        eval.set_value(3, 2.0);
        assert_eq!(eval.min_value(), 1.5);
    }

    #[test]
    fn test_min_index() {
        let mut eval = ExtremaPcGridEvaluator::new(4);
        eval.set_value(0, 5.0);
        eval.set_value(1, 1.5);
        eval.set_value(2, 3.0);
        eval.set_value(3, 2.5);
        assert_eq!(eval.min_index(), Some(1));
    }
}
