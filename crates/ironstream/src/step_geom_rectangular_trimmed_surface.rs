// FILE: step_geom_rectangular_trimmed_surface.rs
// occt: StepGeom_RectangularTrimmedSurface

/// Represents a surface trimmed to a rectangular region in parametric space
pub struct StepGeomRectangularTrimmedSurface {
    name: String,
    base_surface_id: i32,
    u_min: f64,
    u_max: f64,
    v_min: f64,
    v_max: f64,
}

impl StepGeomRectangularTrimmedSurface {
    pub fn new(
        name: String,
        base_surface_id: i32,
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
    ) -> Self {
        StepGeomRectangularTrimmedSurface {
            name,
            base_surface_id,
            u_min,
            u_max,
            v_min,
            v_max,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn base_surface_id(&self) -> i32 {
        self.base_surface_id
    }

    pub fn u_min(&self) -> f64 {
        self.u_min
    }

    pub fn u_max(&self) -> f64 {
        self.u_max
    }

    pub fn v_min(&self) -> f64 {
        self.v_min
    }

    pub fn v_max(&self) -> f64 {
        self.v_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_trimmed_surface() {
        let surface = StepGeomRectangularTrimmedSurface::new(
            "TrimmedSurface1".to_string(),
            1,
            0.0,
            1.0,
            0.0,
            1.0,
        );
        assert_eq!(surface.name(), "TrimmedSurface1");
        assert_eq!(surface.base_surface_id(), 1);
        assert_eq!(surface.u_min(), 0.0);
        assert_eq!(surface.u_max(), 1.0);
    }
}
