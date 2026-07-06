// FILE: graphic3d_transform_pers.rs
// occt: Graphic3d_TransformPers

/// Local model of Graphic3d_TransModeFlags (bitmask newtype, self-contained).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransModeFlags(pub u32);

impl TransModeFlags {
    /// no persistence attributes (normal 3D object)
    pub const NONE: TransModeFlags = TransModeFlags(0x0000);
    /// object does not resize
    pub const ZOOM_PERS: TransModeFlags = TransModeFlags(0x0002);
    /// object does not rotate
    pub const ROTATE_PERS: TransModeFlags = TransModeFlags(0x0008);
    /// object behaves like trihedron
    pub const TRIHEDRON_PERS: TransModeFlags = TransModeFlags(0x0020);
    /// object is defined in 2D screen coordinates
    pub const TMF_2D: TransModeFlags = TransModeFlags(0x0040);
    /// object is in front of the camera
    pub const CAMERA_PERS: TransModeFlags = TransModeFlags(0x0080);
    /// orthographic projection persistence
    pub const ORTHO_PERS: TransModeFlags = TransModeFlags(0x0100);
    /// object does not resize with axial scale
    pub const AXIAL_SCALE_PERS: TransModeFlags = TransModeFlags(0x0200);
    /// object doesn't resize and rotate
    pub const ZOOM_ROTATE_PERS: TransModeFlags = TransModeFlags(0x0002 | 0x0008);
    /// object does not visually resize with either object or axial scale
    pub const AXIAL_ZOOM_PERS: TransModeFlags = TransModeFlags(0x0002 | 0x0200);

    /// Get the raw bit value.
    pub fn bits(&self) -> u32 {
        self.0
    }
}

/// Trihedron corner position (simplified placeholder)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriedronCorner {
    LeftLower,
    LeftUpper,
    RightLower,
    RightUpper,
}

/// 3D anchor point for zoom/rotate transformation persistence
#[derive(Debug, Clone)]
struct PersParams3d {
    pnt_x: f64,
    pnt_y: f64,
    pnt_z: f64,
}

/// 2d/trihedron transformation persistence parameters
#[derive(Debug, Clone)]
struct PersParams2d {
    corner: TriedronCorner,
    offset_x: i32,
    offset_y: i32,
}

/// Transformation Persistence definition.
///
/// Transformation Persistence defines a mutable Local Coordinate system which depends on camera
/// position, so that visual appearance of the object becomes partially immutable while camera
/// moves. Object visually preserves particular property such as size, placement, rotation or their
/// combination.
pub struct TransformPers {
    mode: TransModeFlags,
    params_3d: PersParams3d,
    params_2d: PersParams2d,
}

impl TransformPers {
    /// Return true if specified mode is zoom/rotate transformation persistence.
    pub fn is_zoom_or_rotate_mode(mode: TransModeFlags) -> bool {
        (mode.bits() & (TransModeFlags::ZOOM_PERS.bits() | TransModeFlags::ROTATE_PERS.bits())) != 0
    }

    /// Return true if specified mode is 2d/trihedron transformation persistence.
    pub fn is_trihedron_or_2d_mode(mode: TransModeFlags) -> bool {
        (mode.bits() & (TransModeFlags::TRIHEDRON_PERS.bits() | TransModeFlags::TMF_2D.bits())) != 0
    }

    /// Return true if specified mode is orthographic projection transformation persistence.
    pub fn is_ortho_pers_mode(mode: TransModeFlags) -> bool {
        (mode.bits() & TransModeFlags::ORTHO_PERS.bits()) != 0
    }

    /// Return true if specified mode is axial transformation persistence.
    pub fn is_axial_mode(mode: TransModeFlags) -> bool {
        (mode.bits() & TransModeFlags::AXIAL_SCALE_PERS.bits()) != 0
    }

    /// Create a new TransformPers with the specified mode.
    pub fn new(mode: TransModeFlags) -> Self {
        if mode == TransModeFlags::NONE {
            panic!("Graphic3d_TransformPers: NONE mode not permitted - use NULL handle instead");
        }

        let (params_3d, params_2d) = if Self::is_zoom_or_rotate_mode(mode) || Self::is_axial_mode(mode) {
            (
                PersParams3d {
                    pnt_x: 0.0,
                    pnt_y: 0.0,
                    pnt_z: 0.0,
                },
                PersParams2d {
                    corner: TriedronCorner::LeftLower,
                    offset_x: 0,
                    offset_y: 0,
                },
            )
        } else if Self::is_trihedron_or_2d_mode(mode) {
            (
                PersParams3d {
                    pnt_x: 0.0,
                    pnt_y: 0.0,
                    pnt_z: 0.0,
                },
                PersParams2d {
                    corner: TriedronCorner::LeftLower,
                    offset_x: 0,
                    offset_y: 0,
                },
            )
        } else if mode == TransModeFlags::CAMERA_PERS {
            (
                PersParams3d {
                    pnt_x: 0.0,
                    pnt_y: 0.0,
                    pnt_z: 0.0,
                },
                PersParams2d {
                    corner: TriedronCorner::LeftLower,
                    offset_x: 0,
                    offset_y: 0,
                },
            )
        } else {
            panic!("Graphic3d_TransformPers::new: wrong persistence mode");
        };

        TransformPers {
            mode,
            params_3d,
            params_2d,
        }
    }

    /// Create with Zoom/Rotate transformation persistence and an anchor 3D point.
    pub fn new_with_anchor(mode: TransModeFlags, pnt_x: f64, pnt_y: f64, pnt_z: f64) -> Self {
        if !Self::is_zoom_or_rotate_mode(mode) && !Self::is_axial_mode(mode) {
            panic!("Graphic3d_TransformPers::new_with_anchor: wrong persistence mode");
        }

        TransformPers {
            mode,
            params_3d: PersParams3d { pnt_x, pnt_y, pnt_z },
            params_2d: PersParams2d {
                corner: TriedronCorner::LeftLower,
                offset_x: 0,
                offset_y: 0,
            },
        }
    }

    /// Create 2d/trihedron transformation persistence with a corner and 2D offset.
    pub fn new_with_corner(
        mode: TransModeFlags,
        corner: TriedronCorner,
        offset_x: i32,
        offset_y: i32,
    ) -> Self {
        if !Self::is_trihedron_or_2d_mode(mode) {
            panic!("Graphic3d_TransformPers::new_with_corner: wrong persistence mode");
        }

        TransformPers {
            mode,
            params_3d: PersParams3d {
                pnt_x: 0.0,
                pnt_y: 0.0,
                pnt_z: 0.0,
            },
            params_2d: PersParams2d {
                corner,
                offset_x,
                offset_y,
            },
        }
    }

    /// Return true for Graphic3d_TMF_ZoomPers, Graphic3d_TMF_ZoomRotatePers or Graphic3d_TMF_RotatePers modes.
    pub fn is_zoom_or_rotate(&self) -> bool {
        Self::is_zoom_or_rotate_mode(self.mode)
    }

    /// Return true for Graphic3d_TMF_TriedronPers and Graphic3d_TMF_2d modes.
    pub fn is_trihedron_or_2d(&self) -> bool {
        Self::is_trihedron_or_2d_mode(self.mode)
    }

    /// Return true for Graphic3d_TMF_OrthoPers mode.
    pub fn is_ortho_pers(&self) -> bool {
        Self::is_ortho_pers_mode(self.mode)
    }

    /// Return true for Graphic3d_TMF_AxialScalePers modes.
    pub fn is_axial(&self) -> bool {
        Self::is_axial_mode(self.mode)
    }

    /// Return transformation persistence mode flags.
    pub fn mode(&self) -> TransModeFlags {
        self.mode
    }

    /// Return transformation persistence mode flags.
    pub fn flags(&self) -> TransModeFlags {
        self.mode
    }

    /// Set Zoom/Rotate transformation persistence with an anchor 3D point.
    pub fn set_persistence_with_anchor(&mut self, mode: TransModeFlags, pnt_x: f64, pnt_y: f64, pnt_z: f64) {
        if !Self::is_zoom_or_rotate_mode(mode) && !Self::is_axial_mode(mode) {
            panic!("Graphic3d_TransformPers::set_persistence_with_anchor: wrong persistence mode");
        }

        self.mode = mode;
        self.params_3d = PersParams3d { pnt_x, pnt_y, pnt_z };
    }

    /// Set 2d/trihedron transformation persistence with a corner and 2D offset.
    pub fn set_persistence_with_corner(
        &mut self,
        mode: TransModeFlags,
        corner: TriedronCorner,
        offset_x: i32,
        offset_y: i32,
    ) {
        if !Self::is_trihedron_or_2d_mode(mode) {
            panic!("Graphic3d_TransformPers::set_persistence_with_corner: wrong persistence mode");
        }

        self.mode = mode;
        self.params_2d = PersParams2d {
            corner,
            offset_x,
            offset_y,
        };
    }

    /// Return the anchor point for zoom/rotate transformation persistence.
    pub fn anchor_point(&self) -> (f64, f64, f64) {
        if !self.is_zoom_or_rotate() && !self.is_axial() {
            panic!("Graphic3d_TransformPers::anchor_point: wrong persistence mode");
        }
        (
            self.params_3d.pnt_x,
            self.params_3d.pnt_y,
            self.params_3d.pnt_z,
        )
    }

    /// Set the anchor point for zoom/rotate transformation persistence.
    pub fn set_anchor_point(&mut self, pnt_x: f64, pnt_y: f64, pnt_z: f64) {
        if !self.is_zoom_or_rotate() && !self.is_axial() {
            panic!("Graphic3d_TransformPers::set_anchor_point: wrong persistence mode");
        }
        self.params_3d.pnt_x = pnt_x;
        self.params_3d.pnt_y = pnt_y;
        self.params_3d.pnt_z = pnt_z;
    }

    /// Return the corner for 2d/trihedron transformation persistence.
    pub fn corner_2d(&self) -> TriedronCorner {
        if !self.is_trihedron_or_2d() {
            panic!("Graphic3d_TransformPers::corner_2d: wrong persistence mode");
        }
        self.params_2d.corner
    }

    /// Set the corner for 2d/trihedron transformation persistence.
    pub fn set_corner_2d(&mut self, corner: TriedronCorner) {
        if !self.is_trihedron_or_2d() {
            panic!("Graphic3d_TransformPers::set_corner_2d: wrong persistence mode");
        }
        self.params_2d.corner = corner;
    }

    /// Return the offset from the corner for 2d/trihedron transformation persistence.
    pub fn offset_2d(&self) -> (i32, i32) {
        if !self.is_trihedron_or_2d() {
            panic!("Graphic3d_TransformPers::offset_2d: wrong persistence mode");
        }
        (self.params_2d.offset_x, self.params_2d.offset_y)
    }

    /// Set the offset from the corner for 2d/trihedron transformation persistence.
    pub fn set_offset_2d(&mut self, offset_x: i32, offset_y: i32) {
        if !self.is_trihedron_or_2d() {
            panic!("Graphic3d_TransformPers::set_offset_2d: wrong persistence mode");
        }
        self.params_2d.offset_x = offset_x;
        self.params_2d.offset_y = offset_y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_zoom_or_rotate_flag() {
        assert!(TransformPers::is_zoom_or_rotate_mode(TransModeFlags::ZOOM_PERS));
        assert!(TransformPers::is_zoom_or_rotate_mode(TransModeFlags::ROTATE_PERS));
        assert!(TransformPers::is_zoom_or_rotate_mode(TransModeFlags::ZOOM_ROTATE_PERS));
        assert!(!TransformPers::is_zoom_or_rotate_mode(TransModeFlags::TRIHEDRON_PERS));
    }

    #[test]
    fn test_is_trihedron_or_2d_flag() {
        assert!(TransformPers::is_trihedron_or_2d_mode(TransModeFlags::TRIHEDRON_PERS));
        assert!(TransformPers::is_trihedron_or_2d_mode(TransModeFlags::TMF_2D));
        assert!(!TransformPers::is_trihedron_or_2d_mode(TransModeFlags::ZOOM_PERS));
    }

    #[test]
    fn test_is_ortho_pers_flag() {
        assert!(TransformPers::is_ortho_pers_mode(TransModeFlags::ORTHO_PERS));
        assert!(!TransformPers::is_ortho_pers_mode(TransModeFlags::ZOOM_PERS));
    }

    #[test]
    fn test_is_axial_flag() {
        assert!(TransformPers::is_axial_mode(TransModeFlags::AXIAL_SCALE_PERS));
        assert!(TransformPers::is_axial_mode(TransModeFlags::AXIAL_ZOOM_PERS));
        assert!(!TransformPers::is_axial_mode(TransModeFlags::ZOOM_PERS));
    }

    #[test]
    fn test_new_zoom_pers() {
        let pers = TransformPers::new(TransModeFlags::ZOOM_PERS);
        assert!(pers.is_zoom_or_rotate());
        assert_eq!(pers.mode(), TransModeFlags::ZOOM_PERS);
    }

    #[test]
    fn test_new_trihedron_pers() {
        let pers = TransformPers::new(TransModeFlags::TRIHEDRON_PERS);
        assert!(pers.is_trihedron_or_2d());
        assert_eq!(pers.mode(), TransModeFlags::TRIHEDRON_PERS);
    }

    #[test]
    fn test_new_with_anchor() {
        let pers = TransformPers::new_with_anchor(TransModeFlags::ZOOM_PERS, 1.0, 2.0, 3.0);
        let (x, y, z) = pers.anchor_point();
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_new_with_corner() {
        let pers = TransformPers::new_with_corner(
            TransModeFlags::TRIHEDRON_PERS,
            TriedronCorner::RightUpper,
            10,
            20,
        );
        assert_eq!(pers.corner_2d(), TriedronCorner::RightUpper);
        let (ox, oy) = pers.offset_2d();
        assert_eq!(ox, 10);
        assert_eq!(oy, 20);
    }

    #[test]
    fn test_set_anchor_point() {
        let mut pers = TransformPers::new_with_anchor(TransModeFlags::ROTATE_PERS, 0.0, 0.0, 0.0);
        pers.set_anchor_point(5.0, 6.0, 7.0);
        let (x, y, z) = pers.anchor_point();
        assert_eq!(x, 5.0);
        assert_eq!(y, 6.0);
        assert_eq!(z, 7.0);
    }

    #[test]
    fn test_set_corner_2d() {
        let mut pers = TransformPers::new_with_corner(
            TransModeFlags::TMF_2D,
            TriedronCorner::LeftLower,
            0,
            0,
        );
        pers.set_corner_2d(TriedronCorner::RightLower);
        assert_eq!(pers.corner_2d(), TriedronCorner::RightLower);
    }

    #[test]
    fn test_set_offset_2d() {
        let mut pers = TransformPers::new_with_corner(
            TransModeFlags::TRIHEDRON_PERS,
            TriedronCorner::LeftUpper,
            0,
            0,
        );
        pers.set_offset_2d(15, 25);
        let (ox, oy) = pers.offset_2d();
        assert_eq!(ox, 15);
        assert_eq!(oy, 25);
    }

    #[test]
    #[should_panic]
    fn test_new_none_panics() {
        let _ = TransformPers::new(TransModeFlags::NONE);
    }

    #[test]
    #[should_panic]
    fn test_anchor_point_on_trihedron_panics() {
        let pers = TransformPers::new(TransModeFlags::TRIHEDRON_PERS);
        let _ = pers.anchor_point();
    }

    #[test]
    #[should_panic]
    fn test_corner_2d_on_zoom_panics() {
        let pers = TransformPers::new(TransModeFlags::ZOOM_PERS);
        let _ = pers.corner_2d();
    }
}
