// FILE: int_imp_par_gen_imp_tool.rs
// occt: IntImpParGen_ImpTool

/// Template tool for working with implicit curves.
/// Provides methods to evaluate implicit curves at parameters and
/// compute distances and gradients.
pub struct IntImpParGenImpTool {
    // Store curve parameters and type
    prm1: f64,
    prm2: f64,
    prm3: f64,
    curve_type: i32,
}

impl IntImpParGenImpTool {
    /// Creates a new implicit curve tool.
    pub fn new(curve_type: i32, p1: f64, p2: f64, p3: f64) -> Self {
        IntImpParGenImpTool {
            prm1: p1,
            prm2: p2,
            prm3: p3,
            curve_type,
        }
    }

    /// Evaluates the implicit curve at parameter U.
    /// Returns (x, y) point coordinates.
    pub fn value(&self, u: f64) -> (f64, f64) {
        match self.curve_type {
            1 => {
                // Circle: x^2 + y^2 = r^2
                let r = self.prm1;
                (r * u.cos(), r * u.sin())
            }
            2 => {
                // Ellipse: (x/a)^2 + (y/b)^2 = 1
                (self.prm1 * u.cos(), self.prm2 * u.sin())
            }
            3 => {
                // Parabola: y^2 = 4*p*x
                let p = self.prm1;
                (p * u * u, 2.0 * p * u)
            }
            4 => {
                // Hyperbola: (x/a)^2 - (y/b)^2 = 1
                (self.prm1 * u.cosh(), self.prm2 * u.sinh())
            }
            0 => {
                // Line: y = m*x + b
                (u, self.prm1 * u + self.prm2)
            }
            _ => (0.0, 0.0),
        }
    }

    /// Computes first derivative (tangent) at parameter U.
    /// Returns (point, tangent_vector).
    pub fn d1(&self, u: f64) -> ((f64, f64), (f64, f64)) {
        let point = self.value(u);
        let tangent = match self.curve_type {
            1 => {
                // Circle derivative
                let r = self.prm1;
                (-r * u.sin(), r * u.cos())
            }
            2 => {
                // Ellipse derivative
                (-self.prm1 * u.sin(), self.prm2 * u.cos())
            }
            3 => {
                // Parabola derivative
                let p = self.prm1;
                (2.0 * p * u, 2.0 * p)
            }
            4 => {
                // Hyperbola derivative
                (self.prm1 * u.sinh(), self.prm2 * u.cosh())
            }
            0 => {
                // Line derivative
                (1.0, self.prm1)
            }
            _ => (0.0, 0.0),
        };
        (point, tangent)
    }

    /// Computes second derivative (normal) at parameter U.
    /// Returns (point, tangent, normal).
    pub fn d2(&self, u: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
        let (point, tangent) = self.d1(u);
        let normal = match self.curve_type {
            1 => {
                // Circle second derivative
                let r = self.prm1;
                (-r * u.cos(), -r * u.sin())
            }
            2 => {
                // Ellipse second derivative
                (-self.prm1 * u.cos(), -self.prm2 * u.sin())
            }
            3 => {
                // Parabola second derivative
                (0.0, 0.0)
            }
            4 => {
                // Hyperbola second derivative
                (self.prm1 * u.cosh(), self.prm2 * u.sinh())
            }
            0 => {
                // Line second derivative
                (0.0, 0.0)
            }
            _ => (0.0, 0.0),
        };
        (point, tangent, normal)
    }

    /// Computes the signed distance between a point and the implicit curve.
    pub fn distance(&self, pnt: (f64, f64)) -> f64 {
        match self.curve_type {
            1 => {
                // Circle: distance = ||(pnt)|| - r
                let r = self.prm1;
                (pnt.0 * pnt.0 + pnt.1 * pnt.1).sqrt() - r
            }
            2 => {
                // Ellipse: approximate distance using iterative method
                let a = self.prm1;
                let b = self.prm2;
                Self::ellipse_distance(pnt.0, pnt.1, a, b)
            }
            3 => {
                // Parabola: y^2 - 4*p*x
                let p = self.prm1;
                pnt.1 * pnt.1 - 4.0 * p * pnt.0
            }
            4 => {
                // Hyperbola: (x/a)^2 - (y/b)^2 - 1
                let a = self.prm1;
                let b = self.prm2;
                (pnt.0 / a).powi(2) - (pnt.1 / b).powi(2) - 1.0
            }
            0 => {
                // Line: a*x + b*y + c = 0
                // Distance = (a*x + b*y + c) / sqrt(a^2 + b^2)
                let m = self.prm1;
                let c = self.prm2;
                (pnt.1 - m * pnt.0 - c) / (1.0 + m * m).sqrt()
            }
            _ => 0.0,
        }
    }

    /// Computes the gradient of the signed distance.
    pub fn grad_distance(&self, pnt: (f64, f64)) -> (f64, f64) {
        match self.curve_type {
            1 => {
                // Circle: gradient = pnt / ||pnt||
                let r = (pnt.0 * pnt.0 + pnt.1 * pnt.1).sqrt();
                if r > 1e-12 {
                    (pnt.0 / r, pnt.1 / r)
                } else {
                    (0.0, 0.0)
                }
            }
            2 => {
                // Ellipse: gradient of (x/a)^2 + (y/b)^2 - 1
                let a = self.prm1;
                let b = self.prm2;
                (2.0 * pnt.0 / (a * a), 2.0 * pnt.1 / (b * b))
            }
            3 => {
                // Parabola: gradient of y^2 - 4*p*x
                let p = self.prm1;
                (-4.0 * p, 2.0 * pnt.1)
            }
            4 => {
                // Hyperbola: gradient
                let a = self.prm1;
                let b = self.prm2;
                (2.0 * pnt.0 / (a * a), -2.0 * pnt.1 / (b * b))
            }
            0 => {
                // Line: gradient = (-m, 1) / sqrt(1 + m^2)
                let m = self.prm1;
                let norm = (1.0 + m * m).sqrt();
                (-m / norm, 1.0 / norm)
            }
            _ => (0.0, 0.0),
        }
    }

    /// Finds the parameter U of the point on the implicit curve
    /// corresponding to a given point.
    pub fn find_parameter(&self, pnt: (f64, f64)) -> f64 {
        match self.curve_type {
            1 => {
                // Circle: parameter = atan2(y, x)
                pnt.1.atan2(pnt.0)
            }
            2 => {
                // Ellipse: parameter = atan2(y/b, x/a)
                (pnt.1 / self.prm2).atan2(pnt.0 / self.prm1)
            }
            3 => {
                // Parabola: parameter = y / (2*p)
                let p = self.prm1;
                pnt.1 / (2.0 * p)
            }
            4 => {
                // Hyperbola: parameter = acosh(x/a)
                let a = self.prm1;
                (pnt.0 / a).acosh()
            }
            0 => {
                // Line: parameter = x
                pnt.0
            }
            _ => 0.0,
        }
    }

    /// Helper: compute approximate distance from point to ellipse.
    fn ellipse_distance(px: f64, py: f64, a: f64, b: f64) -> f64 {
        // Use Newton iteration: minimize |P(u) - pnt|^2
        // where P(u) = (a*cos(u), b*sin(u))
        let mut u = py.atan2(px);
        for _ in 0..10 {
            let c = u.cos();
            let s = u.sin();
            let dx = a * c - px;
            let dy = b * s - py;
            let dpx = -a * s;
            let dpy = b * c;
            let ddpx = -a * c;
            let ddpy = -b * s;

            let df = dx * dpx + dy * dpy;
            let ddf = dpx * dpx + dpy * dpy + dx * ddpx + dy * ddpy;

            if ddf.abs() > 1e-12 {
                u -= df / ddf;
            } else {
                break;
            }
        }

        let c = u.cos();
        let s = u.sin();
        let ex = a * c;
        let ey = b * s;
        ((ex - px).powi(2) + (ey - py).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_value() {
        let tool = IntImpParGenImpTool::new(1, 1.0, 0.0, 0.0);
        let (x, y) = tool.value(0.0);
        assert!((x - 1.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_circle_d1() {
        let tool = IntImpParGenImpTool::new(1, 1.0, 0.0, 0.0);
        let ((x, y), (dx, dy)) = tool.d1(0.0);
        assert!((x - 1.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
        assert!(dx.abs() < 1e-10);
        assert!((dy - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_circle_distance() {
        let tool = IntImpParGenImpTool::new(1, 1.0, 0.0, 0.0);
        let dist = tool.distance((2.0, 0.0));
        assert!((dist - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_circle_find_parameter() {
        let tool = IntImpParGenImpTool::new(1, 1.0, 0.0, 0.0);
        let u = tool.find_parameter((1.0, 0.0));
        assert!(u.abs() < 1e-10 || (u - std::f64::consts::TAU).abs() < 1e-10);
    }

    #[test]
    fn test_ellipse_value() {
        let tool = IntImpParGenImpTool::new(2, 2.0, 1.0, 0.0);
        let (x, y) = tool.value(0.0);
        assert!((x - 2.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_parabola_value() {
        let tool = IntImpParGenImpTool::new(3, 1.0, 0.0, 0.0);
        let (x, y) = tool.value(2.0);
        assert!((x - 4.0).abs() < 1e-10);
        assert!((y - 4.0).abs() < 1e-10);
    }
}
