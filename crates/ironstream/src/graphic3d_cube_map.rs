// FILE: graphic3d_cube_map.rs
// occt: Graphic3d_CubeMap

/// Enumeration of cubemap sides
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Graphic3dCubeMapSide {
    /// Positive X side
    PosX,
    /// Negative X side
    NegX,
    /// Positive Y side
    PosY,
    /// Negative Y side
    NegY,
    /// Positive Z side
    PosZ,
    /// Negative Z side
    NegZ,
}

/// Base class for cubemaps.
/// It is iterator over cubemap sides.
pub struct Graphic3dCubeMap {
    /// Iterator state - current cubemap side
    current_side: Graphic3dCubeMapSide,
    /// Indicates whether end of iteration has been reached or hasn't
    end_is_reached: bool,
    /// Indicates whether Z axis is inverted
    z_is_inverted: bool,
    /// Whether to generate mipmaps of cubemap
    has_mipmaps: bool,
}

impl Graphic3dCubeMap {
    /// Create a new cubemap iterator (initially at +X side).
    pub fn new() -> Self {
        Self {
            current_side: Graphic3dCubeMapSide::PosX,
            end_is_reached: false,
            z_is_inverted: false,
            has_mipmaps: false,
        }
    }

    /// Returns whether the iterator has reached the end (true if it hasn't).
    pub fn more(&self) -> bool {
        !self.end_is_reached
    }

    /// Returns current cubemap side (iterator state).
    pub fn current_side(&self) -> Graphic3dCubeMapSide {
        self.current_side
    }

    /// Moves iterator to the next cubemap side.
    /// Uses OpenGL cubemap sides order +X -> -X -> +Y -> -Y -> +Z -> -Z.
    pub fn next(&mut self) {
        if !self.end_is_reached && self.current_side == Graphic3dCubeMapSide::NegZ {
            self.end_is_reached = true;
        } else if !self.end_is_reached {
            // Advance to next side
            self.current_side = match self.current_side {
                Graphic3dCubeMapSide::PosX => Graphic3dCubeMapSide::NegX,
                Graphic3dCubeMapSide::NegX => Graphic3dCubeMapSide::PosY,
                Graphic3dCubeMapSide::PosY => Graphic3dCubeMapSide::NegY,
                Graphic3dCubeMapSide::NegY => Graphic3dCubeMapSide::PosZ,
                Graphic3dCubeMapSide::PosZ => Graphic3dCubeMapSide::NegZ,
                Graphic3dCubeMapSide::NegZ => Graphic3dCubeMapSide::NegZ,
            };
        }
    }

    /// Sets Z axis inversion (vertical flipping).
    pub fn set_z_inversion(&mut self, z_is_inverted: bool) {
        self.z_is_inverted = z_is_inverted;
    }

    /// Returns whether Z axis is inverted.
    pub fn z_is_inverted(&self) -> bool {
        self.z_is_inverted
    }

    /// Returns whether mipmaps of cubemap will be generated or not.
    pub fn has_mipmaps(&self) -> bool {
        self.has_mipmaps
    }

    /// Sets whether to generate mipmaps of cubemap or not.
    pub fn set_mipmaps_generation(&mut self, to_generate_mipmaps: bool) {
        self.has_mipmaps = to_generate_mipmaps;
    }

    /// Sets iterator state to +X cubemap side.
    pub fn reset(&mut self) {
        self.current_side = Graphic3dCubeMapSide::PosX;
        self.end_is_reached = false;
    }
}

impl Default for Graphic3dCubeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cubemap() {
        let cubemap = Graphic3dCubeMap::new();
        assert!(cubemap.more());
        assert_eq!(cubemap.current_side(), Graphic3dCubeMapSide::PosX);
        assert!(!cubemap.z_is_inverted());
        assert!(!cubemap.has_mipmaps());
    }

    #[test]
    fn test_iteration_order() {
        let mut cubemap = Graphic3dCubeMap::new();
        assert_eq!(cubemap.current_side(), Graphic3dCubeMapSide::PosX);
        cubemap.next();
        assert_eq!(cubemap.current_side(), Graphic3dCubeMapSide::NegX);
        cubemap.next();
        assert_eq!(cubemap.current_side(), Graphic3dCubeMapSide::PosY);
        cubemap.next();
        assert_eq!(cubemap.current_side(), Graphic3dCubeMapSide::NegY);
        cubemap.next();
        assert_eq!(cubemap.current_side(), Graphic3dCubeMapSide::PosZ);
        cubemap.next();
        assert_eq!(cubemap.current_side(), Graphic3dCubeMapSide::NegZ);
        assert!(cubemap.more());
        cubemap.next();
        assert!(!cubemap.more());
    }

    #[test]
    fn test_z_inversion() {
        let mut cubemap = Graphic3dCubeMap::new();
        assert!(!cubemap.z_is_inverted());
        cubemap.set_z_inversion(true);
        assert!(cubemap.z_is_inverted());
    }

    #[test]
    fn test_mipmaps() {
        let mut cubemap = Graphic3dCubeMap::new();
        assert!(!cubemap.has_mipmaps());
        cubemap.set_mipmaps_generation(true);
        assert!(cubemap.has_mipmaps());
    }

    #[test]
    fn test_reset() {
        let mut cubemap = Graphic3dCubeMap::new();
        cubemap.next();
        cubemap.next();
        assert_ne!(cubemap.current_side(), Graphic3dCubeMapSide::PosX);
        cubemap.reset();
        assert_eq!(cubemap.current_side(), Graphic3dCubeMapSide::PosX);
        assert!(cubemap.more());
    }

    #[test]
    fn test_default() {
        let cubemap = Graphic3dCubeMap::default();
        assert_eq!(cubemap.current_side(), Graphic3dCubeMapSide::PosX);
    }

    #[test]
    fn test_full_iteration() {
        let mut cubemap = Graphic3dCubeMap::new();
        let sides = vec![
            Graphic3dCubeMapSide::PosX,
            Graphic3dCubeMapSide::NegX,
            Graphic3dCubeMapSide::PosY,
            Graphic3dCubeMapSide::NegY,
            Graphic3dCubeMapSide::PosZ,
            Graphic3dCubeMapSide::NegZ,
        ];

        for (i, &expected_side) in sides.iter().enumerate() {
            assert!(cubemap.more(), "Iterator should have more items at step {}", i);
            assert_eq!(
                cubemap.current_side(),
                expected_side,
                "Side mismatch at step {}",
                i
            );
            cubemap.next();
        }

        assert!(!cubemap.more(), "Iterator should be exhausted");
    }
}
