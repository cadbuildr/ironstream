// FILE: step_element_element_aspect.rs
// occt: StepElement_ElementAspect

/// Representation of STEP SELECT type ElementAspect
#[derive(Clone, Debug, PartialEq)]
pub enum ElementAspect {
    ElementVolume(ElementVolume),
    Volume3dFace(i32),
    Volume2dFace(i32),
    Volume3dEdge(i32),
    Volume2dEdge(i32),
    Surface3dFace(i32),
    Surface2dFace(i32),
    Surface3dEdge(i32),
    Surface2dEdge(i32),
    CurveEdge(CurveEdge),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElementVolume {
    Volume,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurveEdge {
    Edge,
}

impl ElementAspect {
    pub fn case_num(&self) -> i32 {
        0
    }

    pub fn case_mem(&self) -> i32 {
        match self {
            ElementAspect::ElementVolume(_) => 1,
            ElementAspect::Volume3dFace(_) => 2,
            ElementAspect::Volume2dFace(_) => 3,
            ElementAspect::Volume3dEdge(_) => 4,
            ElementAspect::Volume2dEdge(_) => 5,
            ElementAspect::Surface3dFace(_) => 6,
            ElementAspect::Surface2dFace(_) => 7,
            ElementAspect::Surface3dEdge(_) => 8,
            ElementAspect::Surface2dEdge(_) => 9,
            ElementAspect::CurveEdge(_) => 10,
        }
    }

    pub fn element_volume(&self) -> Option<ElementVolume> {
        match self {
            ElementAspect::ElementVolume(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn set_element_volume(&mut self, val: ElementVolume) {
        *self = ElementAspect::ElementVolume(val);
    }

    pub fn volume_3d_face(&self) -> Option<i32> {
        match self {
            ElementAspect::Volume3dFace(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_volume_3d_face(&mut self, val: i32) {
        *self = ElementAspect::Volume3dFace(val);
    }

    pub fn volume_2d_face(&self) -> Option<i32> {
        match self {
            ElementAspect::Volume2dFace(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_volume_2d_face(&mut self, val: i32) {
        *self = ElementAspect::Volume2dFace(val);
    }

    pub fn volume_3d_edge(&self) -> Option<i32> {
        match self {
            ElementAspect::Volume3dEdge(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_volume_3d_edge(&mut self, val: i32) {
        *self = ElementAspect::Volume3dEdge(val);
    }

    pub fn volume_2d_edge(&self) -> Option<i32> {
        match self {
            ElementAspect::Volume2dEdge(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_volume_2d_edge(&mut self, val: i32) {
        *self = ElementAspect::Volume2dEdge(val);
    }

    pub fn surface_3d_face(&self) -> Option<i32> {
        match self {
            ElementAspect::Surface3dFace(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_surface_3d_face(&mut self, val: i32) {
        *self = ElementAspect::Surface3dFace(val);
    }

    pub fn surface_2d_face(&self) -> Option<i32> {
        match self {
            ElementAspect::Surface2dFace(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_surface_2d_face(&mut self, val: i32) {
        *self = ElementAspect::Surface2dFace(val);
    }

    pub fn surface_3d_edge(&self) -> Option<i32> {
        match self {
            ElementAspect::Surface3dEdge(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_surface_3d_edge(&mut self, val: i32) {
        *self = ElementAspect::Surface3dEdge(val);
    }

    pub fn surface_2d_edge(&self) -> Option<i32> {
        match self {
            ElementAspect::Surface2dEdge(v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_surface_2d_edge(&mut self, val: i32) {
        *self = ElementAspect::Surface2dEdge(val);
    }

    pub fn curve_edge(&self) -> Option<CurveEdge> {
        match self {
            ElementAspect::CurveEdge(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn set_curve_edge(&mut self, val: CurveEdge) {
        *self = ElementAspect::CurveEdge(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_mem() {
        let aspect = ElementAspect::ElementVolume(ElementVolume::Volume);
        assert_eq!(aspect.case_mem(), 1);

        let aspect2 = ElementAspect::Volume3dFace(42);
        assert_eq!(aspect2.case_mem(), 2);

        let aspect3 = ElementAspect::CurveEdge(CurveEdge::Edge);
        assert_eq!(aspect3.case_mem(), 10);
    }

    #[test]
    fn test_set_and_get() {
        let mut aspect = ElementAspect::Volume3dFace(0);
        aspect.set_surface_3d_face(100);
        assert_eq!(aspect.surface_3d_face(), Some(100));
        assert_eq!(aspect.volume_3d_face(), None);
    }

    #[test]
    fn test_element_volume() {
        let aspect = ElementAspect::ElementVolume(ElementVolume::Volume);
        assert!(aspect.element_volume().is_some());
        assert_eq!(aspect.element_volume().unwrap(), ElementVolume::Volume);
    }

    #[test]
    fn test_curve_edge() {
        let mut aspect = ElementAspect::CurveEdge(CurveEdge::Edge);
        let edge = aspect.curve_edge();
        assert_eq!(edge, Some(CurveEdge::Edge));
    }
}
