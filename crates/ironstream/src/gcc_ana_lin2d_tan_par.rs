// FILE: gcc_ana_lin2d_tan_par.rs
// occt: GccAna_Lin2dTanPar

/// 2D line parallel to a line and tangent to circle or through point.
pub struct GccAnaLin2dTanPar {
    well_done: bool,
    nbr_sol: i32,
    solutions: Vec<(f64, f64, f64, f64)>, // (x0, y0, dx, dy)
}

impl GccAnaLin2dTanPar {
    /// Line through point parallel to another line.
    pub fn new_point_parallel(point_x: f64, point_y: f64, ref_line_dx: f64, ref_line_dy: f64) -> Self {
        let len = (ref_line_dx * ref_line_dx + ref_line_dy * ref_line_dy).sqrt();
        let dx = ref_line_dx / len;
        let dy = ref_line_dy / len;

        GccAnaLin2dTanPar {
            well_done: true,
            nbr_sol: 1,
            solutions: vec![(point_x, point_y, dx, dy)],
        }
    }

    /// Line tangent to circle and parallel to another line.
    pub fn new_circle_parallel(
        circle_x: f64,
        circle_y: f64,
        circle_r: f64,
        ref_line_dx: f64,
        ref_line_dy: f64,
    ) -> Self {
        let len = (ref_line_dx * ref_line_dx + ref_line_dy * ref_line_dy).sqrt();
        let dx = ref_line_dx / len;
        let dy = ref_line_dy / len;

        // Perpendicular direction
        let perp_x = -dy;
        let perp_y = dx;

        let mut solutions = Vec::new();
        // Two tangent lines at ±r perpendicular distance
        solutions.push((circle_x + circle_r * perp_x, circle_y + circle_r * perp_y, dx, dy));
        solutions.push((circle_x - circle_r * perp_x, circle_y - circle_r * perp_y, dx, dy));

        GccAnaLin2dTanPar {
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
    fn test_point_parallel() {
        let alg = GccAnaLin2dTanPar::new_point_parallel(2.0, 3.0, 1.0, 0.0);
        assert!(alg.is_done());
        assert_eq!(alg.nb_solutions(), 1);
        if let Some((x, y, dx, dy)) = alg.this_solution(1) {
            assert!((x - 2.0).abs() < 1e-10);
            assert!((y - 3.0).abs() < 1e-10);
            assert!((dx - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_circle_parallel() {
        let alg = GccAnaLin2dTanPar::new_circle_parallel(0.0, 0.0, 1.0, 1.0, 0.0);
        assert!(alg.is_done());
        assert_eq!(alg.nb_solutions(), 2);
        assert!(alg.this_solution(1).is_some());
        assert!(alg.this_solution(2).is_some());
    }
}
