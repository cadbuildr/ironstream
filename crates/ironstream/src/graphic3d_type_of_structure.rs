// FILE: graphic3d_type_of_structure.rs
// occt: Graphic3d_TypeOfStructure

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfStructure { Wireframe = 0, Shading = 1, Computed = 2, All = 3, }

impl TypeOfStructure {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val { 0 => Some(TypeOfStructure::Wireframe), _ => None, }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variants() { assert_eq!(TypeOfStructure::Wireframe as u32, 0); }
}
