// FILE: hlr_algo_bi_point.rs
// occt: HLRAlgo_BiPoint

/// A segment of a visible or hidden line between two points.
/// Stores 3D and 2D coordinates, indices, and flags (Rg1Line, RgNLine, OutLine, IntLine, Hidden).
pub struct HlrAlgoBiPoint {
    // 3D Points
    pnt1: [f64; 3],  // First 3D point
    pnt2: [f64; 3],  // Second 3D point
    // Projected 2D Points
    pnt_p1: [f64; 3],  // Projected first point
    pnt_p2: [f64; 3],  // Projected second point

    // Indices
    shape_index: i32,
    face_conex1: i32,
    face1_pt1: i32,
    face1_pt2: i32,
    face_conex2: i32,
    face2_pt1: i32,
    face2_pt2: i32,
    min_seg: i32,
    max_seg: i32,
    seg_flags: i32,
}

impl HlrAlgoBiPoint {
    const EMSK_RG1_LINE: i32 = 1;
    const EMSK_RGN_LINE: i32 = 2;
    const EMSK_OUTLINE: i32 = 4;
    const EMSK_INTLINE: i32 = 8;
    const EMSK_HIDDEN: i32 = 16;

    /// Create a default BiPoint.
    pub fn new() -> Self {
        HlrAlgoBiPoint {
            pnt1: [0.0; 3],
            pnt2: [0.0; 3],
            pnt_p1: [0.0; 3],
            pnt_p2: [0.0; 3],
            shape_index: -1,
            face_conex1: 0,
            face1_pt1: 0,
            face1_pt2: 0,
            face_conex2: 0,
            face2_pt1: 0,
            face2_pt2: 0,
            min_seg: 0,
            max_seg: 0,
            seg_flags: 0,
        }
    }

    /// Create a BiPoint with coordinates and flags.
    pub fn with_points_and_flags(
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
        xt1: f64,
        yt1: f64,
        zt1: f64,
        xt2: f64,
        yt2: f64,
        zt2: f64,
        shape_idx: i32,
        reg1: bool,
        regn: bool,
        outl: bool,
        intl: bool,
    ) -> Self {
        let mut bp = HlrAlgoBiPoint {
            pnt1: [x1, y1, z1],
            pnt2: [x2, y2, z2],
            pnt_p1: [xt1, yt1, zt1],
            pnt_p2: [xt2, yt2, zt2],
            shape_index: shape_idx,
            face_conex1: 0,
            face1_pt1: 0,
            face1_pt2: 0,
            face_conex2: 0,
            face2_pt1: 0,
            face2_pt2: 0,
            min_seg: 0,
            max_seg: 0,
            seg_flags: 0,
        };
        if reg1 {
            bp.seg_flags |= Self::EMSK_RG1_LINE;
        }
        if regn {
            bp.seg_flags |= Self::EMSK_RGN_LINE;
        }
        if outl {
            bp.seg_flags |= Self::EMSK_OUTLINE;
        }
        if intl {
            bp.seg_flags |= Self::EMSK_INTLINE;
        }
        bp
    }

    /// Check if first point is on a regular line (at least C1 continuous).
    pub fn rg1_line(&self) -> bool {
        (self.seg_flags & Self::EMSK_RG1_LINE) != 0
    }

    /// Set Rg1Line flag.
    pub fn set_rg1_line(&mut self, b: bool) {
        if b {
            self.seg_flags |= Self::EMSK_RG1_LINE;
        } else {
            self.seg_flags &= !Self::EMSK_RG1_LINE;
        }
    }

    /// Check if second point is on a regular line (at least C1 continuous).
    pub fn rgn_line(&self) -> bool {
        (self.seg_flags & Self::EMSK_RGN_LINE) != 0
    }

    /// Set RgNLine flag.
    pub fn set_rgn_line(&mut self, b: bool) {
        if b {
            self.seg_flags |= Self::EMSK_RGN_LINE;
        } else {
            self.seg_flags &= !Self::EMSK_RGN_LINE;
        }
    }

    /// Check if segment is on outline.
    pub fn outline(&self) -> bool {
        (self.seg_flags & Self::EMSK_OUTLINE) != 0
    }

    /// Set OutLine flag.
    pub fn set_outline(&mut self, b: bool) {
        if b {
            self.seg_flags |= Self::EMSK_OUTLINE;
        } else {
            self.seg_flags &= !Self::EMSK_OUTLINE;
        }
    }

    /// Check if segment is internal.
    pub fn int_line(&self) -> bool {
        (self.seg_flags & Self::EMSK_INTLINE) != 0
    }

    /// Set IntLine flag.
    pub fn set_int_line(&mut self, b: bool) {
        if b {
            self.seg_flags |= Self::EMSK_INTLINE;
        } else {
            self.seg_flags &= !Self::EMSK_INTLINE;
        }
    }

    /// Check if segment is hidden.
    pub fn hidden(&self) -> bool {
        (self.seg_flags & Self::EMSK_HIDDEN) != 0
    }

    /// Set Hidden flag.
    pub fn set_hidden(&mut self, b: bool) {
        if b {
            self.seg_flags |= Self::EMSK_HIDDEN;
        } else {
            self.seg_flags &= !Self::EMSK_HIDDEN;
        }
    }

    /// Get first 3D point.
    pub fn pnt1(&self) -> [f64; 3] {
        self.pnt1
    }

    /// Get second 3D point.
    pub fn pnt2(&self) -> [f64; 3] {
        self.pnt2
    }

    /// Get first projected 2D point (X, Y only).
    pub fn pnt_p1_2d(&self) -> [f64; 2] {
        [self.pnt_p1[0], self.pnt_p1[1]]
    }

    /// Get second projected 2D point (X, Y only).
    pub fn pnt_p2_2d(&self) -> [f64; 2] {
        [self.pnt_p2[0], self.pnt_p2[1]]
    }

    /// Get shape index.
    pub fn shape_index(&self) -> i32 {
        self.shape_index
    }

    /// Set shape index.
    pub fn set_shape_index(&mut self, idx: i32) {
        self.shape_index = idx;
    }
}

impl Default for HlrAlgoBiPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bp = HlrAlgoBiPoint::new();
        assert_eq!(bp.pnt1(), [0.0, 0.0, 0.0]);
        assert_eq!(bp.pnt2(), [0.0, 0.0, 0.0]);
        assert!(!bp.rg1_line());
        assert!(!bp.hidden());
    }

    #[test]
    fn test_with_points_and_flags() {
        let bp = HlrAlgoBiPoint::with_points_and_flags(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 10, true, false, true,
            false,
        );
        assert_eq!(bp.pnt1(), [1.0, 2.0, 3.0]);
        assert_eq!(bp.pnt2(), [4.0, 5.0, 6.0]);
        assert_eq!(bp.pnt_p1_2d(), [1.1, 2.1]);
        assert_eq!(bp.shape_index(), 10);
        assert!(bp.rg1_line());
        assert!(!bp.rgn_line());
        assert!(bp.outline());
    }

    #[test]
    fn test_flags() {
        let mut bp = HlrAlgoBiPoint::new();
        assert!(!bp.hidden());
        bp.set_hidden(true);
        assert!(bp.hidden());
        bp.set_hidden(false);
        assert!(!bp.hidden());
    }

    #[test]
    fn test_2d_projection() {
        let bp = HlrAlgoBiPoint::with_points_and_flags(
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.5, 0.7, 0.0, 1.5, 1.2, 0.0, 0, false, false, false,
            false,
        );
        assert_eq!(bp.pnt_p1_2d(), [0.5, 0.7]);
        assert_eq!(bp.pnt_p2_2d(), [1.5, 1.2]);
    }
}
