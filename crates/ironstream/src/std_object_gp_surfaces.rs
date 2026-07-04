// FILE: std_object_gp_surfaces.rs
// occt: StdObject_gp_Surfaces

/// Persistent representation of geometric surfaces
pub struct GpSurface {
    surface_type: i32,
    params: Vec<f64>,
}

impl GpSurface {
    /// Create a new surface
    pub fn new(surface_type: i32) -> Self {
        GpSurface {
            surface_type,
            params: Vec::new(),
        }
    }

    /// Get surface type
    pub fn surface_type(&self) -> i32 {
        self.surface_type
    }

    /// Get parameters
    pub fn params(&self) -> &[f64] {
        &self.params
    }

    /// Set parameters
    pub fn set_params(&mut self, params: Vec<f64>) {
        self.params = params;
    }

    /// Add a parameter
    pub fn add_param(&mut self, param: f64) {
        self.params.push(param);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let surface = GpSurface::new(1);
        assert_eq!(surface.surface_type(), 1);
        assert!(surface.params().is_empty());
    }

    #[test]
    fn test_add_param() {
        let mut surface = GpSurface::new(1);
        surface.add_param(1.5);
        surface.add_param(2.5);

        assert_eq!(surface.params().len(), 2);
    }
}
