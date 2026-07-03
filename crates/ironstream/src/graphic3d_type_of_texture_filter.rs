// FILE: graphic3d_type_of_texture_filter.rs
// occt: Graphic3d_TypeOfTextureFilter

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfTextureFilter { Nearest = 0, Bilinear = 1, Trilinear = 2, }

impl TypeOfTextureFilter {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val { 0 => Some(TypeOfTextureFilter::Nearest), _ => None, }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variants() { assert_eq!(TypeOfTextureFilter::Nearest as u32, 0); }
}
