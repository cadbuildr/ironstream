// FILE: top_ope_b_rep_ds_surface_data.rs
// occt: TopOpeBRepDS_SurfaceData

/// Geometry data base structure
pub struct TopOpeBRepDSGeometryData {
    /// List of interferences
    interferences: Vec<i32>,
}

impl TopOpeBRepDSGeometryData {
    pub fn new() -> Self {
        TopOpeBRepDSGeometryData {
            interferences: Vec::new(),
        }
    }

    pub fn add_interference(&mut self, id: i32) {
        self.interferences.push(id);
    }

    pub fn interferences(&self) -> &[i32] {
        &self.interferences
    }
}

impl Default for TopOpeBRepDSGeometryData {
    fn default() -> Self {
        Self::new()
    }
}

/// Surface data containing a surface and geometry data
pub struct TopOpeBRepDSSurfaceData {
    surface: TopOpeBRepDSSurface,
    geometry_data: TopOpeBRepDSGeometryData,
}

/// Simplified surface definition for data structure
#[derive(Clone, Debug)]
pub struct TopOpeBRepDSSurface {
    tolerance: f64,
    keep: bool,
}

impl TopOpeBRepDSSurface {
    pub fn new() -> Self {
        TopOpeBRepDSSurface {
            tolerance: 0.0,
            keep: true,
        }
    }

    pub fn with_tolerance(tolerance: f64) -> Self {
        TopOpeBRepDSSurface {
            tolerance,
            keep: true,
        }
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    pub fn keep(&self) -> bool {
        self.keep
    }

    pub fn set_keep(&mut self, keep: bool) {
        self.keep = keep;
    }
}

impl Default for TopOpeBRepDSSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl TopOpeBRepDSSurfaceData {
    /// Create a new empty surface data
    pub fn new() -> Self {
        TopOpeBRepDSSurfaceData {
            surface: TopOpeBRepDSSurface::new(),
            geometry_data: TopOpeBRepDSGeometryData::new(),
        }
    }

    /// Create surface data from a surface
    pub fn from_surface(surface: TopOpeBRepDSSurface) -> Self {
        TopOpeBRepDSSurfaceData {
            surface,
            geometry_data: TopOpeBRepDSGeometryData::new(),
        }
    }

    /// Get the surface
    pub fn surface(&self) -> &TopOpeBRepDSSurface {
        &self.surface
    }

    /// Get mutable reference to surface
    pub fn surface_mut(&mut self) -> &mut TopOpeBRepDSSurface {
        &mut self.surface
    }

    /// Get the geometry data
    pub fn geometry_data(&self) -> &TopOpeBRepDSGeometryData {
        &self.geometry_data
    }

    /// Get mutable reference to geometry data
    pub fn geometry_data_mut(&mut self) -> &mut TopOpeBRepDSGeometryData {
        &mut self.geometry_data
    }
}

impl Default for TopOpeBRepDSSurfaceData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_surface_data() {
        let data = TopOpeBRepDSSurfaceData::new();
        assert_eq!(data.surface().tolerance(), 0.0);
    }

    #[test]
    fn test_from_surface() {
        let surface = TopOpeBRepDSSurface::with_tolerance(0.01);
        let data = TopOpeBRepDSSurfaceData::from_surface(surface);
        assert_eq!(data.surface().tolerance(), 0.01);
    }

    #[test]
    fn test_surface_mut() {
        let mut data = TopOpeBRepDSSurfaceData::new();
        data.surface_mut().set_tolerance(0.005);
        assert_eq!(data.surface().tolerance(), 0.005);
    }

    #[test]
    fn test_geometry_data() {
        let mut data = TopOpeBRepDSSurfaceData::new();
        data.geometry_data_mut().add_interference(1);
        data.geometry_data_mut().add_interference(2);
        assert_eq!(data.geometry_data().interferences().len(), 2);
    }

    #[test]
    fn test_geometry_data_empty() {
        let data = TopOpeBRepDSSurfaceData::new();
        assert_eq!(data.geometry_data().interferences().len(), 0);
    }

    #[test]
    fn test_surface_keep() {
        let mut data = TopOpeBRepDSSurfaceData::new();
        assert!(data.surface().keep());
        data.surface_mut().set_keep(false);
        assert!(!data.surface().keep());
    }

    #[test]
    fn test_default() {
        let data = TopOpeBRepDSSurfaceData::default();
        assert_eq!(data.surface().tolerance(), 0.0);
    }
}
