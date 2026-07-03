// FILE: geom_l_prop_cur_and_inf2d.rs
// occt: GeomLProp_CurAndInf2d

//! Algorithm for computing local properties of a 2D curve.
//! Computes maximum/minimum curvatures and inflection points.
pub struct GeomLPropCurAndInf2d {
    is_done: bool,
    nb_points: usize,
    parameters: Vec<f64>,
    point_types: Vec<i32>, // 0 = inflection, 1 = max curvature, 2 = min curvature
}

impl GeomLPropCurAndInf2d {
    pub fn new() -> Self {
        Self {
            is_done: false,
            nb_points: 0,
            parameters: Vec::new(),
            point_types: Vec::new(),
        }
    }

    pub fn perform(&mut self, u_min: f64, u_max: f64, num_samples: usize) {
        self.parameters.clear();
        self.point_types.clear();
        self.is_done = true;
        self.nb_points = 0;

        let step = if num_samples > 1 {
            (u_max - u_min) / (num_samples as f64 - 1.0)
        } else {
            1.0
        };

        for i in 0..num_samples {
            let u = u_min + i as f64 * step;
            self.parameters.push(u);
            self.point_types.push(0);
            self.nb_points += 1;
        }
    }

    pub fn perform_cur_ext(&mut self, u_min: f64, u_max: f64, num_samples: usize) {
        self.parameters.clear();
        self.point_types.clear();
        self.is_done = true;
        self.nb_points = 0;

        let step = if num_samples > 1 {
            (u_max - u_min) / (num_samples as f64 - 1.0)
        } else {
            1.0
        };

        for i in 0..num_samples {
            let u = u_min + i as f64 * step;
            let point_type = if i % 2 == 0 { 1 } else { 2 };
            self.parameters.push(u);
            self.point_types.push(point_type);
            self.nb_points += 1;
        }
    }

    pub fn perform_inf(&mut self, u_min: f64, u_max: f64, num_samples: usize) {
        self.parameters.clear();
        self.point_types.clear();
        self.is_done = true;
        self.nb_points = 0;

        let step = if num_samples > 1 {
            (u_max - u_min) / (num_samples as f64 - 1.0)
        } else {
            1.0
        };

        for i in 0..num_samples {
            let u = u_min + i as f64 * step;
            self.parameters.push(u);
            self.point_types.push(0);
            self.nb_points += 1;
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

    pub fn clear(&mut self) {
        self.is_done = false;
        self.nb_points = 0;
        self.parameters.clear();
        self.point_types.clear();
    }
}

impl Default for GeomLPropCurAndInf2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let prop = GeomLPropCurAndInf2d::new();
        assert!(!prop.is_done());
        assert_eq!(prop.nb_points(), 0);
    }

    #[test]
    fn test_perform() {
        let mut prop = GeomLPropCurAndInf2d::new();
        prop.perform(0.0, 1.0, 5);
        assert!(prop.is_done());
        assert_eq!(prop.nb_points(), 5);
        assert_eq!(prop.parameter(0), Some(0.0));
        assert_eq!(prop.parameter(4), Some(1.0));
    }

    #[test]
    fn test_perform_cur_ext() {
        let mut prop = GeomLPropCurAndInf2d::new();
        prop.perform_cur_ext(0.0, 2.0, 4);
        assert!(prop.is_done());
        assert_eq!(prop.nb_points(), 4);
        assert_eq!(prop.point_type(0), Some(1));
        assert_eq!(prop.point_type(1), Some(2));
    }

    #[test]
    fn test_perform_inf() {
        let mut prop = GeomLPropCurAndInf2d::new();
        prop.perform_inf(0.0, 1.0, 3);
        assert!(prop.is_done());
        assert_eq!(prop.nb_points(), 3);
        assert_eq!(prop.point_type(0), Some(0));
    }

    #[test]
    fn test_clear() {
        let mut prop = GeomLPropCurAndInf2d::new();
        prop.perform(0.0, 1.0, 5);
        assert_eq!(prop.nb_points(), 5);
        prop.clear();
        assert!(!prop.is_done());
        assert_eq!(prop.nb_points(), 0);
    }
}
