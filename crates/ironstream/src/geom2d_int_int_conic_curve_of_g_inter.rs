// FILE: geom2d_int_int_conic_curve_of_g_inter.rs
// occt: Geom2dInt_IntConicCurveOfGInter

/// Intersection between a conic and a curve.
#[derive(Debug, Clone)]
pub struct Geom2dIntIntConicCurve {
    conic_type: i32,
    curve_type: i32,
    solutions: Vec<(f64, f64)>,
}

impl Geom2dIntIntConicCurve {
    pub fn new(conic_type: i32, curve_type: i32) -> Self {
        Geom2dIntIntConicCurve {
            conic_type,
            curve_type,
            solutions: Vec::new(),
        }
    }

    pub fn add_solution(&mut self, x: f64, y: f64) {
        self.solutions.push((x, y));
    }

    pub fn nb_solutions(&self) -> usize {
        self.solutions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_conic_curve() {
        let mut ic = Geom2dIntIntConicCurve::new(1, 2);
        ic.add_solution(1.0, 2.0);
        assert_eq!(ic.nb_solutions(), 1);
    }
}
