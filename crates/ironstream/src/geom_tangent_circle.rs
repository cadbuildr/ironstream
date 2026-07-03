// FILE: geom_tangent_circle.rs
// occt: GccAna_Circ2d3Tan, GccAna_Circ2dBisec, GccAna_Lin2d2Tan, GccAna_Lin2dBisec

use std::f64::consts::PI;

/// 2D circle (for tangency computations).
#[derive(Clone, Copy, Debug)]
pub struct Circle2d {
    pub center: [f64; 2],
    pub radius: f64,
}

impl Circle2d {
    pub fn new(center: [f64; 2], radius: f64) -> Self { Self { center, radius } }

    pub fn contains_point(&self, p: [f64; 2]) -> bool {
        dist2(p, self.center) <= self.radius
    }

    pub fn is_tangent_to(&self, other: &Circle2d, tol: f64) -> bool {
        let d = dist2(self.center, other.center).sqrt();
        (d - (self.radius + other.radius)).abs() < tol  // external
        || (d - (self.radius - other.radius).abs()).abs() < tol  // internal
    }

    pub fn area(&self) -> f64 { PI * self.radius * self.radius }
    pub fn perimeter(&self) -> f64 { 2.0 * PI * self.radius }
}

/// occt: GccAna_Circ2d3Tan — circle tangent to three other circles/lines/points.
/// Apollonius circle (CPC case: circle tangent to 3 points = circumcircle).
pub struct Circ2d3Tan;

impl Circ2d3Tan {
    /// Circumcircle of three 2D points (tangent case: touches all three).
    pub fn circumcircle(p1: [f64; 2], p2: [f64; 2], p3: [f64; 2]) -> Option<Circle2d> {
        // Using determinant formula
        let ax = p2[0] - p1[0]; let ay = p2[1] - p1[1];
        let bx = p3[0] - p1[0]; let by = p3[1] - p1[1];
        let d = 2.0 * (ax * by - ay * bx);
        if d.abs() < 1e-14 { return None; } // collinear
        let ux = (by * (ax*ax + ay*ay) - ay * (bx*bx + by*by)) / d;
        let uy = (ax * (bx*bx + by*by) - bx * (ax*ax + ay*ay)) / d;
        let cx = p1[0] + ux; let cy = p1[1] + uy;
        let r = ((p1[0]-cx)*(p1[0]-cx) + (p1[1]-cy)*(p1[1]-cy)).sqrt();
        Some(Circle2d::new([cx, cy], r))
    }
}

/// occt: GccAna_Circ2dBisec — bisector locus of two circles.
pub struct Circ2dBisec;

impl Circ2dBisec {
    /// Radical axis of two circles (locus of equal power).
    /// Returns (point on axis, direction) or None if concentric.
    pub fn radical_axis(c1: &Circle2d, c2: &Circle2d) -> Option<([f64; 2], [f64; 2])> {
        let dx = c2.center[0] - c1.center[0]; let dy = c2.center[1] - c1.center[1];
        let d_sq = dx*dx + dy*dy;
        if d_sq < 1e-28 { return None; } // concentric
        let d = d_sq.sqrt();
        // Power: p1_power = d1^2 - r1^2, p2_power = d2^2 - r2^2
        // Radical axis is perpendicular to line joining centers at parameter t
        let t = (d_sq + c1.radius*c1.radius - c2.radius*c2.radius) / (2.0 * d_sq);
        let px = c1.center[0] + t * dx; let py = c1.center[1] + t * dy;
        let perp = [-dy / d, dx / d];
        Some(([px, py], perp))
    }
}

/// occt: GccAna_Lin2d2Tan — line tangent to two circles.
pub struct Lin2d2Tan;

impl Lin2d2Tan {
    /// Number of common external tangent lines to two non-overlapping circles.
    pub fn nb_external_tangents(c1: &Circle2d, c2: &Circle2d) -> usize {
        let d = dist2(c1.center, c2.center).sqrt();
        if d < 1e-14 { return 0; }
        let sum_r = c1.radius + c2.radius;
        let diff_r = (c1.radius - c2.radius).abs();
        if d > sum_r + 1e-12 { 4 }       // separate: 4 tangents (2 ext + 2 int)
        else if d > diff_r + 1e-12 { 2 } // intersect: 2 external
        else { 0 }                        // one inside other
    }

    /// Tangent line direction for external tangent (approximate, c1.r != c2.r).
    pub fn external_tangent_dir(c1: &Circle2d, c2: &Circle2d) -> Option<[f64; 2]> {
        let d = dist2(c1.center, c2.center).sqrt();
        if d < 1e-14 { return None; }
        if (c1.radius - c2.radius).abs() < 1e-14 {
            // Parallel case
            let dx = c2.center[0]-c1.center[0]; let dy = c2.center[1]-c1.center[1];
            Some([-dy/d, dx/d])
        } else {
            let dr = c2.radius - c1.radius;
            let cos_a = dr / d;
            if cos_a.abs() > 1.0 { return None; }
            let sin_a = (1.0 - cos_a*cos_a).sqrt();
            Some([cos_a, sin_a])
        }
    }
}

/// occt: GccAna_Lin2dBisec — bisector of two 2D lines/circles
pub struct Lin2dBisec;

impl Lin2dBisec {
    /// Angle bisector directions of two lines through origin with directions d1, d2.
    pub fn angle_bisectors(d1: [f64; 2], d2: [f64; 2]) -> ([f64; 2], [f64; 2]) {
        let n1 = normalize2(d1); let n2 = normalize2(d2);
        let b1 = normalize2([n1[0]+n2[0], n1[1]+n2[1]]);
        let b2 = [-b1[1], b1[0]];
        (b1, b2)
    }
}

fn dist2(a: [f64; 2], b: [f64; 2]) -> f64 { let dx=a[0]-b[0]; let dy=a[1]-b[1]; dx*dx+dy*dy }
fn normalize2(v: [f64; 2]) -> [f64; 2] {
    let m = (v[0]*v[0]+v[1]*v[1]).sqrt();
    if m > 1e-14 { [v[0]/m, v[1]/m] } else { [1.0, 0.0] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circumcircle_equilateral() {
        let s = 2.0f64;
        let h = s * 3.0f64.sqrt() / 2.0;
        let p1 = [0.0, 0.0]; let p2 = [s, 0.0]; let p3 = [s/2.0, h];
        let c = Circ2d3Tan::circumcircle(p1, p2, p3).unwrap();
        // Circumradius of equilateral triangle with side s = s / sqrt(3)
        assert!((c.radius - s / 3.0f64.sqrt()).abs() < 1e-8);
    }

    #[test]
    fn radical_axis() {
        let c1 = Circle2d::new([0.0,0.0], 3.0);
        let c2 = Circle2d::new([5.0,0.0], 2.0);
        let (pt, _dir) = Circ2dBisec::radical_axis(&c1, &c2).unwrap();
        // Power from c1: pt[0]^2 - 9; from c2: (pt[0]-5)^2 - 4; should be equal
        let p1 = pt[0]*pt[0] - 9.0;
        let p2 = (pt[0]-5.0)*(pt[0]-5.0) - 4.0;
        assert!((p1 - p2).abs() < 1e-8);
    }

    #[test]
    fn tangent_count() {
        let c1 = Circle2d::new([0.0,0.0], 1.0);
        let c2 = Circle2d::new([10.0,0.0], 1.0);
        assert_eq!(Lin2d2Tan::nb_external_tangents(&c1, &c2), 4);
    }

    #[test]
    fn angle_bisectors() {
        let d1 = [1.0, 0.0]; let d2 = [0.0, 1.0];
        let (b1, b2) = Lin2dBisec::angle_bisectors(d1, d2);
        assert!((b1[0] - 1.0/2.0f64.sqrt()).abs() < 1e-10);
        // b2 should be perpendicular to b1
        let dot = b1[0]*b2[0]+b1[1]*b2[1];
        assert!(dot.abs() < 1e-10);
    }

    #[test]
    fn circle_tangency() {
        let c1 = Circle2d::new([0.0,0.0], 2.0);
        let c2 = Circle2d::new([5.0,0.0], 3.0);
        assert!(c1.is_tangent_to(&c2, 1e-8));
    }
}
