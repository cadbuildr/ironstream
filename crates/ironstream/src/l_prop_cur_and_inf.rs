// FILE: l_prop_cur_and_inf.rs
// occt: LProp_CurAndInf

//! Base algorithm for computing local properties of curves.
//! Computes curvature extrema and inflection points.
pub struct LPropCurAndInf {
    is_done: bool,
    nb_points: usize,
    parameters: Vec<f64>,
    point_types: Vec<i32>, // 0 = inflection, 1 = max, 2 = min
}

impl LPropCurAndInf {
    pub fn new() -> Self {
        Self {
            is_done: false,
            nb_points: 0,
            parameters: Vec::new(),
            point_types: Vec::new(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_points(&self) -> usize {
        self.nb_points
    }

    pub fn parameter(&self, index: usize) -> Option<f64> {
        if index < self.parameters.len() {
            Some(self.parameters[index])
        } else {
            None
        }
    }

    pub fn point_type(&self, index: usize) -> Option<i32> {
        if index < self.point_types.len() {
            Some(self.point_types[index])
        } else {
            None
        }
    }

    pub fn add_point(&mut self, param: f64, point_type: i32) {
        self.parameters.push(param);
        self.point_types.push(point_type);
        self.nb_points += 1;
    }

    pub fn set_done(&mut self, done: bool) {
        self.is_done = done;
    }

    pub fn clear(&mut self) {
        self.is_done = false;
        self.nb_points = 0;
        self.parameters.clear();
        self.point_types.clear();
    }

    pub fn compute_from_samples(
        &mut self,
        u_min: f64,
        u_max: f64,
        num_samples: usize,
        curvatures: &[f64],
    ) {
        self.clear();
        if num_samples < 3 || curvatures.len() < num_samples {
            return;
        }

        let step = (u_max - u_min) / (num_samples as f64 - 1.0);

        for i in 1..num_samples - 1 {
            let u = u_min + i as f64 * step;
            let k_prev = curvatures[i - 1];
            let k_curr = curvatures[i];
            let k_next = curvatures[i + 1];

            if (k_prev - k_curr).abs() > 1e-10 && (k_curr - k_next).abs() > 1e-10 {
                if (k_curr > k_prev && k_curr > k_next) || (k_curr < k_prev && k_curr < k_next) {
                    let point_type = if k_curr > k_prev && k_curr > k_next { 1 } else { 2 };
                    self.add_point(u, point_type);
                } else if (k_prev - k_curr).signum() != (k_curr - k_next).signum() {
                    self.add_point(u, 0);
                }
            }
        }

        self.is_done = true;
    }
}

impl Default for LPropCurAndInf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let prop = LPropCurAndInf::new();
        assert!(!prop.is_done());
        assert_eq!(prop.nb_points(), 0);
    }

    #[test]
    fn test_add_point() {
        let mut prop = LPropCurAndInf::new();
        prop.add_point(0.5, 1);
        assert_eq!(prop.nb_points(), 1);
        assert_eq!(prop.parameter(0), Some(0.5));
        assert_eq!(prop.point_type(0), Some(1));
    }

    #[test]
    fn test_multiple_points() {
        let mut prop = LPropCurAndInf::new();
        prop.add_point(0.2, 0);
        prop.add_point(0.5, 1);
        prop.add_point(0.8, 2);
        assert_eq!(prop.nb_points(), 3);
        assert_eq!(prop.parameter(1), Some(0.5));
    }

    #[test]
    fn test_set_done() {
        let mut prop = LPropCurAndInf::new();
        prop.set_done(true);
        assert!(prop.is_done());
    }

    #[test]
    fn test_clear() {
        let mut prop = LPropCurAndInf::new();
        prop.add_point(0.5, 1);
        assert_eq!(prop.nb_points(), 1);
        prop.clear();
        assert_eq!(prop.nb_points(), 0);
        assert!(!prop.is_done());
    }

    #[test]
    fn test_compute_from_samples() {
        let mut prop = LPropCurAndInf::new();
        let curvatures = vec![0.1, 0.5, 0.2, 0.1, 0.3, 0.1];
        prop.compute_from_samples(0.0, 1.0, 6, &curvatures);
        assert!(prop.is_done());
    }

    #[test]
    fn test_empty_samples() {
        let mut prop = LPropCurAndInf::new();
        let curvatures = vec![];
        prop.compute_from_samples(0.0, 1.0, 0, &curvatures);
        assert!(!prop.is_done());
    }
}
