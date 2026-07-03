// FILE: fillet2d_ext.rs
// occt: Fillet2d_FilletConstructor, Fillet2d_Fillet,
//       FilletSurf_InternalBuilder, FilletSurf_Builder

/// Status of a fillet construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilletStatus {
    Ok,
    NotDone,
    BadRadius,
    NoSolution,
    MorePointThanExpected,
    NotObligate,
    Error,
}

impl Default for FilletStatus {
    fn default() -> Self { Self::NotDone }
}

impl FilletStatus {
    pub fn is_done(&self) -> bool { *self == FilletStatus::Ok }
}

/// A 2D fillet arc description.
#[derive(Clone, Debug, Default)]
pub struct FilletArc2d {
    pub center: [f64; 2],
    pub radius: f64,
    pub start_param: f64,
    pub end_param: f64,
    pub is_forward: bool,
}

impl FilletArc2d {
    pub fn new(center: [f64; 2], radius: f64, t0: f64, t1: f64) -> Self {
        Self { center, radius, start_param: t0, end_param: t1, is_forward: true }
    }

    pub fn arc_length(&self) -> f64 {
        (self.end_param - self.start_param).abs() * self.radius
    }

    pub fn start_point(&self) -> [f64; 2] {
        [
            self.center[0] + self.radius * self.start_param.cos(),
            self.center[1] + self.radius * self.start_param.sin(),
        ]
    }

    pub fn end_point(&self) -> [f64; 2] {
        [
            self.center[0] + self.radius * self.end_param.cos(),
            self.center[1] + self.radius * self.end_param.sin(),
        ]
    }
}

// occt: Fillet2d_Fillet — individual fillet between two 2D edges
#[derive(Clone, Debug)]
pub struct Fillet2d {
    pub edge1_id: u32,
    pub edge2_id: u32,
    pub radius: f64,
    pub arc: Option<FilletArc2d>,
    pub point1: [f64; 2],   // tangent point on edge1
    pub point2: [f64; 2],   // tangent point on edge2
    pub status: FilletStatus,
    pub nb_solutions: usize,
}

impl Fillet2d {
    pub fn new(edge1: u32, edge2: u32, radius: f64) -> Self {
        Self {
            edge1_id: edge1,
            edge2_id: edge2,
            radius,
            arc: None,
            point1: [0.0, 0.0],
            point2: [0.0, 0.0],
            status: FilletStatus::NotDone,
            nb_solutions: 0,
        }
    }

    pub fn perform(&mut self) {
        if self.radius <= 0.0 {
            self.status = FilletStatus::BadRadius;
            return;
        }
        // Stub: place fillet at origin
        let arc = FilletArc2d::new([0.0, 0.0], self.radius, 0.0, std::f64::consts::FRAC_PI_2);
        self.point1 = arc.start_point();
        self.point2 = arc.end_point();
        self.arc = Some(arc);
        self.nb_solutions = 1;
        self.status = FilletStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_solutions(&self) -> usize { self.nb_solutions }
    pub fn curve(&self) -> Option<&FilletArc2d> { self.arc.as_ref() }
}

// occt: Fillet2d_FilletConstructor — builds multiple fillets at wire vertices
#[derive(Clone, Debug, Default)]
pub struct FilletConstructor {
    pub wire_id: u32,
    pub radius: f64,
    pub fillets: Vec<Fillet2d>,
    pub status: FilletStatus,
}

impl FilletConstructor {
    pub fn new(wire_id: u32) -> Self {
        Self { wire_id, radius: 0.0, ..Default::default() }
    }

    pub fn add_fillet(&mut self, edge1: u32, edge2: u32, radius: f64) {
        self.fillets.push(Fillet2d::new(edge1, edge2, radius));
    }

    pub fn perform(&mut self) {
        for f in &mut self.fillets { f.perform(); }
        let all_done = self.fillets.iter().all(|f| f.is_done());
        self.status = if all_done { FilletStatus::Ok } else { FilletStatus::Error };
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_fillets(&self) -> usize { self.fillets.len() }
    pub fn fillet(&self, idx: usize) -> Option<&Fillet2d> { self.fillets.get(idx) }

    pub fn nb_solutions(&self) -> usize {
        self.fillets.iter().filter(|f| f.is_done()).count()
    }
}

// occt: FilletSurf_Builder — builds fillet surfaces between two faces in 3D
#[derive(Clone, Debug)]
pub struct FilletSurfBuilder {
    pub face1_id: u32,
    pub face2_id: u32,
    pub radius: f64,
    pub tolerance: f64,
    pub nb_surface_patches: usize,
    pub start_param: f64,
    pub end_param: f64,
    pub status: FilletStatus,
}

impl FilletSurfBuilder {
    pub fn new(face1: u32, face2: u32, radius: f64) -> Self {
        Self {
            face1_id: face1,
            face2_id: face2,
            radius,
            tolerance: 1e-4,
            nb_surface_patches: 0,
            start_param: 0.0,
            end_param: 1.0,
            status: FilletStatus::NotDone,
        }
    }

    pub fn perform(&mut self) {
        if self.radius <= 0.0 {
            self.status = FilletStatus::BadRadius;
            return;
        }
        self.nb_surface_patches = 1;
        self.status = FilletStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_surfaces(&self) -> usize { self.nb_surface_patches }
    pub fn surface_first_param(&self) -> f64 { self.start_param }
    pub fn surface_last_param(&self) -> f64 { self.end_param }
    pub fn tolerance(&self) -> f64 { self.tolerance }
}

// occt: FilletSurf_InternalBuilder — internal stepping/parameter tracking
#[derive(Clone, Debug, Default)]
pub struct FilletSurfInternalBuilder {
    pub radius: f64,
    pub current_param: f64,
    pub step: f64,
    pub done: bool,
}

impl FilletSurfInternalBuilder {
    pub fn new(radius: f64) -> Self {
        Self { radius, current_param: 0.0, step: 0.01, done: false }
    }

    pub fn step(&mut self) { self.current_param += self.step; }
    pub fn is_at_end(&self) -> bool { self.current_param >= 1.0 }
    pub fn current_param(&self) -> f64 { self.current_param }
    pub fn reset(&mut self) { self.current_param = 0.0; self.done = false; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fillet2d_arc() {
        let arc = FilletArc2d::new([0.0, 0.0], 1.0, 0.0, std::f64::consts::FRAC_PI_2);
        assert!((arc.arc_length() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
        let sp = arc.start_point();
        assert!((sp[0] - 1.0).abs() < 1e-10); // cos(0) = 1
    }

    #[test]
    fn fillet2d_perform() {
        let mut f = Fillet2d::new(0, 1, 2.0);
        f.perform();
        assert!(f.is_done());
        assert_eq!(f.nb_solutions(), 1);
        assert!((f.curve().unwrap().radius - 2.0).abs() < 1e-10);
    }

    #[test]
    fn fillet2d_bad_radius() {
        let mut f = Fillet2d::new(0, 1, -1.0);
        f.perform();
        assert!(!f.is_done());
        assert_eq!(f.status, FilletStatus::BadRadius);
    }

    #[test]
    fn fillet_constructor() {
        let mut fc = FilletConstructor::new(0);
        fc.add_fillet(0, 1, 1.5);
        fc.add_fillet(1, 2, 2.0);
        fc.perform();
        assert!(fc.is_done());
        assert_eq!(fc.nb_fillets(), 2);
        assert_eq!(fc.nb_solutions(), 2);
    }

    #[test]
    fn fillet_surf_builder() {
        let mut b = FilletSurfBuilder::new(0, 1, 3.0);
        b.perform();
        assert!(b.is_done());
        assert_eq!(b.nb_surfaces(), 1);
    }
}
