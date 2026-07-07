// FILE: prs3d_datum_axes.rs
// occt: Prs3d_DatumAxes

//! Enumeration defining axes used in datum aspect, see Prs3d_Datum.
//! In OCCT this is a C enum used as a bitmask; modelled here as a
//! newtype over u32 with named constants (aliases included).

#![allow(non_upper_case_globals)]

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Prs3d_DatumAxes(u32);

impl Prs3d_DatumAxes {
    /// X axis of the datum
    pub const Prs3d_DatumAxes_XAxis: Prs3d_DatumAxes = Prs3d_DatumAxes(0x01);
    /// Y axis of the datum
    pub const Prs3d_DatumAxes_YAxis: Prs3d_DatumAxes = Prs3d_DatumAxes(0x02);
    /// Z axis of the datum
    pub const Prs3d_DatumAxes_ZAxis: Prs3d_DatumAxes = Prs3d_DatumAxes(0x04);
    /// XOY 2D axes
    pub const Prs3d_DatumAxes_XYAxes: Prs3d_DatumAxes = Prs3d_DatumAxes(0x01 | 0x02);
    /// YOZ 2D axes
    pub const Prs3d_DatumAxes_YZAxes: Prs3d_DatumAxes = Prs3d_DatumAxes(0x02 | 0x04);
    /// XOZ 2D axes
    pub const Prs3d_DatumAxes_XZAxes: Prs3d_DatumAxes = Prs3d_DatumAxes(0x01 | 0x04);
    /// XYZ 3D axes
    pub const Prs3d_DatumAxes_XYZAxes: Prs3d_DatumAxes = Prs3d_DatumAxes(0x01 | 0x02 | 0x04);

    // old aliases
    pub const Prs3d_DA_XAxis: Prs3d_DatumAxes = Self::Prs3d_DatumAxes_XAxis;
    pub const Prs3d_DA_YAxis: Prs3d_DatumAxes = Self::Prs3d_DatumAxes_YAxis;
    pub const Prs3d_DA_ZAxis: Prs3d_DatumAxes = Self::Prs3d_DatumAxes_ZAxis;
    pub const Prs3d_DA_XYAxis: Prs3d_DatumAxes = Self::Prs3d_DatumAxes_XYAxes;
    pub const Prs3d_DA_YZAxis: Prs3d_DatumAxes = Self::Prs3d_DatumAxes_YZAxes;
    pub const Prs3d_DA_XZAxis: Prs3d_DatumAxes = Self::Prs3d_DatumAxes_XZAxes;
    pub const Prs3d_DA_XYZAxis: Prs3d_DatumAxes = Self::Prs3d_DatumAxes_XYZAxes;

    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// Build from a raw value; valid values are non-empty combinations
    /// of the X/Y/Z axis bits.
    pub fn from_u32(val: u32) -> Option<Self> {
        if val != 0 && (val & !0x07) == 0 {
            Some(Prs3d_DatumAxes(val))
        } else {
            None
        }
    }

    /// True if the X axis bit is set.
    pub const fn has_x(self) -> bool {
        self.0 & 0x01 != 0
    }

    /// True if the Y axis bit is set.
    pub const fn has_y(self) -> bool {
        self.0 & 0x02 != 0
    }

    /// True if the Z axis bit is set.
    pub const fn has_z(self) -> bool {
        self.0 & 0x04 != 0
    }
}

impl std::ops::BitOr for Prs3d_DatumAxes {
    type Output = Prs3d_DatumAxes;

    fn bitor(self, rhs: Self) -> Self {
        Prs3d_DatumAxes(self.0 | rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_datum_axes_sanity() {
        let v = Prs3d_DatumAxes::from_u32(0x01).unwrap();
        assert_eq!(v.as_u32(), 0x01);
        assert_eq!(v, Prs3d_DatumAxes::Prs3d_DatumAxes_XAxis);
    }

    #[test]
    fn combined_values_match_occt() {
        assert_eq!(Prs3d_DatumAxes::Prs3d_DatumAxes_XYAxes.as_u32(), 0x03);
        assert_eq!(Prs3d_DatumAxes::Prs3d_DatumAxes_YZAxes.as_u32(), 0x06);
        assert_eq!(Prs3d_DatumAxes::Prs3d_DatumAxes_XZAxes.as_u32(), 0x05);
        assert_eq!(Prs3d_DatumAxes::Prs3d_DatumAxes_XYZAxes.as_u32(), 0x07);
    }

    #[test]
    fn old_aliases_equal_new_names() {
        assert_eq!(
            Prs3d_DatumAxes::Prs3d_DA_XAxis,
            Prs3d_DatumAxes::Prs3d_DatumAxes_XAxis
        );
        assert_eq!(
            Prs3d_DatumAxes::Prs3d_DA_XYZAxis,
            Prs3d_DatumAxes::Prs3d_DatumAxes_XYZAxes
        );
    }

    #[test]
    fn bitor_combines_axes() {
        let xy = Prs3d_DatumAxes::Prs3d_DatumAxes_XAxis | Prs3d_DatumAxes::Prs3d_DatumAxes_YAxis;
        assert_eq!(xy, Prs3d_DatumAxes::Prs3d_DatumAxes_XYAxes);
        assert!(xy.has_x());
        assert!(xy.has_y());
        assert!(!xy.has_z());
    }

    #[test]
    fn from_u32_rejects_invalid() {
        assert_eq!(Prs3d_DatumAxes::from_u32(0), None);
        assert_eq!(Prs3d_DatumAxes::from_u32(0x08), None);
        assert!(Prs3d_DatumAxes::from_u32(0x07).is_some());
    }
}
