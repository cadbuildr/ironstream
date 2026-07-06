// FILE: prs3d_datum_parts.rs
// occt: Prs3d_DatumParts

//! Enumeration defining a part of datum aspect, see Prs3d_Datum.
//! The old Prs3d_DP_* names are aliases of the new ones, expressed
//! as associated constants (Rust enums cannot repeat discriminants).

#![allow(non_upper_case_globals)]

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3d_DatumParts {
    Prs3d_DatumParts_Origin = 0,
    Prs3d_DatumParts_XAxis = 1,
    Prs3d_DatumParts_YAxis = 2,
    Prs3d_DatumParts_ZAxis = 3,
    Prs3d_DatumParts_XArrow = 4,
    Prs3d_DatumParts_YArrow = 5,
    Prs3d_DatumParts_ZArrow = 6,
    Prs3d_DatumParts_XOYAxis = 7,
    Prs3d_DatumParts_YOZAxis = 8,
    Prs3d_DatumParts_XOZAxis = 9,
    Prs3d_DatumParts_None = 10,
}

/// Prs3d_DatumParts_NB = Prs3d_DatumParts_None + 1
pub const Prs3d_DatumParts_NB: u32 = Prs3d_DatumParts::Prs3d_DatumParts_None.as_u32() + 1;

impl Prs3d_DatumParts {
    // old aliases
    pub const Prs3d_DP_Origin: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_Origin;
    pub const Prs3d_DP_XAxis: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_XAxis;
    pub const Prs3d_DP_YAxis: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_YAxis;
    pub const Prs3d_DP_ZAxis: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_ZAxis;
    pub const Prs3d_DP_XArrow: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_XArrow;
    pub const Prs3d_DP_YArrow: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_YArrow;
    pub const Prs3d_DP_ZArrow: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_ZArrow;
    pub const Prs3d_DP_XOYAxis: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_XOYAxis;
    pub const Prs3d_DP_YOZAxis: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_YOZAxis;
    pub const Prs3d_DP_XOZAxis: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_XOZAxis;
    pub const Prs3d_DP_None: Prs3d_DatumParts = Prs3d_DatumParts::Prs3d_DatumParts_None;

    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Prs3d_DatumParts::Prs3d_DatumParts_Origin),
            1 => Some(Prs3d_DatumParts::Prs3d_DatumParts_XAxis),
            2 => Some(Prs3d_DatumParts::Prs3d_DatumParts_YAxis),
            3 => Some(Prs3d_DatumParts::Prs3d_DatumParts_ZAxis),
            4 => Some(Prs3d_DatumParts::Prs3d_DatumParts_XArrow),
            5 => Some(Prs3d_DatumParts::Prs3d_DatumParts_YArrow),
            6 => Some(Prs3d_DatumParts::Prs3d_DatumParts_ZArrow),
            7 => Some(Prs3d_DatumParts::Prs3d_DatumParts_XOYAxis),
            8 => Some(Prs3d_DatumParts::Prs3d_DatumParts_YOZAxis),
            9 => Some(Prs3d_DatumParts::Prs3d_DatumParts_XOZAxis),
            10 => Some(Prs3d_DatumParts::Prs3d_DatumParts_None),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_datum_parts_sanity() {
        let v = Prs3d_DatumParts::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
        assert_eq!(v, Prs3d_DatumParts::Prs3d_DatumParts_Origin);
    }

    #[test]
    fn all_values_roundtrip() {
        for i in 0..=10 {
            let v = Prs3d_DatumParts::from_u32(i).unwrap();
            assert_eq!(v.as_u32(), i);
        }
        assert_eq!(Prs3d_DatumParts::from_u32(11), None);
    }

    #[test]
    fn old_aliases_equal_new_names() {
        assert_eq!(
            Prs3d_DatumParts::Prs3d_DP_Origin,
            Prs3d_DatumParts::Prs3d_DatumParts_Origin
        );
        assert_eq!(
            Prs3d_DatumParts::Prs3d_DP_XArrow,
            Prs3d_DatumParts::Prs3d_DatumParts_XArrow
        );
        assert_eq!(
            Prs3d_DatumParts::Prs3d_DP_None,
            Prs3d_DatumParts::Prs3d_DatumParts_None
        );
    }

    #[test]
    fn nb_constant_matches_occt() {
        assert_eq!(Prs3d_DatumParts_NB, 11);
    }
}
