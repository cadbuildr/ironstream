// FILE: step_geom_intersection_curve.rs
// occt: StepGeom_IntersectionCurve

//! Represents an intersection curve between two surfaces.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Surface {
    id: String,
}

impl Surface {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone)]
pub struct Curve {
    id: String,
}

impl Curve {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone)]
pub struct StepGeomIntersectionCurve {
    name: Option<String>,
    curve_3d: Option<Rc<Curve>>,
    surface_1: Option<Rc<Surface>>,
    surface_2: Option<Rc<Surface>>,
}

impl StepGeomIntersectionCurve {
    pub fn new() -> Self {
        Self {
            name: None,
            curve_3d: None,
            surface_1: None,
            surface_2: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        curve: Rc<Curve>,
        surf1: Rc<Surface>,
        surf2: Rc<Surface>,
    ) {
        self.name = Some(name);
        self.curve_3d = Some(curve);
        self.surface_1 = Some(surf1);
        self.surface_2 = Some(surf2);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn curve_3d(&self) -> Option<&Rc<Curve>> {
        self.curve_3d.as_ref()
    }

    pub fn surface_1(&self) -> Option<&Rc<Surface>> {
        self.surface_1.as_ref()
    }

    pub fn surface_2(&self) -> Option<&Rc<Surface>> {
        self.surface_2.as_ref()
    }
}

impl Default for StepGeomIntersectionCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ic = StepGeomIntersectionCurve::new();
        assert_eq!(ic.name(), None);
    }

    #[test]
    fn test_init() {
        let mut ic = StepGeomIntersectionCurve::new();
        let curve = Rc::new(Curve::new("CURVE_1".to_string()));
        let surf1 = Rc::new(Surface::new("SURF_A".to_string()));
        let surf2 = Rc::new(Surface::new("SURF_B".to_string()));
        ic.init("int_curve".to_string(), curve, surf1, surf2);
        assert_eq!(ic.name(), Some("int_curve"));
    }
}
