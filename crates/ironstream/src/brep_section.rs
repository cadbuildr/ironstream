// FILE: brep_section.rs
// occt: BRepAlgoAPI_Section, BRepBuilderAPI_MakeEdge (from intersection curve)

// occt: BRepAlgoAPI_Section
/// Computes the intersection curve between two shapes.
#[derive(Clone, Debug)]
pub struct SectionOperation {
    pub shape_a: Vec<[f64; 3]>,
    pub shape_b_plane: Option<[f64; 4]>,
    pub shape_b_pts: Vec<[f64; 3]>,
    pub result_curves: Vec<SectionCurve>,
    pub done: bool,
    pub approximation: bool,
}

/// One connected section curve (polyline approximation).
#[derive(Clone, Debug)]
pub struct SectionCurve {
    pub points: Vec<[f64; 3]>,
    pub is_closed: bool,
}

impl SectionCurve {
    pub fn new(points: Vec<[f64; 3]>) -> Self {
        let is_closed = points.len() > 2 && {
            let a = &points[0]; let b = points.last().unwrap();
            let dx = a[0]-b[0]; let dy = a[1]-b[1]; let dz = a[2]-b[2];
            (dx*dx+dy*dy+dz*dz).sqrt() < 1e-10
        };
        Self { points, is_closed }
    }

    pub fn nb_points(&self) -> usize { self.points.len() }
}

impl SectionOperation {
    pub fn new(shape_a: Vec<[f64; 3]>) -> Self {
        Self {
            shape_a, shape_b_plane: None, shape_b_pts: Vec::new(),
            result_curves: Vec::new(), done: false, approximation: true,
        }
    }

    pub fn set_plane(&mut self, normal: [f64; 3], d: f64) {
        self.shape_b_plane = Some([normal[0], normal[1], normal[2], d]);
    }

    pub fn set_shape_b(&mut self, pts: Vec<[f64; 3]>) {
        self.shape_b_pts = pts;
    }

    pub fn set_approximation(&mut self, approx: bool) {
        self.approximation = approx;
    }

    pub fn build(&mut self) {
        // Stub: if a plane is set, project shape_a points onto it
        if let Some(plane) = self.shape_b_plane {
            let n = [plane[0], plane[1], plane[2]];
            let d = plane[3];
            let pts: Vec<[f64; 3]> = self.shape_a.iter().map(|&p| {
                let dist = p[0]*n[0]+p[1]*n[1]+p[2]*n[2] - d;
                [p[0]-n[0]*dist, p[1]-n[1]*dist, p[2]-n[2]*dist]
            }).collect();
            if !pts.is_empty() {
                self.result_curves.push(SectionCurve::new(pts));
            }
        }
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_edges(&self) -> usize { self.result_curves.len() }
}

// occt: BOPAlgo_Section
/// BOP-based section (same concept, different implementation path).
pub type BopSection = SectionOperation;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_with_xy_plane() {
        let pts = vec![[0.0,0.0,1.0],[1.0,0.0,1.0],[2.0,0.0,1.0]];
        let mut s = SectionOperation::new(pts);
        s.set_plane([0.0, 0.0, 1.0], 0.0);
        s.build();
        assert!(s.is_done());
        assert_eq!(s.nb_edges(), 1);
        let curve = &s.result_curves[0];
        for p in &curve.points {
            assert!(p[2].abs() < 1e-10, "section should be at z=0");
        }
    }

    #[test]
    fn section_no_plane() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let mut s = SectionOperation::new(pts);
        s.build();
        assert!(s.is_done());
        assert_eq!(s.nb_edges(), 0);
    }

    #[test]
    fn section_curve_closed() {
        let pts = vec![
            [0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0],[0.0,0.0,0.0]
        ];
        let c = SectionCurve::new(pts);
        assert!(c.is_closed);
    }
}
