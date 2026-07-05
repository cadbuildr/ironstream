// FILE: db_rep_params.rs
// occt: DBRep_Params

//! DBRep parameters for controlling display of shapes.

use std::f64::consts::PI;

/// DBRep_Params: Parameters for shape display control.
#[derive(Clone, Debug, PartialEq)]
pub struct DbrepParams {
    /// Number of isoparametric curves in U and V
    pub nb_isos: i32,
    /// Size parameter for display
    pub size: f64,
    /// Discretization number of points for curves
    pub discretization: i32,
    /// Display triangulations
    pub disp_triangles: bool,
    /// Display polygons
    pub display_polygons: bool,
    /// Discretization angle for edges (in radians)
    pub hlr_angle: f64,
    /// Minimum hidden line angle
    pub h_ang_min: f64,
    /// Maximum hidden line angle
    pub h_ang_max: f64,
    /// True if HLR (Hidden Line Removal), False if wireframe
    pub with_hlr: bool,
    /// True if display Rg1 lines
    pub with_rg1: bool,
    /// True if display RgN lines
    pub with_rgn: bool,
    /// True if display hidden lines
    pub with_hid: bool,
}

impl Default for DbrepParams {
    fn default() -> Self {
        DbrepParams {
            nb_isos: 2,
            size: 100.0,
            discretization: 30,
            disp_triangles: false,
            display_polygons: false,
            hlr_angle: 35.0 * PI / 180.0,
            h_ang_min: 1.0 * PI / 180.0,
            h_ang_max: 35.0 * PI / 180.0,
            with_hlr: false,
            with_rg1: true,
            with_rgn: false,
            with_hid: false,
        }
    }
}

impl DbrepParams {
    /// Create new parameters with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of isoparametric curves.
    pub fn set_nb_isos(&mut self, nb: i32) {
        self.nb_isos = nb;
    }

    /// Set the size parameter.
    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    /// Set the discretization number.
    pub fn set_discretization(&mut self, disc: i32) {
        self.discretization = disc;
    }

    /// Enable or disable triangle display.
    pub fn set_disp_triangles(&mut self, enable: bool) {
        self.disp_triangles = enable;
    }

    /// Enable or disable polygon display.
    pub fn set_display_polygons(&mut self, enable: bool) {
        self.display_polygons = enable;
    }

    /// Set the HLR discretization angle (in radians).
    pub fn set_hlr_angle(&mut self, angle: f64) {
        self.hlr_angle = angle;
    }

    /// Set the HLR discretization angle from degrees.
    pub fn set_hlr_angle_degrees(&mut self, angle_deg: f64) {
        self.hlr_angle = angle_deg * PI / 180.0;
    }

    /// Set the minimum hidden line angle (in radians).
    pub fn set_h_ang_min(&mut self, angle: f64) {
        self.h_ang_min = angle;
    }

    /// Set the maximum hidden line angle (in radians).
    pub fn set_h_ang_max(&mut self, angle: f64) {
        self.h_ang_max = angle;
    }

    /// Enable or disable HLR mode.
    pub fn set_with_hlr(&mut self, enable: bool) {
        self.with_hlr = enable;
    }

    /// Enable or disable Rg1 line display.
    pub fn set_with_rg1(&mut self, enable: bool) {
        self.with_rg1 = enable;
    }

    /// Enable or disable RgN line display.
    pub fn set_with_rgn(&mut self, enable: bool) {
        self.with_rgn = enable;
    }

    /// Enable or disable hidden line display.
    pub fn set_with_hid(&mut self, enable: bool) {
        self.with_hid = enable;
    }

    /// Get HLR angle in degrees.
    pub fn hlr_angle_degrees(&self) -> f64 {
        self.hlr_angle * 180.0 / PI
    }

    /// Get minimum HLR angle in degrees.
    pub fn h_ang_min_degrees(&self) -> f64 {
        self.h_ang_min * 180.0 / PI
    }

    /// Get maximum HLR angle in degrees.
    pub fn h_ang_max_degrees(&self) -> f64 {
        self.h_ang_max * 180.0 / PI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_params() {
        let params = DbrepParams::default();
        assert_eq!(params.nb_isos, 2);
        assert_eq!(params.size, 100.0);
        assert_eq!(params.discretization, 30);
        assert!(!params.disp_triangles);
        assert!(!params.display_polygons);
        assert!(!params.with_hlr);
        assert!(params.with_rg1);
        assert!(!params.with_rgn);
        assert!(!params.with_hid);
    }

    #[test]
    fn test_hlr_angle_conversion() {
        let mut params = DbrepParams::new();
        params.set_hlr_angle_degrees(45.0);

        let radians = params.hlr_angle;
        let degrees = params.hlr_angle_degrees();

        assert!((radians - PI / 4.0).abs() < 0.0001);
        assert!((degrees - 45.0).abs() < 0.01);
    }

    #[test]
    fn test_set_nb_isos() {
        let mut params = DbrepParams::new();
        params.set_nb_isos(10);
        assert_eq!(params.nb_isos, 10);
    }

    #[test]
    fn test_set_size() {
        let mut params = DbrepParams::new();
        params.set_size(50.0);
        assert_eq!(params.size, 50.0);
    }

    #[test]
    fn test_set_discretization() {
        let mut params = DbrepParams::new();
        params.set_discretization(50);
        assert_eq!(params.discretization, 50);
    }

    #[test]
    fn test_triangle_display() {
        let mut params = DbrepParams::new();
        assert!(!params.disp_triangles);

        params.set_disp_triangles(true);
        assert!(params.disp_triangles);

        params.set_disp_triangles(false);
        assert!(!params.disp_triangles);
    }

    #[test]
    fn test_polygon_display() {
        let mut params = DbrepParams::new();
        assert!(!params.display_polygons);

        params.set_display_polygons(true);
        assert!(params.display_polygons);
    }

    #[test]
    fn test_hlr_modes() {
        let mut params = DbrepParams::new();

        params.set_with_hlr(true);
        assert!(params.with_hlr);

        params.set_with_rg1(false);
        assert!(!params.with_rg1);

        params.set_with_rgn(true);
        assert!(params.with_rgn);

        params.set_with_hid(true);
        assert!(params.with_hid);
    }

    #[test]
    fn test_angle_ranges() {
        let mut params = DbrepParams::new();

        params.set_h_ang_min(5.0 * PI / 180.0);
        params.set_h_ang_max(60.0 * PI / 180.0);

        assert!((params.h_ang_min_degrees() - 5.0).abs() < 0.01);
        assert!((params.h_ang_max_degrees() - 60.0).abs() < 0.01);
    }

    #[test]
    fn test_equality() {
        let params1 = DbrepParams::new();
        let params2 = DbrepParams::new();

        assert_eq!(params1, params2);
    }

    #[test]
    fn test_clone() {
        let mut params1 = DbrepParams::new();
        params1.set_nb_isos(15);

        let params2 = params1.clone();
        assert_eq!(params2.nb_isos, 15);
    }

    #[test]
    fn test_comprehensive_setup() {
        let mut params = DbrepParams::new();

        params.set_nb_isos(20);
        params.set_size(200.0);
        params.set_discretization(100);
        params.set_disp_triangles(true);
        params.set_display_polygons(true);
        params.set_hlr_angle_degrees(30.0);
        params.set_with_hlr(true);
        params.set_with_rg1(false);
        params.set_with_rgn(true);
        params.set_with_hid(true);

        assert_eq!(params.nb_isos, 20);
        assert_eq!(params.size, 200.0);
        assert_eq!(params.discretization, 100);
        assert!(params.disp_triangles);
        assert!(params.display_polygons);
        assert!((params.hlr_angle_degrees() - 30.0).abs() < 0.01);
        assert!(params.with_hlr);
        assert!(!params.with_rg1);
        assert!(params.with_rgn);
        assert!(params.with_hid);
    }
}
