// FILE: poly_triangulation_parameters.rs
// occt: Poly_TriangulationParameters

/// Parameters for triangulation algorithms.
#[derive(Debug, Clone)]
pub struct TriangulationParameters {
    pub deflection: f64,           // Maximum deflection
    pub angle_deflection: f64,     // Angle deflection in radians
    pub min_segment_size: f64,     // Minimum segment size
    pub use_mesh_params: bool,     // Use specific mesh parameters
}

impl TriangulationParameters {
    /// Create default parameters.
    pub fn new() -> Self {
        TriangulationParameters {
            deflection: 0.1,
            angle_deflection: 0.1,
            min_segment_size: 0.0,
            use_mesh_params: false,
        }
    }

    /// Set linear deflection.
    pub fn set_deflection(&mut self, d: f64) {
        self.deflection = d.max(1e-15);
    }

    /// Get linear deflection.
    pub fn get_deflection(&self) -> f64 {
        self.deflection
    }

    /// Set angle deflection.
    pub fn set_angle_deflection(&mut self, angle: f64) {
        self.angle_deflection = angle;
    }

    /// Get angle deflection.
    pub fn get_angle_deflection(&self) -> f64 {
        self.angle_deflection
    }

    /// Set minimum segment size.
    pub fn set_min_segment_size(&mut self, size: f64) {
        self.min_segment_size = size.max(0.0);
    }

    /// Get minimum segment size.
    pub fn get_min_segment_size(&self) -> f64 {
        self.min_segment_size
    }

    /// Set whether to use mesh-specific parameters.
    pub fn set_use_mesh_params(&mut self, use_params: bool) {
        self.use_mesh_params = use_params;
    }

    /// Check if using mesh parameters.
    pub fn is_use_mesh_params(&self) -> bool {
        self.use_mesh_params
    }
}

impl Default for TriangulationParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulation_parameters_default() {
        let params = TriangulationParameters::new();
        assert!(params.deflection > 0.0);
        assert!(params.angle_deflection > 0.0);
        assert!(!params.use_mesh_params);
    }

    #[test]
    fn test_set_deflection() {
        let mut params = TriangulationParameters::new();
        params.set_deflection(0.05);
        assert!((params.get_deflection() - 0.05).abs() < 1e-10);
    }

    #[test]
    fn test_set_angle_deflection() {
        let mut params = TriangulationParameters::new();
        params.set_angle_deflection(0.2);
        assert!((params.get_angle_deflection() - 0.2).abs() < 1e-10);
    }

    #[test]
    fn test_set_min_segment_size() {
        let mut params = TriangulationParameters::new();
        params.set_min_segment_size(0.01);
        assert!((params.get_min_segment_size() - 0.01).abs() < 1e-10);
    }

    #[test]
    fn test_set_use_mesh_params() {
        let mut params = TriangulationParameters::new();
        params.set_use_mesh_params(true);
        assert!(params.is_use_mesh_params());
    }

    #[test]
    fn test_negative_deflection_clamped() {
        let mut params = TriangulationParameters::new();
        params.set_deflection(-0.1);
        assert!(params.get_deflection() > 0.0);
    }
}
