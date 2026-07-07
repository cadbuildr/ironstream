// FILE: std_prs_wf_deflection_surface.rs
// occt: StdPrs_WFDeflectionSurface

/// Adaptor interface for 3D surfaces
pub struct Adaptor3dSurface {
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
    is_periodic_u: bool,
    is_periodic_v: bool,
}

impl Adaptor3dSurface {
    /// Creates a new 3D surface adaptor
    pub fn new(u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        Adaptor3dSurface {
            u_min,
            u_max,
            v_min,
            v_max,
            is_periodic_u: false,
            is_periodic_v: false,
        }
    }

    /// Returns U parameter range
    pub fn u_range(&self) -> (f64, f64) {
        (self.u_min, self.u_max)
    }

    /// Returns V parameter range
    pub fn v_range(&self) -> (f64, f64) {
        (self.v_min, self.v_max)
    }

    /// Sets U as periodic
    pub fn set_periodic_u(&mut self, periodic: bool) {
        self.is_periodic_u = periodic;
    }

    /// Sets V as periodic
    pub fn set_periodic_v(&mut self, periodic: bool) {
        self.is_periodic_v = periodic;
    }

    /// Returns true if U is periodic
    pub fn is_u_periodic(&self) -> bool {
        self.is_periodic_u
    }

    /// Returns true if V is periodic
    pub fn is_v_periodic(&self) -> bool {
        self.is_periodic_v
    }
}

/// Presentation drawer
pub struct Prs3dDrawer {
    nb_u_iso: i32,
    nb_v_iso: i32,
    deflection: f64,
}

impl Prs3dDrawer {
    /// Creates a new drawer
    pub fn new() -> Self {
        Prs3dDrawer {
            nb_u_iso: 10,
            nb_v_iso: 10,
            deflection: 0.1,
        }
    }

    /// Sets number of U isoparameters
    pub fn set_nb_u_iso(&mut self, count: i32) {
        self.nb_u_iso = count;
    }

    /// Sets number of V isoparameters
    pub fn set_nb_v_iso(&mut self, count: i32) {
        self.nb_v_iso = count;
    }

    /// Sets deflection value
    pub fn set_deflection(&mut self, defl: f64) {
        self.deflection = defl;
    }

    /// Gets number of U isoparameters
    pub fn nb_u_iso(&self) -> i32 {
        self.nb_u_iso
    }

    /// Gets number of V isoparameters
    pub fn nb_v_iso(&self) -> i32 {
        self.nb_v_iso
    }

    /// Gets deflection value
    pub fn deflection(&self) -> f64 {
        self.deflection
    }
}

impl Default for Prs3dDrawer {
    fn default() -> Self {
        Self::new()
    }
}

/// Wireframe presentation of surface with isoparametric curves
pub struct StdPrsWfDeflectionSurface;

impl StdPrsWfDeflectionSurface {
    /// Adds surface wireframe presentation with isoparameters and boundaries
    pub fn add(
        _presentation: usize,
        surface: &Adaptor3dSurface,
        drawer: &Prs3dDrawer,
    ) {
        // In a real implementation, this would:
        // 1. Extract the bounds from the surface
        // 2. Create isoparametric curves based on drawer settings
        // 3. Add boundaries
        // 4. Add curves to the presentation with appropriate color and style

        // The surface parameter ranges are available
        let (_u_min, _u_max) = surface.u_range();
        let (_v_min, _v_max) = surface.v_range();

        // Drawer settings control the number of iso curves
        let _nb_u = drawer.nb_u_iso();
        let _nb_v = drawer.nb_v_iso();
        let _defl = drawer.deflection();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptor_3d_surface_ranges() {
        let surf = Adaptor3dSurface::new(0.0, 1.0, 0.0, 2.0);
        assert_eq!(surf.u_range(), (0.0, 1.0));
        assert_eq!(surf.v_range(), (0.0, 2.0));
    }

    #[test]
    fn test_adaptor_3d_surface_periodicity() {
        let mut surf = Adaptor3dSurface::new(0.0, 1.0, 0.0, 1.0);
        assert!(!surf.is_u_periodic());
        assert!(!surf.is_v_periodic());

        surf.set_periodic_u(true);
        surf.set_periodic_v(true);

        assert!(surf.is_u_periodic());
        assert!(surf.is_v_periodic());
    }

    #[test]
    fn test_prs3d_drawer_defaults() {
        let drawer = Prs3dDrawer::new();
        assert_eq!(drawer.nb_u_iso(), 10);
        assert_eq!(drawer.nb_v_iso(), 10);
        assert_eq!(drawer.deflection(), 0.1);
    }

    #[test]
    fn test_prs3d_drawer_setters() {
        let mut drawer = Prs3dDrawer::new();
        drawer.set_nb_u_iso(20);
        drawer.set_nb_v_iso(15);
        drawer.set_deflection(0.05);

        assert_eq!(drawer.nb_u_iso(), 20);
        assert_eq!(drawer.nb_v_iso(), 15);
        assert!((drawer.deflection() - 0.05).abs() < 1e-9);
    }

    #[test]
    fn test_add_wireframe() {
        let surf = Adaptor3dSurface::new(0.0, 1.0, 0.0, 1.0);
        let drawer = Prs3dDrawer::new();
        // Should not panic
        StdPrsWfDeflectionSurface::add(0, &surf, &drawer);
    }

    #[test]
    fn test_add_with_custom_isos() {
        let surf = Adaptor3dSurface::new(0.0, 10.0, 0.0, 20.0);
        let mut drawer = Prs3dDrawer::new();
        drawer.set_nb_u_iso(15);
        drawer.set_nb_v_iso(25);
        drawer.set_deflection(0.01);

        StdPrsWfDeflectionSurface::add(0, &surf, &drawer);
    }

    #[test]
    fn test_add_periodic_surface() {
        let mut surf = Adaptor3dSurface::new(0.0, 2.0 * std::f64::consts::PI, 0.0, 1.0);
        surf.set_periodic_u(true);

        let drawer = Prs3dDrawer::new();
        StdPrsWfDeflectionSurface::add(0, &surf, &drawer);
    }
}
