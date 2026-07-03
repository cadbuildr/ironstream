// FILE: intf_ext.rs
// occt: Intf_Interference, Intf_SectionPoint, Intf_SectionLine, Intf_TangentZone

/// Status of an interference computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntfStatus {
    Done,
    NotDone,
    Empty,
    Error,
}

impl Default for IntfStatus {
    fn default() -> Self { Self::NotDone }
}

impl IntfStatus {
    pub fn is_done(&self) -> bool { *self == IntfStatus::Done }
}

/// A section point: intersection of two objects.
#[derive(Clone, Debug)]
pub struct IntfSectionPoint {
    pub pnt: [f64; 3],
    pub param_on_obj1: f64,
    pub param_on_obj2: f64,
    pub tolerance: f64,
}

impl IntfSectionPoint {
    pub fn new(pnt: [f64; 3], p1: f64, p2: f64, tol: f64) -> Self {
        Self { pnt, param_on_obj1: p1, param_on_obj2: p2, tolerance: tol }
    }

    pub fn pnt(&self) -> [f64; 3] { self.pnt }
    pub fn param_on_first(&self) -> f64 { self.param_on_obj1 }
    pub fn param_on_second(&self) -> f64 { self.param_on_obj2 }
    pub fn tolerance(&self) -> f64 { self.tolerance }
}

/// A section line: ordered list of section points.
#[derive(Clone, Debug, Default)]
pub struct IntfSectionLine {
    pub points: Vec<IntfSectionPoint>,
}

impl IntfSectionLine {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, pt: IntfSectionPoint) { self.points.push(pt); }
    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn value(&self, i: usize) -> Option<&IntfSectionPoint> { self.points.get(i) }
    pub fn get_point(&self, i: usize) -> Option<[f64; 3]> { self.points.get(i).map(|p| p.pnt) }
    pub fn is_empty(&self) -> bool { self.points.is_empty() }

    pub fn first(&self) -> Option<&IntfSectionPoint> { self.points.first() }
    pub fn last(&self) -> Option<&IntfSectionPoint> { self.points.last() }
}

/// A tangent zone: parameter interval where objects are tangent.
#[derive(Clone, Debug)]
pub struct IntfTangentZone {
    pub points: Vec<IntfSectionPoint>,
}

impl IntfTangentZone {
    pub fn new() -> Self { Self { points: Vec::new() } }

    pub fn append(&mut self, pt: IntfSectionPoint) { self.points.push(pt); }
    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn value(&self, i: usize) -> Option<&IntfSectionPoint> { self.points.get(i) }
    pub fn has_points(&self) -> bool { !self.points.is_empty() }

    pub fn param_range_on_first(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() { return None; }
        let first = self.points.first().unwrap().param_on_obj1;
        let last = self.points.last().unwrap().param_on_obj1;
        Some((first.min(last), first.max(last)))
    }
}

// occt: Intf_Interference — result of interference between two objects
#[derive(Clone, Debug, Default)]
pub struct IntfInterference {
    pub section_points: Vec<IntfSectionPoint>,
    pub section_lines: Vec<IntfSectionLine>,
    pub tangent_zones: Vec<IntfTangentZone>,
    pub status: IntfStatus,
    pub tolerance: f64,
}

impl IntfInterference {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance, ..Default::default() }
    }

    pub fn add_section_point(&mut self, pt: IntfSectionPoint) { self.section_points.push(pt); }
    pub fn add_section_line(&mut self, line: IntfSectionLine) { self.section_lines.push(line); }
    pub fn add_tangent_zone(&mut self, tz: IntfTangentZone) { self.tangent_zones.push(tz); }

    pub fn nb_section_points(&self) -> usize { self.section_points.len() }
    pub fn nb_section_lines(&self) -> usize { self.section_lines.len() }
    pub fn nb_tangent_zones(&self) -> usize { self.tangent_zones.len() }

    pub fn section_point(&self, i: usize) -> Option<&IntfSectionPoint> { self.section_points.get(i) }
    pub fn section_line(&self, i: usize) -> Option<&IntfSectionLine> { self.section_lines.get(i) }
    pub fn tangent_zone(&self, i: usize) -> Option<&IntfTangentZone> { self.tangent_zones.get(i) }

    pub fn is_empty(&self) -> bool {
        self.section_points.is_empty() && self.section_lines.is_empty() && self.tangent_zones.is_empty()
    }

    pub fn mark_done(&mut self) { self.status = IntfStatus::Done; }
    pub fn is_done(&self) -> bool { self.status.is_done() }
}

// occt: Intf_Tool — wraps interference computations for common geometry types
pub struct IntfTool;

impl IntfTool {
    /// Compute interference between two bounding boxes (overlap test).
    pub fn boxes_overlap(b1_min: [f64; 3], b1_max: [f64; 3], b2_min: [f64; 3], b2_max: [f64; 3], tol: f64) -> bool {
        for i in 0..3 {
            if b1_max[i] + tol < b2_min[i] || b2_max[i] + tol < b1_min[i] { return false; }
        }
        true
    }

    /// Compute interference between a segment and a triangle.
    pub fn segment_triangle(
        p0: [f64; 3], p1: [f64; 3],
        a: [f64; 3], b: [f64; 3], c: [f64; 3],
        tol: f64,
    ) -> Option<IntfSectionPoint> {
        // Moeller-Trumbore stub
        let _ = (p0, p1, a, b, c, tol);
        None
    }

    /// Trivial interference check by bounding sphere overlap.
    pub fn spheres_overlap(c1: [f64; 3], r1: f64, c2: [f64; 3], r2: f64, tol: f64) -> bool {
        let dist2 = (c1[0]-c2[0]).powi(2) + (c1[1]-c2[1]).powi(2) + (c1[2]-c2[2]).powi(2);
        let r_sum = r1 + r2 + tol;
        dist2 <= r_sum * r_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_point_basic() {
        let pt = IntfSectionPoint::new([1.0, 0.0, 0.0], 0.5, 0.3, 1e-6);
        assert_eq!(pt.pnt(), [1.0, 0.0, 0.0]);
        assert!((pt.param_on_first() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn section_line_basic() {
        let mut line = IntfSectionLine::new();
        line.append(IntfSectionPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 1e-6));
        line.append(IntfSectionPoint::new([1.0, 0.0, 0.0], 1.0, 0.5, 1e-6));
        assert_eq!(line.nb_points(), 2);
        assert!(!line.is_empty());
    }

    #[test]
    fn tangent_zone_range() {
        let mut tz = IntfTangentZone::new();
        tz.append(IntfSectionPoint::new([0.0, 0.0, 0.0], 0.2, 0.1, 1e-5));
        tz.append(IntfSectionPoint::new([0.5, 0.0, 0.0], 0.7, 0.4, 1e-5));
        let (lo, hi) = tz.param_range_on_first().unwrap();
        assert!((lo - 0.2).abs() < 1e-10);
        assert!((hi - 0.7).abs() < 1e-10);
    }

    #[test]
    fn interference_add_elements() {
        let mut intf = IntfInterference::new(1e-5);
        intf.add_section_point(IntfSectionPoint::new([0.0; 3], 0.0, 0.0, 1e-5));
        let mut line = IntfSectionLine::new();
        line.append(IntfSectionPoint::new([0.0; 3], 0.0, 0.0, 1e-5));
        intf.add_section_line(line);
        assert_eq!(intf.nb_section_points(), 1);
        assert_eq!(intf.nb_section_lines(), 1);
        assert!(!intf.is_empty());
    }

    #[test]
    fn intf_tool_boxes_overlap() {
        assert!(IntfTool::boxes_overlap([0.0; 3], [1.0; 3], [0.5; 3], [1.5; 3], 0.0));
        assert!(!IntfTool::boxes_overlap([0.0; 3], [1.0; 3], [2.0; 3], [3.0; 3], 0.0));
    }

    #[test]
    fn intf_tool_spheres_overlap() {
        assert!(IntfTool::spheres_overlap([0.0; 3], 1.0, [1.5, 0.0, 0.0], 1.0, 0.0));
        assert!(!IntfTool::spheres_overlap([0.0; 3], 0.4, [2.0, 0.0, 0.0], 0.4, 0.0));
    }
}
