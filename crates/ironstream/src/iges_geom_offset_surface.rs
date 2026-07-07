// FILE: iges_geom_offset_surface.rs
// occt: IGESGeom_OffsetSurface

/// Defines IGESOffsetSurface, Type <140> Form <0> in package IGESGeom.
/// An offset surface is a surface defined in terms of an already existing surface.
/// If S(u, v) is a parametrized regular surface and N(u, v) is a differential field
/// of unit normal vectors, then the offset surface is O(u, v) = S(u, v) + d * N(u, v).
#[derive(Clone, Debug)]
pub struct OffsetSurface {
    /// Offset indicator vector
    indicator: [f64; 3],
    /// Offset distance
    distance: f64,
    /// Surface entity ID that is offset
    surface_id: Option<i32>,
    /// Entity type for IGES (always 140)
    entity_type: i32,
}

impl OffsetSurface {
    /// Creates a new OffsetSurface entity.
    pub fn new() -> Self {
        OffsetSurface {
            indicator: [0.0, 0.0, 0.0],
            distance: 0.0,
            surface_id: None,
            entity_type: 140,
        }
    }

    /// Initializes the OffsetSurface with indicator, distance, and base surface.
    pub fn init(&mut self, indicator: [f64; 3], distance: f64, surface: Option<i32>) {
        self.indicator = indicator;
        self.distance = distance;
        self.surface_id = surface;
    }

    /// Returns the offset indicator vector.
    pub fn offset_indicator(&self) -> [f64; 3] {
        self.indicator
    }

    /// Returns the offset indicator after transformation.
    pub fn transformed_offset_indicator(&self) -> [f64; 3] {
        // TODO: Apply transformation matrix if present
        self.indicator
    }

    /// Returns the offset distance.
    pub fn distance(&self) -> f64 {
        self.distance
    }

    /// Returns the surface entity ID that has been offset.
    pub fn surface(&self) -> Option<i32> {
        self.surface_id
    }

    /// Returns the entity type number (always 140).
    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for OffsetSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_offset_surface() {
        let surface = OffsetSurface::new();
        assert_eq!(surface.offset_indicator(), [0.0, 0.0, 0.0]);
        assert_eq!(surface.distance(), 0.0);
        assert_eq!(surface.surface(), None);
        assert_eq!(surface.entity_type(), 140);
    }

    #[test]
    fn test_init_offset_surface() {
        let mut surface = OffsetSurface::new();
        surface.init([1.0, 0.0, 0.0], 5.0, Some(1));

        assert_eq!(surface.offset_indicator(), [1.0, 0.0, 0.0]);
        assert_eq!(surface.distance(), 5.0);
        assert_eq!(surface.surface(), Some(1));
    }

    #[test]
    fn test_transformed_offset_indicator() {
        let mut surface = OffsetSurface::new();
        surface.init([0.0, 1.0, 0.0], 2.5, Some(2));

        // Without transformation, transformed equals original
        assert_eq!(surface.transformed_offset_indicator(), [0.0, 1.0, 0.0]);
    }
}
