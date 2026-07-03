// FILE: gcc_ana_circ_pnt2d_bisec.rs
// occt: GccAna_CircPnt2dBisec

use std::f64;

/// Bisecting curves between a 2D circle and a point.
/// A bisecting curve is such that each point is at equal distance
/// from the circle and the point. Can be ellipse, hyperbola, circle or line.
pub struct GccAnaCircPnt2dBisec {
    well_done: bool,
    nbr_sol: i32,
    circle_x: f64,
    circle_y: f64,
    circle_radius: f64,
    point_x: f64,
    point_y: f64,
    position: i32, // -1: point inside, 0: point on circle, 1: point outside
    tolerance: f64,
}

#[derive(Clone, Debug)]
pub enum BisecSolution {
    Line { x0: f64, y0: f64, dx: f64, dy: f64 },
    Circle { cx: f64, cy: f64, r: f64 },
    Ellipse { cx: f64, cy: f64, a: f64, b: f64, angle: f64 },
    Hyperbola { cx: f64, cy: f64, a: f64, b: f64, angle: f64 },
}

impl GccAnaCircPnt2dBisec {
    /// Construct bisecting curves between circle and point.
    pub fn new(circle_x: f64, circle_y: f64, circle_radius: f64, point_x: f64, point_y: f64) -> Self {
        let mut result = GccAnaCircPnt2dBisec {
            well_done: false,
            nbr_sol: 0,
            circle_x,
            circle_y,
            circle_radius,
            point_x,
            point_y,
            position: 0,
            tolerance: 1e-10,
        };
        result.define_solutions();
        result
    }

    /// Construct with custom tolerance.
    pub fn new_with_tolerance(
        circle_x: f64,
        circle_y: f64,
        circle_radius: f64,
        point_x: f64,
        point_y: f64,
        tolerance: f64,
    ) -> Self {
        let mut result = GccAnaCircPnt2dBisec {
            well_done: false,
            nbr_sol: 0,
            circle_x,
            circle_y,
            circle_radius,
            point_x,
            point_y,
            position: 0,
            tolerance: 1e-10_f64.max(tolerance),
        };
        result.define_solutions();
        result
    }

    fn define_solutions(&mut self) {
        let dx = self.point_x - self.circle_x;
        let dy = self.point_y - self.circle_y;
        let dist = (dx * dx + dy * dy).sqrt();
        let diff = self.circle_radius - dist;

        if diff.abs() < self.tolerance {
            self.position = 0;
            self.nbr_sol = 1;
        } else if diff > 0.0 {
            self.position = -1;
            self.nbr_sol = 1;
        } else {
            self.position = 1;
            self.nbr_sol = 2;
        }
        self.well_done = true;
    }

    /// Returns true if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.well_done
    }

    /// Returns number of solutions.
    pub fn nb_solutions(&self) -> i32 {
        self.nbr_sol
    }

    /// Returns the solution at the given index (1-based).
    pub fn this_solution(&self, index: i32) -> Option<BisecSolution> {
        if !self.well_done || index < 1 || index > self.nbr_sol {
            return None;
        }

        let dx = self.point_x - self.circle_x;
        let dy = self.point_y - self.circle_y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < self.tolerance {
            // Point coincides with circle center
            let radius = self.circle_radius / 2.0;
            return Some(BisecSolution::Circle {
                cx: self.point_x,
                cy: self.point_y,
                r: radius,
            });
        }

        let center_x = (self.point_x + self.circle_x) / 2.0;
        let center_y = (self.point_y + self.circle_y) / 2.0;

        if self.position == -1 {
            // Point inside circle: ellipse solution
            let a = self.circle_radius / 2.0;
            let b = (self.circle_radius * self.circle_radius - dist * dist).sqrt() / 2.0;
            let angle = if dx != 0.0 || dy != 0.0 {
                dy.atan2(dx)
            } else {
                0.0
            };
            Some(BisecSolution::Ellipse {
                cx: center_x,
                cy: center_y,
                a,
                b,
                angle,
            })
        } else if self.position == 0 {
            // Point on circle: line solution
            let dx_norm = if dx.abs() > 1e-15 || dy.abs() > 1e-15 {
                dx
            } else {
                1.0
            };
            let dy_norm = dy;
            let len = (dx_norm * dx_norm + dy_norm * dy_norm).sqrt();
            Some(BisecSolution::Line {
                x0: self.point_x,
                y0: self.point_y,
                dx: dx_norm / len,
                dy: dy_norm / len,
            })
        } else {
            // Point outside circle: hyperbola solutions
            let d1 = ((dist * dist - self.circle_radius * self.circle_radius).sqrt()) / 2.0;
            let d2 = self.circle_radius / 2.0;
            let angle = dy.atan2(dx);
            Some(BisecSolution::Hyperbola {
                cx: center_x,
                cy: center_y,
                a: d2,
                b: d1,
                angle,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_inside_circle() {
        let bisec = GccAnaCircPnt2dBisec::new(0.0, 0.0, 2.0, 0.5, 0.0);
        assert!(bisec.is_done());
        assert_eq!(bisec.nb_solutions(), 1);

        let sol = bisec.this_solution(1);
        assert!(matches!(sol, Some(BisecSolution::Ellipse { .. })));
    }

    #[test]
    fn test_point_outside_circle() {
        let bisec = GccAnaCircPnt2dBisec::new(0.0, 0.0, 1.0, 5.0, 0.0);
        assert!(bisec.is_done());
        assert_eq!(bisec.nb_solutions(), 2);

        let sol1 = bisec.this_solution(1);
        let sol2 = bisec.this_solution(2);
        assert!(matches!(sol1, Some(BisecSolution::Hyperbola { .. })));
        assert!(matches!(sol2, Some(BisecSolution::Hyperbola { .. })));
    }

    #[test]
    fn test_point_on_circle() {
        let bisec = GccAnaCircPnt2dBisec::new(0.0, 0.0, 2.0, 2.0, 0.0);
        assert!(bisec.is_done());
        assert_eq!(bisec.nb_solutions(), 1);

        let sol = bisec.this_solution(1);
        assert!(matches!(sol, Some(BisecSolution::Line { .. })));
    }

    #[test]
    fn test_point_at_center() {
        let bisec = GccAnaCircPnt2dBisec::new(1.0, 1.0, 3.0, 1.0, 1.0);
        assert!(bisec.is_done());
        assert_eq!(bisec.nb_solutions(), 1);

        if let Some(BisecSolution::Circle { cx, cy, r }) = bisec.this_solution(1) {
            assert!((cx - 1.0).abs() < 1e-10);
            assert!((cy - 1.0).abs() < 1e-10);
            assert!((r - 1.5).abs() < 1e-10);
        } else {
            panic!("Expected circle solution");
        }
    }
}
