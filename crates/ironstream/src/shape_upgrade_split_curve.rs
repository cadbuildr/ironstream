// FILE: shape_upgrade_split_curve.rs
// occt: ShapeUpgrade_SplitCurve

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok = 0,
    Done1 = 1,  // splitting required, more than one segment
    Done2 = 2,  // splitting required, only one segment
    Done3 = 3,  // form or parametrisation modified
}

/// Splits a curve with a criterion
pub struct ShapeUpgradeSplitCurve {
    first: f64,
    last: f64,
    split_values: Vec<f64>,
    nb_curves: i32,
    status: Status,
}

impl ShapeUpgradeSplitCurve {
    /// Empty constructor
    pub fn new() -> Self {
        ShapeUpgradeSplitCurve {
            first: 0.0,
            last: 0.0,
            split_values: Vec::new(),
            nb_curves: 0,
            status: Status::Ok,
        }
    }

    /// Initializes with curve first and last parameters
    pub fn init(&mut self, first: f64, last: f64) {
        self.first = first;
        self.last = last;
        self.split_values.clear();
        self.split_values.push(first);
        self.split_values.push(last);
    }

    /// Sets the parameters where splitting has to be done
    pub fn set_split_values(&mut self, values: &[f64]) {
        self.split_values.clear();
        for &v in values {
            self.split_values.push(v);
        }
        self.split_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    }

    /// Builds result - if Segment true, segments; if false, trimmed curves
    pub fn build(&mut self, _segment: bool) {
        // Placeholder for curve building
        if self.split_values.len() > 2 {
            self.nb_curves = (self.split_values.len() - 1) as i32;
            self.status = Status::Done1;
        } else {
            self.status = Status::Ok;
        }
    }

    /// Returns all splitting values
    pub fn split_values(&self) -> &[f64] {
        &self.split_values
    }

    /// Calculates points for correction/splitting of curve
    pub fn compute(&mut self) {
        // Placeholder for computation
    }

    /// Performs correction/splitting of the curve
    pub fn perform(&mut self, segment: bool) {
        self.compute();
        self.build(segment);
    }

    /// Returns the status
    pub fn status(&self) -> Status {
        self.status
    }

    /// Check status value
    pub fn has_status(&self, s: Status) -> bool {
        self.status == s
    }
}

impl Default for ShapeUpgradeSplitCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{ShapeUpgradeSplitCurve, Status};

    #[test]
    fn test_new() {
        let curve = ShapeUpgradeSplitCurve::new();
        assert_eq!(curve.status(), Status::Ok);
    }

    #[test]
    fn test_init() {
        let mut curve = ShapeUpgradeSplitCurve::new();
        curve.init(0.0, 1.0);
        assert_eq!(curve.first, 0.0);
        assert_eq!(curve.last, 1.0);
    }

    #[test]
    fn test_set_split_values() {
        let mut curve = ShapeUpgradeSplitCurve::new();
        curve.init(0.0, 1.0);
        curve.set_split_values(&[0.0, 0.25, 0.5, 0.75, 1.0]);
        assert_eq!(curve.split_values().len(), 5);
    }

    #[test]
    fn test_build() {
        let mut curve = ShapeUpgradeSplitCurve::new();
        curve.init(0.0, 1.0);
        curve.set_split_values(&[0.0, 0.5, 1.0]);
        curve.build(true);
        assert_eq!(curve.status(), Status::Done1);
    }

    #[test]
    fn test_perform() {
        let mut curve = ShapeUpgradeSplitCurve::new();
        curve.init(0.0, 1.0);
        curve.set_split_values(&[0.0, 0.33, 0.66, 1.0]);
        curve.perform(true);
        assert_ne!(curve.status(), Status::Ok);
    }
}
