// FILE: iges_geom_bounded_surface_o.rs
// occt: IGESGeom_BoundedSurface

use std::collections::HashMap;

pub struct IgesGeomBoundedSurface {
    representation_type: i32,
    surface: Option<Box<dyn std::any::Any>>,
    boundaries: Vec<Option<Box<dyn std::any::Any>>>,
}

impl IgesGeomBoundedSurface {
    pub fn new() -> Self {
        IgesGeomBoundedSurface {
            representation_type: 0,
            surface: None,
            boundaries: Vec::new(),
        }
    }

    pub fn init(&mut self, rep_type: i32, surface: Option<Box<dyn std::any::Any>>, boundaries: Vec<Option<Box<dyn std::any::Any>>>) {
        self.representation_type = rep_type;
        self.surface = surface;
        self.boundaries = boundaries;
    }

    pub fn representation_type(&self) -> i32 {
        self.representation_type
    }

    pub fn surface(&self) -> Option<&Box<dyn std::any::Any>> {
        self.surface.as_ref()
    }

    pub fn nb_boundaries(&self) -> usize {
        self.boundaries.len()
    }

    pub fn boundary(&self, index: usize) -> Option<&Box<dyn std::any::Any>> {
        if index > 0 && index <= self.boundaries.len() {
            self.boundaries[index - 1].as_ref()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_surface_creation() {
        let mut bs = IgesGeomBoundedSurface::new();
        assert_eq!(bs.representation_type(), 0);
        assert_eq!(bs.nb_boundaries(), 0);
    }

    #[test]
    fn test_bounded_surface_init() {
        let mut bs = IgesGeomBoundedSurface::new();
        let boundaries: Vec<Option<Box<dyn std::any::Any>>> = vec![
            Some(Box::new(1)),
            Some(Box::new(2)),
        ];
        bs.init(1, None, boundaries);

        assert_eq!(bs.representation_type(), 1);
        assert_eq!(bs.nb_boundaries(), 2);
    }

    #[test]
    fn test_boundary_access() {
        let mut bs = IgesGeomBoundedSurface::new();
        let boundaries: Vec<Option<Box<dyn std::any::Any>>> = vec![
            Some(Box::new(1)),
        ];
        bs.init(0, None, boundaries);

        assert!(bs.boundary(1).is_some());
        assert!(bs.boundary(0).is_none());
        assert!(bs.boundary(2).is_none());
    }
}
