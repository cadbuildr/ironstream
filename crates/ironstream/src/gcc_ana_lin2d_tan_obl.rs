// FILE: gcc_ana_lin2d_tan_obl.rs
// occt: GccAna_Lin2dTanObl

/// 2D line tangent to circle/point and making angle with a line.
pub struct GccAnaLin2dTanObl {
    well_done: bool,
    nbr_sol: i32,
    solutions: Vec<(f64, f64, f64, f64)>, // (x0, y0, dx, dy)
}

impl GccAnaLin2dTanObl {
    /// Create line through point, making angle with line.
    pub fn new_point_line_angle(point_x: f64, point_y: f64, line_x0: f64, line_y0: f64, line_dx: f64, line_dy: f64, angle: f64) -> Self {
        let mut solutions = Vec::new();

        // Normalize line direction
        let len = (line_dx * line_dx + line_dy * line_dy).sqrt();
        let line_dx_n = line_dx / len;
        let line_dy_n = line_dy / len;

        // Two solutions: angle and -angle from the reference line
        for sign in &[1.0, -1.0] {
            let angle_eff = angle * sign;
            let cos_a = angle_eff.cos();
            let sin_a = angle_eff.sin();

            // Rotate line direction by angle
            let new_dx = line_dx_n * cos_a - line_dy_n * sin_a;
            let new_dy = line_dx_n * sin_a + line_dy_n * cos_a;

            solutions.push((point_x, point_y, new_dx, new_dy));
        }

        GccAnaLin2dTanObl {
            well_done: true,
            nbr_sol: 2,
            solutions,
        }
    }

    pub fn is_done(&self) -> bool {
        self.well_done
    }

    pub fn nb_solutions(&self) -> i32 {
        self.nbr_sol
    }

    pub fn this_solution(&self, index: i32) -> Option<(f64, f64, f64, f64)> {
        if index < 1 || index as usize > self.solutions.len() {
            return None;
        }
        Some(self.solutions[(index - 1) as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_line_angle() {
        let alg = GccAnaLin2dTanObl::new_point_line_angle(1.0, 1.0, 0.0, 0.0, 1.0, 0.0, std::f64::consts::PI / 4.0);
        assert!(alg.is_done());
        assert_eq!(alg.nb_solutions(), 2);
        assert!(alg.this_solution(1).is_some());
        assert!(alg.this_solution(3).is_none());
    }

    #[test]
    fn test_zero_angle() {
        let alg = GccAnaLin2dTanObl::new_point_line_angle(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        if let Some((_, _, dx, dy)) = alg.this_solution(1) {
            assert!((dx - 1.0).abs() < 1e-10);
            assert!((dy - 0.0).abs() < 1e-10);
        }
    }
}
