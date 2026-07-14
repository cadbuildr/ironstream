// FILE: rust/ironstream/crates/ironstream/src/geom2d_api_inter.rs

// occt-note: intersection result point — holds a 2D point and the two curve parameters that produced it
#[derive(Clone, Debug)]
pub struct Geom2dIntersectionPoint {
    point: [f64; 2],
    param1: f64,
    param2: f64,
}

impl Geom2dIntersectionPoint {
    pub fn new(point: [f64; 2], param1: f64, param2: f64) -> Self {
        Self { point, param1, param2 }
    }

    pub fn point(&self) -> [f64; 2] {
        self.point
    }

    pub fn param1(&self) -> f64 {
        self.param1
    }

    pub fn param2(&self) -> f64 {
        self.param2
    }
}

// occt-note: intersection segment result — bounded overlap region between two curves
#[derive(Clone, Debug)]
pub struct Geom2dIntersectionSegment {
    pt1: Geom2dIntersectionPoint,
    pt2: Geom2dIntersectionPoint,
}

impl Geom2dIntersectionSegment {
    pub fn new(pt1: Geom2dIntersectionPoint, pt2: Geom2dIntersectionPoint) -> Self {
        Self { pt1, pt2 }
    }

    pub fn first_point(&self) -> &Geom2dIntersectionPoint {
        &self.pt1
    }

    pub fn last_point(&self) -> &Geom2dIntersectionPoint {
        &self.pt2
    }
}

// occt-ref: Geom2dAPI_InterCurveCurve // — intersection query result container
pub struct Geom2dInterCurveCurve {
    points: Vec<Geom2dIntersectionPoint>,
    segments: Vec<Geom2dIntersectionSegment>,
    is_done: bool,
}

impl Geom2dInterCurveCurve {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            segments: Vec::new(),
            is_done: false,
        }
    }

    pub fn add_point(&mut self, p: Geom2dIntersectionPoint) {
        self.points.push(p);
    }

    pub fn add_segment(&mut self, s: Geom2dIntersectionSegment) {
        self.segments.push(s);
    }

    pub fn mark_done(&mut self) {
        self.is_done = true;
    }

    pub fn nb_points(&self) -> usize {
        self.points.len()
    }

    pub fn nb_segments(&self) -> usize {
        self.segments.len()
    }

    pub fn point(&self, i: usize) -> &Geom2dIntersectionPoint {
        &self.points[i]
    }

    pub fn segment(&self, i: usize) -> &Geom2dIntersectionSegment {
        &self.segments[i]
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

impl Default for Geom2dInterCurveCurve {
    fn default() -> Self {
        Self::new()
    }
}
