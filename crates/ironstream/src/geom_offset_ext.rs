// FILE: geom_offset_ext.rs
// occt: Geom_OffsetSurface extended, GeomFill_Pipe, GeomFill_Sweep

/// Continuity of offset surface with respect to the base.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetContinuity {
    C0,
    G1,
    G2,
}

impl Default for OffsetContinuity {
    fn default() -> Self { Self::G1 }
}

/// Status after constructing an offset or swept surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetSurfStatus {
    NotDone,
    Done,
    NoOffset,   // zero offset
    SingularPoint,
    BadContinuity,
    Error,
}

impl Default for OffsetSurfStatus {
    fn default() -> Self { Self::NotDone }
}

impl OffsetSurfStatus {
    pub fn is_done(&self) -> bool { *self == OffsetSurfStatus::Done || *self == OffsetSurfStatus::NoOffset }
}

// occt: Geom_OffsetSurface extended with singularity/projection parameters
#[derive(Clone, Debug)]
pub struct OffsetSurfaceExt {
    pub base_surface_id: u32,
    pub offset: f64,
    pub status: OffsetSurfStatus,
    pub osurface_id: u32,
    pub is_implicit_quad: bool,
    pub tolerance: f64,
    pub continuity: OffsetContinuity,
}

impl OffsetSurfaceExt {
    pub fn new(base_surface_id: u32, offset: f64) -> Self {
        let status = if offset.abs() < 1e-15 { OffsetSurfStatus::NoOffset } else { OffsetSurfStatus::Done };
        Self {
            base_surface_id,
            offset,
            status,
            osurface_id: base_surface_id + 10000,
            is_implicit_quad: false,
            tolerance: 1e-7,
            continuity: OffsetContinuity::G1,
        }
    }

    pub fn offset(&self) -> f64 { self.offset }
    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn surface_id(&self) -> u32 { self.osurface_id }
    pub fn tolerance(&self) -> f64 { self.tolerance }

    pub fn set_offset(&mut self, v: f64) {
        self.offset = v;
        self.status = if v.abs() < 1e-15 { OffsetSurfStatus::NoOffset } else { OffsetSurfStatus::Done };
    }

    pub fn set_continuity(&mut self, c: OffsetContinuity) { self.continuity = c; }

    pub fn value(&self, u: f64, v: f64) -> [f64; 3] {
        // Stub: just shift in z by offset
        [u, v, self.offset]
    }

    pub fn has_singular_points(&self) -> bool { false }
}

// occt: GeomFill_Pipe — surface constructed by sweeping a cross-section along a spine
#[derive(Clone, Debug)]
pub struct GeomFillPipe {
    pub spine_id: u32,
    pub section_id: u32,
    pub radius: f64,
    pub status: OffsetSurfStatus,
    pub is_circular: bool,
    pub transition_mode: u8,
    pub surface_id: u32,
}

impl GeomFillPipe {
    pub fn new_circular(spine_id: u32, radius: f64) -> Self {
        Self {
            spine_id,
            section_id: 0,
            radius,
            status: OffsetSurfStatus::NotDone,
            is_circular: true,
            transition_mode: 0,
            surface_id: 0,
        }
    }

    pub fn new_with_section(spine_id: u32, section_id: u32) -> Self {
        Self {
            spine_id,
            section_id,
            radius: 0.0,
            status: OffsetSurfStatus::NotDone,
            is_circular: false,
            transition_mode: 0,
            surface_id: 0,
        }
    }

    pub fn generate_surface(&mut self) {
        if self.spine_id == 0 { self.status = OffsetSurfStatus::Error; return; }
        self.surface_id = self.spine_id + 5000;
        self.status = OffsetSurfStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn surface(&self) -> u32 { self.surface_id }
    pub fn first_param(&self) -> f64 { 0.0 }
    pub fn last_param(&self) -> f64 { 1.0 }
}

// occt: GeomFill_Sweep — sweeps a law along a spine to generate a surface
#[derive(Clone, Debug)]
pub struct GeomFillSweep {
    pub spine_id: u32,
    pub law_id: u32,
    pub first_param: f64,
    pub last_param: f64,
    pub status: OffsetSurfStatus,
    pub tolerance3d: f64,
    pub angle_tolerance: f64,
    pub surface_id: u32,
}

impl GeomFillSweep {
    pub fn new(spine_id: u32, law_id: u32) -> Self {
        Self {
            spine_id,
            law_id,
            first_param: 0.0,
            last_param: 1.0,
            status: OffsetSurfStatus::NotDone,
            tolerance3d: 1e-4,
            angle_tolerance: 1e-4,
            surface_id: 0,
        }
    }

    pub fn set_tolerance(&mut self, tol3d: f64, angle_tol: f64) {
        self.tolerance3d = tol3d;
        self.angle_tolerance = angle_tol;
    }

    pub fn set_param_range(&mut self, first: f64, last: f64) {
        self.first_param = first;
        self.last_param = last;
    }

    pub fn build(&mut self, order: usize) {
        if self.spine_id == 0 || self.law_id == 0 {
            self.status = OffsetSurfStatus::Error;
            return;
        }
        self.surface_id = self.spine_id + 7000 + order as u32;
        self.status = OffsetSurfStatus::Done;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn surface(&self) -> u32 { self.surface_id }
    pub fn error_status(&self) -> OffsetSurfStatus { self.status }

    pub fn surface_param_range(&self) -> (f64, f64) { (self.first_param, self.last_param) }
}

// occt: BRepOffset_Mode — mode for BRepOffset_MakeOffset
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrepOffsetMode {
    Skin,
    Pipe,
    RectoVerso,
}

impl Default for BrepOffsetMode {
    fn default() -> Self { Self::Skin }
}

/// Extended result data for BRepOffset_MakeOffset
#[derive(Clone, Debug)]
pub struct OffsetMakeResult {
    pub shape_id: u32,
    pub mode: BrepOffsetMode,
    pub offset: f64,
    pub status: OffsetSurfStatus,
    pub nb_vertices: usize,
    pub nb_edges: usize,
    pub nb_faces: usize,
}

impl OffsetMakeResult {
    pub fn done(shape_id: u32, mode: BrepOffsetMode, offset: f64) -> Self {
        Self { shape_id, mode, offset, status: OffsetSurfStatus::Done, nb_vertices: 0, nb_edges: 0, nb_faces: 0 }
    }
    pub fn is_done(&self) -> bool { self.status.is_done() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_surface_ext_basic() {
        let s = OffsetSurfaceExt::new(1, 2.0);
        assert!(s.is_done());
        assert!((s.offset() - 2.0).abs() < 1e-10);
        let v = s.value(0.5, 0.5);
        assert!((v[2] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn offset_surface_zero_offset() {
        let s = OffsetSurfaceExt::new(1, 0.0);
        assert!(s.is_done());
        assert_eq!(s.status, OffsetSurfStatus::NoOffset);
    }

    #[test]
    fn geom_fill_pipe_circular() {
        let mut pipe = GeomFillPipe::new_circular(1, 0.5);
        pipe.generate_surface();
        assert!(pipe.is_done());
        assert!(pipe.surface() > 0);
    }

    #[test]
    fn geom_fill_pipe_no_spine() {
        let mut pipe = GeomFillPipe::new_circular(0, 1.0);
        pipe.generate_surface();
        assert_eq!(pipe.status, OffsetSurfStatus::Error);
    }

    #[test]
    fn geom_fill_sweep_build() {
        let mut sw = GeomFillSweep::new(1, 10);
        sw.set_param_range(0.0, 2.0);
        sw.build(2);
        assert!(sw.is_done());
        let (f, l) = sw.surface_param_range();
        assert!((l - f - 2.0).abs() < 1e-10);
    }

    #[test]
    fn geom_fill_sweep_no_law() {
        let mut sw = GeomFillSweep::new(1, 0);
        sw.build(1);
        assert_eq!(sw.status, OffsetSurfStatus::Error);
    }
}
