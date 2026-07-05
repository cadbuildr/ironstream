// FILE: draw_tr_surf_params.rs
// occt: DrawTrSurf_Params

//! Display parameters for DrawTrSurf objects.

#[derive(Clone, Debug, Default)]
pub struct DrawTrSurfParams {
    pub u_samples: usize,
    pub v_samples: usize,
    pub discretization_mode: DiscretizationMode,
    pub show_iso_u: bool,
    pub show_iso_v: bool,
    pub iso_u_count: usize,
    pub iso_v_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscretizationMode {
    Parametric,
    Adaptive,
    Uniform,
}

impl Default for DiscretizationMode {
    fn default() -> Self {
        DiscretizationMode::Parametric
    }
}

impl DrawTrSurfParams {
    pub fn new() -> Self {
        Self {
            u_samples: 50,
            v_samples: 50,
            discretization_mode: DiscretizationMode::Parametric,
            show_iso_u: true,
            show_iso_v: true,
            iso_u_count: 10,
            iso_v_count: 10,
        }
    }

    pub fn set_u_samples(&mut self, count: usize) {
        self.u_samples = count;
    }

    pub fn set_v_samples(&mut self, count: usize) {
        self.v_samples = count;
    }

    pub fn set_discretization_mode(&mut self, mode: DiscretizationMode) {
        self.discretization_mode = mode;
    }

    pub fn set_iso_display(&mut self, show_u: bool, show_v: bool) {
        self.show_iso_u = show_u;
        self.show_iso_v = show_v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let params = DrawTrSurfParams::new();
        assert_eq!(params.u_samples, 50);
        assert_eq!(params.v_samples, 50);
        assert!(params.show_iso_u);
        assert!(params.show_iso_v);
    }

    #[test]
    fn test_set_samples() {
        let mut params = DrawTrSurfParams::new();
        params.set_u_samples(100);
        params.set_v_samples(150);

        assert_eq!(params.u_samples, 100);
        assert_eq!(params.v_samples, 150);
    }

    #[test]
    fn test_set_discretization_mode() {
        let mut params = DrawTrSurfParams::new();
        params.set_discretization_mode(DiscretizationMode::Adaptive);

        assert_eq!(params.discretization_mode, DiscretizationMode::Adaptive);
    }

    #[test]
    fn test_iso_display() {
        let mut params = DrawTrSurfParams::new();
        params.set_iso_display(false, true);

        assert!(!params.show_iso_u);
        assert!(params.show_iso_v);
    }
}
