// FILE: intf_tool.rs
// occt: Intf_Interference, Intf_SectionLine, Intf_TangentZone,
//       Intf_SectionPoint, Intf_Tool

/// A single point where two shapes intersect.
#[derive(Clone, Debug)]
pub struct SectionPoint {
    pub point: [f64; 3],
    pub param_on_g1: f64,
    pub param_on_g2: f64,
    pub tolerance: f64,
}

impl SectionPoint {
    pub fn new(pt: [f64; 3], p1: f64, p2: f64, tol: f64) -> Self {
        Self { point: pt, param_on_g1: p1, param_on_g2: p2, tolerance: tol }
    }

    pub fn is_on_boundary_of_g1(&self) -> bool { self.param_on_g1 <= self.tolerance }
    pub fn is_on_boundary_of_g2(&self) -> bool { self.param_on_g2 <= self.tolerance }
}

// occt: Intf_SectionLine — a connected sequence of section points
#[derive(Clone, Debug, Default)]
pub struct SectionLine {
    pub points: Vec<SectionPoint>,
    pub is_closed: bool,
}

impl SectionLine {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, pt: SectionPoint) { self.points.push(pt); }
    pub fn prepend(&mut self, pt: SectionPoint) { self.points.insert(0, pt); }

    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn is_empty(&self) -> bool { self.points.is_empty() }

    pub fn close(&mut self) {
        if self.points.len() >= 2 { self.is_closed = true; }
    }

    pub fn get(&self, idx: usize) -> Option<&SectionPoint> { self.points.get(idx) }

    pub fn bounding_box(&self) -> Option<([f64; 3], [f64; 3])> {
        if self.points.is_empty() { return None; }
        let mut mn = self.points[0].point;
        let mut mx = self.points[0].point;
        for p in &self.points {
            for i in 0..3 {
                if p.point[i] < mn[i] { mn[i] = p.point[i]; }
                if p.point[i] > mx[i] { mx[i] = p.point[i]; }
            }
        }
        Some((mn, mx))
    }
}

// occt: Intf_TangentZone — region where two shapes are tangent (touch without crossing)
#[derive(Clone, Debug, Default)]
pub struct TangentZone {
    pub points: Vec<SectionPoint>,
    pub param_range_g1: (f64, f64),
    pub param_range_g2: (f64, f64),
}

impl TangentZone {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, pt: SectionPoint) {
        let p1 = pt.param_on_g1;
        let p2 = pt.param_on_g2;
        if self.points.is_empty() {
            self.param_range_g1 = (p1, p1);
            self.param_range_g2 = (p2, p2);
        } else {
            self.param_range_g1.0 = self.param_range_g1.0.min(p1);
            self.param_range_g1.1 = self.param_range_g1.1.max(p1);
            self.param_range_g2.0 = self.param_range_g2.0.min(p2);
            self.param_range_g2.1 = self.param_range_g2.1.max(p2);
        }
        self.points.push(pt);
    }

    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn is_empty(&self) -> bool { self.points.is_empty() }
    pub fn extent_g1(&self) -> f64 { self.param_range_g1.1 - self.param_range_g1.0 }
    pub fn extent_g2(&self) -> f64 { self.param_range_g2.1 - self.param_range_g2.0 }
}

// occt: Intf_Interference — result of an interference computation between two shapes
#[derive(Clone, Debug, Default)]
pub struct Interference {
    pub section_lines: Vec<SectionLine>,
    pub tangent_zones: Vec<TangentZone>,
    pub tolerance: f64,
    pub self_interference: bool,
}

impl Interference {
    pub fn new(tol: f64) -> Self {
        Self { tolerance: tol, ..Default::default() }
    }

    pub fn add_section_line(&mut self, line: SectionLine) { self.section_lines.push(line); }
    pub fn add_tangent_zone(&mut self, zone: TangentZone) { self.tangent_zones.push(zone); }

    pub fn nb_section_lines(&self) -> usize { self.section_lines.len() }
    pub fn nb_tangent_zones(&self) -> usize { self.tangent_zones.len() }

    pub fn section_line(&self, idx: usize) -> Option<&SectionLine> { self.section_lines.get(idx) }
    pub fn tangent_zone(&self, idx: usize) -> Option<&TangentZone> { self.tangent_zones.get(idx) }

    pub fn nb_total_section_points(&self) -> usize {
        self.section_lines.iter().map(|l| l.nb_points()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.section_lines.is_empty() && self.tangent_zones.is_empty()
    }

    pub fn clear(&mut self) {
        self.section_lines.clear();
        self.tangent_zones.clear();
    }
}

/// AABB-based interference detection stub.
// occt: Intf_Tool
#[derive(Clone, Debug, Default)]
pub struct IntfTool;

impl IntfTool {
    pub fn new() -> Self { Self }

    /// Check if two axis-aligned bounding boxes overlap.
    pub fn boxes_overlap(
        &self,
        min1: [f64; 3], max1: [f64; 3],
        min2: [f64; 3], max2: [f64; 3],
    ) -> bool {
        (0..3).all(|i| min1[i] <= max2[i] && min2[i] <= max1[i])
    }

    /// Build an interference result for two overlapping segments in 3D.
    pub fn segment_segment(
        &self,
        p1: [f64; 3], p2: [f64; 3],
        p3: [f64; 3], p4: [f64; 3],
        tol: f64,
    ) -> Interference {
        let mut intf = Interference::new(tol);
        // Closest-point on segment 1 to midpoint of segment 2 — simplified stub
        let mid2 = [(p3[0]+p4[0])*0.5, (p3[1]+p4[1])*0.5, (p3[2]+p4[2])*0.5];
        let d01 = [p2[0]-p1[0], p2[1]-p1[1], p2[2]-p1[2]];
        let len2 = d01[0]*d01[0] + d01[1]*d01[1] + d01[2]*d01[2];
        if len2 < 1e-28 { return intf; }
        let t = ((mid2[0]-p1[0])*d01[0] + (mid2[1]-p1[1])*d01[1] + (mid2[2]-p1[2])*d01[2]) / len2;
        let t = t.clamp(0.0, 1.0);
        let closest = [p1[0]+t*d01[0], p1[1]+t*d01[1], p1[2]+t*d01[2]];
        let dist2 = (0..3).map(|i| (closest[i]-mid2[i]).powi(2)).sum::<f64>();
        if dist2.sqrt() <= tol {
            let mut line = SectionLine::new();
            line.append(SectionPoint::new(closest, t, 0.5, tol));
            intf.add_section_line(line);
        }
        intf
    }
}

/// Common interference types for shape-vs-shape queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterferenceType {
    Unknown,
    Touching,    // single point contact
    Edge,        // along an edge
    Face,        // face overlap
    Mixed,
}

impl Default for InterferenceType {
    fn default() -> Self { Self::Unknown }
}

/// Summary report of an interference check.
#[derive(Clone, Debug, Default)]
pub struct InterferenceReport {
    pub interference_type: InterferenceType,
    pub nb_contacts: usize,
    pub max_penetration: f64,
    pub contact_points: Vec<[f64; 3]>,
}

impl InterferenceReport {
    pub fn none() -> Self { Self::default() }

    pub fn touching(pt: [f64; 3]) -> Self {
        Self {
            interference_type: InterferenceType::Touching,
            nb_contacts: 1,
            max_penetration: 0.0,
            contact_points: vec![pt],
        }
    }

    pub fn has_interference(&self) -> bool { self.nb_contacts > 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_line_ops() {
        let mut sl = SectionLine::new();
        sl.append(SectionPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 1e-6));
        sl.append(SectionPoint::new([1.0, 0.0, 0.0], 1.0, 1.0, 1e-6));
        assert_eq!(sl.nb_points(), 2);
        let (mn, mx) = sl.bounding_box().unwrap();
        assert!((mn[0]).abs() < 1e-10);
        assert!((mx[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn tangent_zone_extent() {
        let mut tz = TangentZone::new();
        tz.append(SectionPoint::new([0.0; 3], 0.1, 0.2, 1e-6));
        tz.append(SectionPoint::new([1.0; 3], 0.9, 0.8, 1e-6));
        assert!((tz.extent_g1() - 0.8).abs() < 1e-10);
    }

    #[test]
    fn interference_empty() {
        let intf = Interference::new(1e-7);
        assert!(intf.is_empty());
        assert_eq!(intf.nb_total_section_points(), 0);
    }

    #[test]
    fn intf_tool_boxes_overlap() {
        let tool = IntfTool::new();
        assert!(tool.boxes_overlap([0.0; 3], [1.0; 3], [0.5, 0.5, 0.5], [2.0; 3]));
        assert!(!tool.boxes_overlap([0.0; 3], [1.0; 3], [2.0; 3], [3.0; 3]));
    }

    #[test]
    fn intf_tool_segment_segment() {
        let tool = IntfTool::new();
        let intf = tool.segment_segment(
            [0.0, 0.0, 0.0], [1.0, 0.0, 0.0],
            [0.5, -0.5, 0.0], [0.5, 0.5, 0.0],
            1.0,
        );
        assert!(!intf.is_empty());
    }

    #[test]
    fn interference_report() {
        let r = InterferenceReport::touching([1.0, 2.0, 3.0]);
        assert!(r.has_interference());
        assert_eq!(r.nb_contacts, 1);
    }
}
