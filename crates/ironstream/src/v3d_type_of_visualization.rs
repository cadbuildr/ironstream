// FILE: v3d_type_of_visualization.rs
// occt: V3d_TypeOfVisualization

//! Determines the type of visualization in the view.
//! - WIREFRAME: show edges and silhouettes only
//! - ZBUFFER: shaded rendering with z-buffer depth testing

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TypeOfVisualization {
    #[default]
    Wireframe,
    Zbuffer,
}

impl TypeOfVisualization {
    pub fn as_u32(&self) -> u32 {
        match self {
            TypeOfVisualization::Wireframe => 0,
            TypeOfVisualization::Zbuffer => 1,
        }
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(TypeOfVisualization::Wireframe),
            1 => Some(TypeOfVisualization::Zbuffer),
            _ => None,
        }
    }

    pub fn is_wireframe(&self) -> bool {
        *self == TypeOfVisualization::Wireframe
    }

    pub fn is_shaded(&self) -> bool {
        *self == TypeOfVisualization::Zbuffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_of_visualization_conversions() {
        assert_eq!(TypeOfVisualization::Wireframe.as_u32(), 0);
        assert_eq!(TypeOfVisualization::Zbuffer.as_u32(), 1);
    }

    #[test]
    fn type_of_visualization_from_u32() {
        assert_eq!(TypeOfVisualization::from_u32(0), Some(TypeOfVisualization::Wireframe));
        assert_eq!(TypeOfVisualization::from_u32(1), Some(TypeOfVisualization::Zbuffer));
        assert_eq!(TypeOfVisualization::from_u32(2), None);
    }

    #[test]
    fn type_of_visualization_predicates() {
        assert!(TypeOfVisualization::Wireframe.is_wireframe());
        assert!(!TypeOfVisualization::Wireframe.is_shaded());
        assert!(!TypeOfVisualization::Zbuffer.is_wireframe());
        assert!(TypeOfVisualization::Zbuffer.is_shaded());
    }
}
