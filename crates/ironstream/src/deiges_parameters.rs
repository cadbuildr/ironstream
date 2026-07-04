// FILE: deiges_parameters.rs
// occt: DEIGES_Parameters

/// Parameters for IGES file read and write operations.
pub struct Parameters {
    pub read_bspline_continuity: ReadModeBSplineContinuity,
    pub read_precision_mode: ReadModePrecision,
    pub read_precision_val: f64,
    pub read_max_precision_val: f64,
    pub read_color: bool,
    pub read_name: bool,
    pub read_layer: bool,
    pub write_brep_mode: WriteModeBRep,
    pub write_convert_surface_mode: WriteModeConvertSurface,
    pub write_precision_mode: WriteModePrecisionMode,
    pub write_precision_val: f64,
    pub write_plane_mode: WriteModeePlaneMode,
    pub write_color: bool,
    pub write_name: bool,
    pub write_layer: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadModeBSplineContinuity {
    C0,
    C1,
    C2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadModePrecision {
    File,
    User,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteModeBRep {
    Faces,
    BRep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteModeConvertSurface {
    Off,
    On,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteModePrecisionMode {
    Least,
    Average,
    Greatest,
    Session,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteModeePlaneMode {
    Plane,
    BSpline,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            read_bspline_continuity: ReadModeBSplineContinuity::C1,
            read_precision_mode: ReadModePrecision::File,
            read_precision_val: 0.0001,
            read_max_precision_val: 1.0,
            read_color: true,
            read_name: true,
            read_layer: true,
            write_brep_mode: WriteModeBRep::Faces,
            write_convert_surface_mode: WriteModeConvertSurface::Off,
            write_precision_mode: WriteModePrecisionMode::Average,
            write_precision_val: 0.0001,
            write_plane_mode: WriteModeePlaneMode::Plane,
            write_color: true,
            write_name: true,
            write_layer: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let params = Parameters::default();
        assert!(params.read_color);
        assert_eq!(params.read_precision_val, 0.0001);
    }
}
