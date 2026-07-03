// FILE: graphic3d_type_of_material.rs
// occt: Graphic3d_TypeOfMaterial

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfMaterial { Aspect = 0, Physic = 1, }

impl TypeOfMaterial {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val { 0 => Some(TypeOfMaterial::Aspect), 1 => Some(TypeOfMaterial::Physic), _ => None, }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variants() { assert_eq!(TypeOfMaterial::Aspect as u32, 0); }
}
