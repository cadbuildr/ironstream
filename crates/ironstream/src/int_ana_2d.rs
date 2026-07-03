// FILE: int_ana_2d.rs

// occt: IntAna2d_IntPoint
#[derive(Clone, Copy, Debug)]
pub struct IntAna2dPoint {
    pub x: f64,
    pub y: f64,
    pub param1: f64,
    pub param2: f64,
}

impl IntAna2dPoint {
    pub fn new(x: f64, y: f64, param1: f64, param2: f64) -> Self {
        Self { x, y, param1, param2 }
    }
}

// occt: IntAna2d_Conic
pub struct IntAna2dConic {
    // Ax² + Bxy + Cy² + Dx + Ey + F = 0
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl IntAna2dConic {
    pub fn from_line(x0: f64, y0: f64, dx: f64, dy: f64) -> Self {
        Self {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: dy,
            e: -dx,
            f: -dy * x0 + dx * y0,
        }
    }

    pub fn from_circle(cx: f64, cy: f64, r: f64) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 1.0,
            d: -2.0 * cx,
            e: -2.0 * cy,
            f: cx * cx + cy * cy - r * r,
        }
    }
}

// occt: IntAna2d_AnaIntersection
pub struct IntAna2dAnaIntersection {
    points: Vec<IntAna2dPoint>,
    is_done: bool,
    is_empty: bool,
    is_identical: bool,
    is_parallel: bool,
}

impl IntAna2dAnaIntersection {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            is_done: false,
            is_empty: true,
            is_identical: false,
            is_parallel: false,
        }
    }

    /// Intersect two lines given as (point, direction) pairs using Cramer's rule.
    /// Line 1: (x1,y1) + t*(dx1,dy1), Line 2: (x2,y2) + s*(dx2,dy2).
    pub fn perform_line_line(
        x1: f64, y1: f64, dx1: f64, dy1: f64,
        x2: f64, y2: f64, dx2: f64, dy2: f64,
    ) -> Self {
        let cross = dx1 * dy2 - dy1 * dx2;
        if cross.abs() < 1e-14 {
            return Self {
                points: Vec::new(),
                is_done: true,
                is_empty: true,
                is_identical: false,
                is_parallel: true,
            };
        }
        let rx = x2 - x1;
        let ry = y2 - y1;
        let t = (rx * dy2 - ry * dx2) / cross;
        let s = (rx * dy1 - ry * dx1) / cross;
        let ix = x1 + t * dx1;
        let iy = y1 + t * dy1;
        Self {
            points: vec![IntAna2dPoint::new(ix, iy, t, s)],
            is_done: true,
            is_empty: false,
            is_identical: false,
            is_parallel: false,
        }
    }

    /// Intersect a line (point + direction) with a circle (centre + radius).
    /// Parameterize: x = lx + t*ldx, y = ly + t*ldy, substitute into circle equation.
    /// (lx + t*ldx - cx)² + (ly + t*ldy - cy)² = r²
    pub fn perform_line_circle(
        lx: f64, ly: f64, ldx: f64, ldy: f64,
        cx: f64, cy: f64, r: f64,
    ) -> Self {
        let fx = lx - cx;
        let fy = ly - cy;
        let a = ldx * ldx + ldy * ldy;
        let b = 2.0 * (fx * ldx + fy * ldy);
        let c = fx * fx + fy * fy - r * r;
        let disc = b * b - 4.0 * a * c;

        let tol = 1e-14 * r.max(1.0) * r.max(1.0);
        if disc < -tol {
            return Self {
                points: Vec::new(),
                is_done: true,
                is_empty: true,
                is_identical: false,
                is_parallel: false,
            };
        }

        let mut pts = Vec::new();
        if disc.abs() <= tol {
            let t = -b / (2.0 * a);
            let ix = lx + t * ldx;
            let iy = ly + t * ldy;
            let ang = (iy - cy).atan2(ix - cx);
            pts.push(IntAna2dPoint::new(ix, iy, t, ang));
        } else {
            let sq = disc.sqrt();
            let t1 = (-b - sq) / (2.0 * a);
            let t2 = (-b + sq) / (2.0 * a);
            let ix1 = lx + t1 * ldx;
            let iy1 = ly + t1 * ldy;
            let ang1 = (iy1 - cy).atan2(ix1 - cx);
            pts.push(IntAna2dPoint::new(ix1, iy1, t1, ang1));
            let ix2 = lx + t2 * ldx;
            let iy2 = ly + t2 * ldy;
            let ang2 = (iy2 - cy).atan2(ix2 - cx);
            pts.push(IntAna2dPoint::new(ix2, iy2, t2, ang2));
        }

        let empty = pts.is_empty();
        Self {
            points: pts,
            is_done: true,
            is_empty: empty,
            is_identical: false,
            is_parallel: false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    pub fn point(&self, idx: usize) -> Option<&IntAna2dPoint> {
        self.points.get(idx)
    }

    pub fn is_parallel(&self) -> bool {
        self.is_parallel
    }

    pub fn is_identical(&self) -> bool {
        self.is_identical
    }
}

impl Default for IntAna2dAnaIntersection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_line_x_and_y_axes_intersect_at_origin() {
        let inter = IntAna2dAnaIntersection::perform_line_line(
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        assert!(inter.is_done());
        assert!(!inter.is_empty());
        let p = inter.point(0).unwrap();
        assert!((p.x).abs() < 1e-12);
        assert!((p.y).abs() < 1e-12);
    }

    #[test]
    fn line_line_parallel_is_empty() {
        let inter = IntAna2dAnaIntersection::perform_line_line(
            0.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 1.0, 0.0,
        );
        assert!(inter.is_done());
        assert!(inter.is_empty());
        assert!(inter.is_parallel());
    }

    #[test]
    fn line_circle_two_points() {
        let inter = IntAna2dAnaIntersection::perform_line_circle(
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        );
        assert!(inter.is_done());
        assert_eq!(inter.nb_points(), 2);
    }

    #[test]
    fn line_circle_tangent_one_point() {
        let inter = IntAna2dAnaIntersection::perform_line_circle(
            0.0, 1.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        );
        assert!(inter.is_done());
        assert_eq!(inter.nb_points(), 1);
        let p = inter.point(0).unwrap();
        assert!(p.x.abs() < 1e-6);
        assert!((p.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn line_circle_no_intersection() {
        let inter = IntAna2dAnaIntersection::perform_line_circle(
            0.0, 2.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        );
        assert!(inter.is_done());
        assert!(inter.is_empty());
        assert_eq!(inter.nb_points(), 0);
    }
}
