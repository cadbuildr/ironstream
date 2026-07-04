// FILE: step_geom_uniform_surface_and_rational_b_spline_surface.rs
// occt: StepGeom_UniformSurfaceAndRationalBSplineSurface

pub struct UniformSurfaceAndRationalBSplineSurface {
    uniform_surface: Option<Box<dyn std::any::Any>>,
    rational_b_spline_surface: Option<Box<dyn std::any::Any>>,
}

impl UniformSurfaceAndRationalBSplineSurface {
    pub fn new() -> Self {
        UniformSurfaceAndRationalBSplineSurface {
            uniform_surface: None,
            rational_b_spline_surface: None,
        }
    }

    pub fn set_uniform_surface(&mut self, surface: Option<Box<dyn std::any::Any>>) {
        self.uniform_surface = surface;
    }

    pub fn uniform_surface(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.uniform_surface
    }

    pub fn set_rational_b_spline_surface(&mut self, surface: Option<Box<dyn std::any::Any>>) {
        self.rational_b_spline_surface = surface;
    }

    pub fn rational_b_spline_surface(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.rational_b_spline_surface
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_surface_and_rational_b_spline_surface_creation() {
        let surface = UniformSurfaceAndRationalBSplineSurface::new();
        assert!(surface.uniform_surface().is_none());
        assert!(surface.rational_b_spline_surface().is_none());
    }
}
