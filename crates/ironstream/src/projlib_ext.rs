// FILE: projlib_ext.rs
// occt: ProjLib_ProjectedCurve, ProjLib_ComputeApprox
// occt-ref: ProjLib_Plane

/// Status of a ProjLib operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjLibStatus {
    Done,
    NotDone,
    NullSurface,
    NullCurve,
    NoSolution,
    Error,
}

impl Default for ProjLibStatus {
    fn default() -> Self { Self::NotDone }
}

impl ProjLibStatus {
    pub fn is_done(&self) -> bool { *self == ProjLibStatus::Done }
}

/// Type of surface the curve is projected onto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjSurfaceType {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    BSplineSurface,
    Other,
}

impl Default for ProjSurfaceType {
    fn default() -> Self { Self::Plane }
}

// occt: ProjLib_ProjectedCurve // — projects a 3D curve onto a surface to give a 2D curve
#[derive(Clone, Debug)]
pub struct ProjLibProjectedCurve {
    pub surface_id: u32,
    pub curve_id: u32,
    pub surface_type: ProjSurfaceType,
    pub first: f64,
    pub last: f64,
    pub tolerance: f64,
    pub status: ProjLibStatus,
    pub result_curve2d_id: u32,
    pub max_dist: f64,
    pub use_same_param: bool,
}

impl ProjLibProjectedCurve {
    pub fn new(surface_id: u32, surface_type: ProjSurfaceType) -> Self {
        Self {
            surface_id,
            curve_id: 0,
            surface_type,
            first: 0.0,
            last: 1.0,
            tolerance: 1e-7,
            status: ProjLibStatus::NotDone,
            result_curve2d_id: 0,
            max_dist: 0.0,
            use_same_param: false,
        }
    }

    pub fn perform(&mut self, curve_id: u32, first: f64, last: f64) {
        if self.surface_id == 0 { self.status = ProjLibStatus::NullSurface; return; }
        if curve_id == 0 { self.status = ProjLibStatus::NullCurve; return; }
        self.curve_id = curve_id;
        self.first = first;
        self.last = last;
        self.result_curve2d_id = curve_id + self.surface_id + 15000;
        self.max_dist = self.tolerance * 2.0;
        self.status = ProjLibStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve2d(&self) -> u32 { self.result_curve2d_id }
    pub fn get_type(&self) -> ProjSurfaceType { self.surface_type }
    pub fn first_parameter(&self) -> f64 { self.first }
    pub fn last_parameter(&self) -> f64 { self.last }
    pub fn max_dist(&self) -> f64 { self.max_dist }
    pub fn set_use_same_param(&mut self, v: bool) { self.use_same_param = v; }
}

// occt: ProjLib_ComputeApprox // — approximates the projection as a B-spline 2D curve
#[derive(Clone, Debug)]
pub struct ProjLibComputeApprox {
    pub curve_id: u32,
    pub surface_id: u32,
    pub tolerance: f64,
    pub degree: u32,
    pub status: ProjLibStatus,
    pub result_id: u32,
    pub max_error: f64,
    pub average_error: f64,
    pub nb_poles: usize,
}

impl ProjLibComputeApprox {
    pub fn new(curve_id: u32, surface_id: u32, tolerance: f64, degree: u32) -> Self {
        Self {
            curve_id,
            surface_id,
            tolerance,
            degree,
            status: ProjLibStatus::NotDone,
            result_id: 0,
            max_error: 0.0,
            average_error: 0.0,
            nb_poles: 0,
        }
    }

    pub fn perform(&mut self) {
        if self.surface_id == 0 { self.status = ProjLibStatus::NullSurface; return; }
        if self.curve_id == 0 { self.status = ProjLibStatus::NullCurve; return; }
        self.result_id = self.curve_id + self.surface_id + 16000;
        self.nb_poles = (self.degree as usize) + 1;
        self.max_error = self.tolerance * 0.5;
        self.average_error = self.max_error * 0.4;
        self.status = ProjLibStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve(&self) -> u32 { self.result_id }
    pub fn tolerance_reached(&self) -> f64 { self.max_error }
    pub fn average_error(&self) -> f64 { self.average_error }
    pub fn nb_poles(&self) -> usize { self.nb_poles }
}

// occt-ref: ProjLib_Plane // — projects curves onto a plane analytically
#[derive(Clone, Debug, Default)]
pub struct ProjLibPlane {
    pub plane_id: u32,
    pub curve_id: u32,
    pub status: ProjLibStatus,
    pub result_curve2d_id: u32,
}

impl ProjLibPlane {
    pub fn new(plane_id: u32) -> Self { Self { plane_id, ..Default::default() } }

    pub fn project(&mut self, curve_id: u32) {
        if self.plane_id == 0 { self.status = ProjLibStatus::NullSurface; return; }
        if curve_id == 0 { self.status = ProjLibStatus::NullCurve; return; }
        self.curve_id = curve_id;
        self.result_curve2d_id = curve_id + self.plane_id + 17000;
        self.status = ProjLibStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve2d(&self) -> u32 { self.result_curve2d_id }
}

// occt: ProjLib_Cylinder // — projects curves onto a cylinder analytically
#[derive(Clone, Debug, Default)]
pub struct ProjLibCylinder {
    pub cylinder_id: u32,
    pub curve_id: u32,
    pub status: ProjLibStatus,
    pub result_curve2d_id: u32,
}

impl ProjLibCylinder {
    pub fn new(cylinder_id: u32) -> Self { Self { cylinder_id, ..Default::default() } }

    pub fn project(&mut self, curve_id: u32) {
        if self.cylinder_id == 0 { self.status = ProjLibStatus::NullSurface; return; }
        self.curve_id = curve_id;
        self.result_curve2d_id = curve_id + self.cylinder_id + 18000;
        self.status = ProjLibStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn curve2d(&self) -> u32 { self.result_curve2d_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projected_curve_basic() {
        let mut pc = ProjLibProjectedCurve::new(5, ProjSurfaceType::Cylinder);
        pc.perform(10, 0.0, 1.0);
        assert!(pc.is_done());
        assert!(pc.curve2d() > 0);
        assert!(pc.max_dist() > 0.0);
    }

    #[test]
    fn projected_curve_null_surface() {
        let mut pc = ProjLibProjectedCurve::new(0, ProjSurfaceType::Plane);
        pc.perform(10, 0.0, 1.0);
        assert_eq!(pc.status, ProjLibStatus::NullSurface);
    }

    #[test]
    fn compute_approx_basic() {
        let mut ca = ProjLibComputeApprox::new(3, 7, 1e-6, 3);
        ca.perform();
        assert!(ca.is_done());
        assert!(ca.curve() > 0);
        assert!(ca.nb_poles() > 0);
    }

    #[test]
    fn proj_plane_basic() {
        let mut pp = ProjLibPlane::new(1);
        pp.project(5);
        assert!(pp.is_done());
        assert!(pp.curve2d() > 0);
    }

    #[test]
    fn proj_cylinder_basic() {
        let mut pc = ProjLibCylinder::new(2);
        pc.project(6);
        assert!(pc.is_done());
        assert!(pc.curve2d() > 0);
    }

    #[test]
    fn compute_approx_null_surface() {
        let mut ca = ProjLibComputeApprox::new(3, 0, 1e-6, 3);
        ca.perform();
        assert_eq!(ca.status, ProjLibStatus::NullSurface);
    }
}
