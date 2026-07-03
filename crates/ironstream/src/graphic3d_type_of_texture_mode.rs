// FILE: graphic3d_type_of_texture_mode.rs
// occt: Graphic3d_TypeOfTextureMode

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfTextureMode { Object = 0, Sphere = 1, Eye = 2, Manual = 3, Sprite = 4, }

impl TypeOfTextureMode {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val { 0 => Some(TypeOfTextureMode::Object), _ => None, }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variants() { assert_eq!(TypeOfTextureMode::Object as u32, 0); }
}
