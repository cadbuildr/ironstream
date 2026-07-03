// FILE: graphic3d_type_of_visualization.rs
// occt: Graphic3d_TypeOfVisualization

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfVisualization { Wireframe = 0, Shading = 1, }

impl TypeOfVisualization {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val { 0 => Some(TypeOfVisualization::Wireframe), 1 => Some(TypeOfVisualization::Shading), _ => None, }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wireframe() { assert_eq!(TypeOfVisualization::Wireframe as u32, 0); }
}
