// FILE: graphic3d_cube_map_side.rs
// occt: Graphic3d_CubeMapSide

//! Sides of cubemap in order of OpenGL rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CubeMapSide {
    /// X axis positive direction side
    PosX = 0,
    /// X axis negative direction side
    NegX = 1,
    /// Y axis positive direction side
    PosY = 2,
    /// Y axis negative direction side
    NegY = 3,
    /// Z axis positive direction side
    PosZ = 4,
    /// Z axis negative direction side
    NegZ = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_map_side_variants() {
        // Verify all variants are distinct
        assert_eq!(CubeMapSide::PosX as i32, 0);
        assert_eq!(CubeMapSide::NegX as i32, 1);
        assert_eq!(CubeMapSide::PosY as i32, 2);
        assert_eq!(CubeMapSide::NegY as i32, 3);
        assert_eq!(CubeMapSide::PosZ as i32, 4);
        assert_eq!(CubeMapSide::NegZ as i32, 5);

        // Verify they are not equal
        assert_ne!(CubeMapSide::PosX, CubeMapSide::NegX);
        assert_ne!(CubeMapSide::PosY, CubeMapSide::NegY);
        assert_ne!(CubeMapSide::PosZ, CubeMapSide::NegZ);
    }

    #[test]
    fn test_cube_map_side_copy() {
        let side = CubeMapSide::PosX;
        let side_copy = side;
        assert_eq!(side, side_copy);
    }
}
