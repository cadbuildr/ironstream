// FILE: rust/ironstream/crates/ironstream/src/geom_segment_intersect.rs

// occt: IntAna2d result type — enum NoIntersection, Point, Coincident, Parallel
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SegIntersectResult {
    NoIntersection,
    Point,
    Coincident,
    Parallel,
}

// occt: IntAna2d_AnaIntersection for 2D rays
#[derive(Clone, Copy, Debug)]
pub struct SegIntersect2d {
    result_type: SegIntersectResult,
    intersection_point: [f64; 2],
    param1: f64,
    param2: f64,
}

impl SegIntersect2d {
    pub fn new() -> Self {
        Self {
            result_type: SegIntersectResult::NoIntersection,
            intersection_point: [0.0, 0.0],
            param1: 0.0,
            param2: 0.0,
        }
    }

    /// Intersect two 2D rays: ray1 = p1 + t*d1, ray2 = p2 + s*d2.
    /// cross = d1.x*d2.y - d1.y*d2.x; if |cross|<1e-10 => Parallel; else solve for t,s and set Point.
    pub fn intersect(
        &mut self,
        p1: [f64; 2],
        d1: [f64; 2],
        p2: [f64; 2],
        d2: [f64; 2],
    ) {
        let cross = d1[0] * d2[1] - d1[1] * d2[0];

        if cross.abs() < 1e-10 {
            // Directions are parallel; check if rays are coincident.
            // A point on ray2 (p2) relative to p1: check if (p2-p1) x d1 == 0.
            let dx = p2[0] - p1[0];
            let dy = p2[1] - p1[1];
            let offset_cross = dx * d1[1] - dy * d1[0];
            if offset_cross.abs() < 1e-10 {
                self.result_type = SegIntersectResult::Coincident;
            } else {
                self.result_type = SegIntersectResult::Parallel;
            }
            self.intersection_point = [0.0, 0.0];
            self.param1 = 0.0;
            self.param2 = 0.0;
            return;
        }

        // Solve: p1 + t*d1 = p2 + s*d2
        // t = ((p2 - p1) x d2) / (d1 x d2)
        // s = ((p2 - p1) x d1) / (d1 x d2)
        let dx = p2[0] - p1[0];
        let dy = p2[1] - p1[1];

        let t = (dx * d2[1] - dy * d2[0]) / cross;
        let s = (dx * d1[1] - dy * d1[0]) / cross;

        self.param1 = t;
        self.param2 = s;
        self.intersection_point = [p1[0] + t * d1[0], p1[1] + t * d1[1]];
        self.result_type = SegIntersectResult::Point;
    }

    pub fn result_type(&self) -> SegIntersectResult {
        self.result_type
    }

    pub fn point(&self) -> [f64; 2] {
        self.intersection_point
    }

    pub fn param1(&self) -> f64 {
        self.param1
    }

    pub fn param2(&self) -> f64 {
        self.param2
    }
}

impl Default for SegIntersect2d {
    fn default() -> Self {
        Self::new()
    }
}

// occt: 3D ray closest approach
#[derive(Clone, Copy, Debug)]
pub struct SegIntersect3d {
    result_type: SegIntersectResult,
    closest_pt1: [f64; 3],
    closest_pt2: [f64; 3],
    distance: f64,
}

impl SegIntersect3d {
    pub fn new() -> Self {
        Self {
            result_type: SegIntersectResult::NoIntersection,
            closest_pt1: [0.0, 0.0, 0.0],
            closest_pt2: [0.0, 0.0, 0.0],
            distance: 0.0,
        }
    }

    /// Compute the closest approach between two 3D rays:
    ///   ray1 = p1 + t*d1,  ray2 = p2 + s*d2.
    ///
    /// Uses the standard closest-point-on-two-lines formula.
    /// If the rays are parallel (|d1 x d2| < 1e-10) sets Parallel (or Coincident).
    /// Otherwise sets Point (or NoIntersection when distance > 0, but we still
    /// record the closest approach) — callers can inspect distance() to decide.
    pub fn intersect(
        &mut self,
        p1: [f64; 3],
        d1: [f64; 3],
        p2: [f64; 3],
        d2: [f64; 3],
    ) {
        // w = p1 - p2
        let w = [p1[0] - p2[0], p1[1] - p2[1], p1[2] - p2[2]];

        let a = dot3(d1, d1); // d1 · d1
        let b = dot3(d1, d2); // d1 · d2
        let c = dot3(d2, d2); // d2 · d2
        let d = dot3(d1, w);  // d1 · w
        let e = dot3(d2, w);  // d2 · w

        let denom = a * c - b * b; // |d1 x d2|²

        if denom.abs() < 1e-20 {
            // Rays are parallel.
            // Check if they are coincident: project w onto the plane perp to d1.
            let w_cross_d1 = cross3(w, d1);
            let cross_len_sq = dot3(w_cross_d1, w_cross_d1);
            if cross_len_sq < 1e-20 {
                self.result_type = SegIntersectResult::Coincident;
                self.closest_pt1 = p1;
                self.closest_pt2 = p1;
                self.distance = 0.0;
            } else {
                // Parallel but offset; closest point is foot from p2 onto ray1.
                let t = -d / a;
                let foot = [
                    p1[0] + t * d1[0],
                    p1[1] + t * d1[1],
                    p1[2] + t * d1[2],
                ];
                let dist = dist3(foot, p2);
                self.result_type = SegIntersectResult::Parallel;
                self.closest_pt1 = foot;
                self.closest_pt2 = p2;
                self.distance = dist;
            }
            return;
        }

        let t = (b * e - c * d) / denom;
        let s = (a * e - b * d) / denom;

        let cp1 = [
            p1[0] + t * d1[0],
            p1[1] + t * d1[1],
            p1[2] + t * d1[2],
        ];
        let cp2 = [
            p2[0] + s * d2[0],
            p2[1] + s * d2[1],
            p2[2] + s * d2[2],
        ];
        let dist = dist3(cp1, cp2);

        self.closest_pt1 = cp1;
        self.closest_pt2 = cp2;
        self.distance = dist;
        if dist < 1e-10 {
            self.result_type = SegIntersectResult::Point;
        } else {
            self.result_type = SegIntersectResult::NoIntersection;
        }
    }

    pub fn result_type(&self) -> SegIntersectResult {
        self.result_type
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn closest_pt1(&self) -> [f64; 3] {
        self.closest_pt1
    }

    pub fn closest_pt2(&self) -> [f64; 3] {
        self.closest_pt2
    }
}

impl Default for SegIntersect3d {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Internal helpers — no std beyond arithmetic.
// ---------------------------------------------------------------------------

#[inline]
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}
