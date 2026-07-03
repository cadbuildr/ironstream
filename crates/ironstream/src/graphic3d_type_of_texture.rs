// FILE: graphic3d_type_of_texture.rs
// occt: Graphic3d_TypeOfTexture

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeOfTexture { Texture1D = 0, Texture2D = 1, Texture3D = 2, Cubemap = 3, Texture2DMipmap = 4, }

impl TypeOfTexture {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val { 0 => Some(TypeOfTexture::Texture1D), _ => None, }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variants() { assert_eq!(TypeOfTexture::Texture1D as u32, 0); }
}
