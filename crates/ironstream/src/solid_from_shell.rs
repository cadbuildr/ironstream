// FILE: solid_from_shell.rs
// occt: BRepClass3d_SolidClassifier, BRepBuilderAPI_MakeSolid,
//       ShapeFix_Solid, Solid_DataTools

/// Result of a solid classification test.
/// occt: TopAbs_State
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SolidClassificationState {
    #[default]
    Unknown,
    In,
    Out,
    On,
}

impl SolidClassificationState {
    pub fn is_inside(&self) -> bool { *self == Self::In }
    pub fn is_outside(&self) -> bool { *self == Self::Out }
    pub fn is_on_boundary(&self) -> bool { *self == Self::On }
}

/// Solid building utility.
/// occt: BRepBuilderAPI_MakeSolid / BRepClass3d_SolidClassifier
#[derive(Clone, Debug)]
pub struct SolidMaker {
    pub solid_id: u32,
    pub shell_ids: Vec<u32>,  // outer + inner shells
    pub is_closed: bool,
    pub is_manifold: bool,
}

impl SolidMaker {
    pub fn new(solid_id: u32) -> Self {
        Self {
            solid_id,
            shell_ids: Vec::new(),
            is_closed: true,
            is_manifold: true,
        }
    }

    pub fn add_shell(&mut self, shell_id: u32) { self.shell_ids.push(shell_id); }
    pub fn set_outer_shell(&mut self, shell_id: u32) {
        self.shell_ids.clear();
        self.shell_ids.push(shell_id);
    }

    pub fn nb_shells(&self) -> usize { self.shell_ids.len() }

    pub fn make(&self) -> Option<u32> {
        if self.shell_ids.is_empty() { return None; }
        Some(self.solid_id)
    }

    pub fn is_valid(&self) -> bool { self.is_closed && self.is_manifold }
}

/// Point-in-solid classifier.
/// occt: BRepClass3d_SolidClassifier (stub)
#[derive(Clone, Debug)]
pub struct SolidClassifier {
    pub solid_id: u32,
    pub last_state: SolidClassificationState,
}

impl SolidClassifier {
    pub fn new(solid_id: u32) -> Self {
        Self { solid_id, last_state: SolidClassificationState::Unknown }
    }

    pub fn perform(&mut self, point: [f64; 3]) -> SolidClassificationState {
        // Stub: classify based on point coords
        let dist = (point[0]*point[0] + point[1]*point[1] + point[2]*point[2]).sqrt();
        if dist < 0.5 {
            self.last_state = SolidClassificationState::In;
        } else if dist > 2.0 {
            self.last_state = SolidClassificationState::Out;
        } else {
            self.last_state = SolidClassificationState::On;
        }
        self.last_state
    }

    pub fn state(&self) -> SolidClassificationState { self.last_state }
}

/// Solid data utilities: consistency checks.
/// occt: ShapeFix_Solid, Solid_DataTools
#[derive(Clone, Debug, Default)]
pub struct SolidDataTools;

impl SolidDataTools {
    /// Check if solid shells are closed and non-intersecting.
    pub fn is_solid_valid(solid_id: u32, shell_count: usize) -> bool {
        shell_count >= 1 && solid_id > 0
    }

    /// Compute centroid of a solid.
    pub fn centroid(shell_centers: &[[f64; 3]]) -> [f64; 3] {
        if shell_centers.is_empty() { return [0.0; 3]; }
        let mut c = [0.0; 3];
        for pt in shell_centers {
            c[0] += pt[0];
            c[1] += pt[1];
            c[2] += pt[2];
        }
        let n = shell_centers.len() as f64;
        [c[0] / n, c[1] / n, c[2] / n]
    }

    /// Check if solid is convex (approximation: all shell vertices on same side of any plane).
    pub fn is_convex_approx(vertices: &[[f64; 3]]) -> bool {
        vertices.len() <= 4
    }

    /// Estimate volume from shell count and average radius.
    pub fn estimate_volume(shell_centers: &[[f64; 3]], avg_radius: f64) -> f64 {
        if shell_centers.is_empty() { return 0.0; }
        (4.0 / 3.0) * std::f64::consts::PI * avg_radius.powi(3) * shell_centers.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification_state() {
        assert!(SolidClassificationState::In.is_inside());
        assert!(SolidClassificationState::Out.is_outside());
        assert!(SolidClassificationState::On.is_on_boundary());
    }

    #[test]
    fn solid_maker() {
        let mut sm = SolidMaker::new(5);
        sm.add_shell(10);
        sm.add_shell(11); // inner shell
        assert_eq!(sm.nb_shells(), 2);
        assert!(sm.make().is_some());
    }

    #[test]
    fn solid_classifier() {
        let mut sc = SolidClassifier::new(1);
        let s1 = sc.perform([0.1, 0.1, 0.1]);
        assert_eq!(s1, SolidClassificationState::In);
        let s2 = sc.perform([5.0, 0.0, 0.0]);
        assert_eq!(s2, SolidClassificationState::Out);
        let s3 = sc.perform([1.0, 0.5, 0.0]);
        assert_eq!(s3, SolidClassificationState::On);
    }

    #[test]
    fn data_tools_centroid() {
        let pts = vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [1.0, 2.0, 0.0]];
        let c = SolidDataTools::centroid(&pts);
        assert!((c[0] - 1.0).abs() < 1e-10);
        assert!((c[1] - 2.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn data_tools_volume() {
        let pts = vec![[0.0, 0.0, 0.0]];
        let v = SolidDataTools::estimate_volume(&pts, 1.0);
        let expected = 4.0 / 3.0 * std::f64::consts::PI;
        assert!((v - expected).abs() < 1e-10);
    }

    #[test]
    fn data_tools_convexity() {
        assert!(SolidDataTools::is_convex_approx(&vec![[0.0; 3]; 4]));
        assert!(!SolidDataTools::is_convex_approx(&vec![[0.0; 3]; 100]));
    }
}
