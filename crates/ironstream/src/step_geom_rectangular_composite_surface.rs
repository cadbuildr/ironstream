// FILE: step_geom_rectangular_composite_surface.rs
// occt: StepGeom_RectangularCompositeSurface

/// Represents a rectangular composite surface (grid of surface patches)
pub struct StepGeomRectangularCompositeSurface {
    name: String,
    nb_surfaces_u: i32,
    nb_surfaces_v: i32,
    /// Surface patch IDs in grid order
    patches: Vec<i32>,
}

impl StepGeomRectangularCompositeSurface {
    pub fn new(name: String, nb_u: i32, nb_v: i32) -> Self {
        StepGeomRectangularCompositeSurface {
            name,
            nb_surfaces_u: nb_u,
            nb_surfaces_v: nb_v,
            patches: Vec::with_capacity((nb_u * nb_v) as usize),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn nb_surfaces_u(&self) -> i32 {
        self.nb_surfaces_u
    }

    pub fn nb_surfaces_v(&self) -> i32 {
        self.nb_surfaces_v
    }

    pub fn add_patch(&mut self, patch_id: i32) {
        self.patches.push(patch_id);
    }

    pub fn patches(&self) -> &[i32] {
        &self.patches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_composite_surface() {
        let surface = StepGeomRectangularCompositeSurface::new("CompositeSurface1".to_string(), 2, 3);
        assert_eq!(surface.name(), "CompositeSurface1");
        assert_eq!(surface.nb_surfaces_u(), 2);
        assert_eq!(surface.nb_surfaces_v(), 3);
    }

    #[test]
    fn test_add_patches() {
        let mut surface =
            StepGeomRectangularCompositeSurface::new("CompositeSurface1".to_string(), 2, 2);
        surface.add_patch(1);
        surface.add_patch(2);
        surface.add_patch(3);
        surface.add_patch(4);
        assert_eq!(surface.patches().len(), 4);
    }
}
