// FILE: rust/ironstream/crates/ironstream/src/int_surf.rs

// occt: IntSurf_PntOn2S // — point on two surfaces
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntSurfPntOn2S {
    pub pt: [f64; 3],
    pub u1: f64,
    pub v1: f64,
    pub u2: f64,
    pub v2: f64,
}

impl IntSurfPntOn2S {
    pub fn new(pt: [f64; 3], u1: f64, v1: f64, u2: f64, v2: f64) -> Self {
        Self { pt, u1, v1, u2, v2 }
    }

    pub fn point(&self) -> [f64; 3] {
        self.pt
    }

    pub fn parameters(&self) -> (f64, f64, f64, f64) {
        (self.u1, self.v1, self.u2, self.v2)
    }

    pub fn set_value(&mut self, pt: [f64; 3], u1: f64, v1: f64, u2: f64, v2: f64) {
        self.pt = pt;
        self.u1 = u1;
        self.v1 = v1;
        self.u2 = u2;
        self.v2 = v2;
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.pt[0] - other.pt[0];
        let dy = self.pt[1] - other.pt[1];
        let dz = self.pt[2] - other.pt[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// occt: IntSurf_Transition // — topology transition type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntSurfTransition {
    Unknown,
    Touch,
    Undecided,
    In,
    Out,
}

impl IntSurfTransition {
    pub fn trans_s1(&self) -> IntSurfTransition {
        *self
    }

    pub fn trans_s2(&self) -> IntSurfTransition {
        match self {
            IntSurfTransition::In => IntSurfTransition::Out,
            IntSurfTransition::Out => IntSurfTransition::In,
            other => *other,
        }
    }
}

// occt: IntSurf_LineOn2S // — ordered list of IntSurf_PntOn2S
pub struct IntSurfLineOn2S {
    points: Vec<IntSurfPntOn2S>,
}

impl IntSurfLineOn2S {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add(&mut self, p: IntSurfPntOn2S) {
        self.points.push(p);
    }

    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    /// 0-based indexing, panics if out of range
    pub fn value(&self, i: usize) -> &IntSurfPntOn2S {
        &self.points[i]
    }

    pub fn reverse(&mut self) {
        self.points.reverse();
    }

    pub fn first(&self) -> Option<&IntSurfPntOn2S> {
        self.points.first()
    }

    pub fn last(&self) -> Option<&IntSurfPntOn2S> {
        self.points.last()
    }
}

impl Default for IntSurfLineOn2S {
    fn default() -> Self {
        Self::new()
    }
}

// occt: IntSurf_Situation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntSurfOrientation {
    Inside,
    Outside,
    Unknown,
}

// occt: IntSurf_InteriorPoint
pub struct IntSurfInteriorPoint {
    pub pnt: IntSurfPntOn2S,
    pub tangent: [f64; 3],
}

impl IntSurfInteriorPoint {
    pub fn new(pnt: IntSurfPntOn2S, tangent: [f64; 3]) -> Self {
        Self { pnt, tangent }
    }
}

// occt: IntSurf_Quadric
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntSurfQuadricKind {
    Plane,
    Cylinder,
    Cone,
    Sphere,
}

// occt: IntSurf_Quadric
pub struct IntSurfQuadric {
    pub kind: IntSurfQuadricKind,
    pub radius: f64,
    pub half_angle: f64,
}

impl IntSurfQuadric {
    pub fn plane() -> Self {
        Self { kind: IntSurfQuadricKind::Plane, radius: 0.0, half_angle: 0.0 }
    }

    pub fn cylinder(r: f64) -> Self {
        Self { kind: IntSurfQuadricKind::Cylinder, radius: r, half_angle: 0.0 }
    }

    pub fn sphere(r: f64) -> Self {
        Self { kind: IntSurfQuadricKind::Sphere, radius: r, half_angle: 0.0 }
    }

    pub fn cone(half_angle: f64, r: f64) -> Self {
        Self { kind: IntSurfQuadricKind::Cone, radius: r, half_angle }
    }
}
