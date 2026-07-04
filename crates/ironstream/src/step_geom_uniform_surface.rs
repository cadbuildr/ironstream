// FILE: step_geom_uniform_surface.rs
// occt: StepGeom_UniformSurface

pub struct UniformSurface;

impl UniformSurface {
    pub fn new() -> Self {
        UniformSurface
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_surface_creation() {
        let _surface = UniformSurface::new();
    }
}
