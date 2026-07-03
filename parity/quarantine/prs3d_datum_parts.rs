// FILE: prs3d_datum_parts.rs
// occt: Prs3d_DatumParts

/// Enumeration for Prs3d_DatumParts.
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
    Prs3d_DP_Origin = Prs3d_DatumParts_Origin,
    Prs3d_DP_XAxis = Prs3d_DatumParts_XAxis,
    Prs3d_DP_YAxis = Prs3d_DatumParts_YAxis,
    Prs3d_DP_ZAxis = Prs3d_DatumParts_ZAxis,
    Prs3d_DP_XArrow = Prs3d_DatumParts_XArrow,
    Prs3d_DP_YArrow = Prs3d_DatumParts_YArrow,
    Prs3d_DP_ZArrow = Prs3d_DatumParts_ZArrow,
    Prs3d_DP_XOYAxis = Prs3d_DatumParts_XOYAxis,
    Prs3d_DP_YOZAxis = Prs3d_DatumParts_YOZAxis,
    Prs3d_DP_XOZAxis = Prs3d_DatumParts_XOZAxis,
    Prs3d_DP_None = Prs3d_DatumParts_None,
}

impl Prs3d_DatumParts {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Prs3d_DatumParts::Prs3d_DatumParts_Origin),
            0 => Some(Prs3d_DatumParts::Prs3d_DatumParts_XAxis),
            1 => Some(Prs3d_DatumParts::Prs3d_DatumParts_YAxis),
            2 => Some(Prs3d_DatumParts::Prs3d_DatumParts_ZAxis),
            3 => Some(Prs3d_DatumParts::Prs3d_DatumParts_XArrow),
            4 => Some(Prs3d_DatumParts::Prs3d_DatumParts_YArrow),
            5 => Some(Prs3d_DatumParts::Prs3d_DatumParts_ZArrow),
            6 => Some(Prs3d_DatumParts::Prs3d_DatumParts_XOYAxis),
            7 => Some(Prs3d_DatumParts::Prs3d_DatumParts_YOZAxis),
            8 => Some(Prs3d_DatumParts::Prs3d_DatumParts_XOZAxis),
            9 => Some(Prs3d_DatumParts::Prs3d_DatumParts_None),
            Prs3d_DatumParts_Origin => Some(Prs3d_DatumParts::Prs3d_DP_Origin),
            Prs3d_DatumParts_XAxis => Some(Prs3d_DatumParts::Prs3d_DP_XAxis),
            Prs3d_DatumParts_YAxis => Some(Prs3d_DatumParts::Prs3d_DP_YAxis),
            Prs3d_DatumParts_ZAxis => Some(Prs3d_DatumParts::Prs3d_DP_ZAxis),
            Prs3d_DatumParts_XArrow => Some(Prs3d_DatumParts::Prs3d_DP_XArrow),
            Prs3d_DatumParts_YArrow => Some(Prs3d_DatumParts::Prs3d_DP_YArrow),
            Prs3d_DatumParts_ZArrow => Some(Prs3d_DatumParts::Prs3d_DP_ZArrow),
            Prs3d_DatumParts_XOYAxis => Some(Prs3d_DatumParts::Prs3d_DP_XOYAxis),
            Prs3d_DatumParts_YOZAxis => Some(Prs3d_DatumParts::Prs3d_DP_YOZAxis),
            Prs3d_DatumParts_XOZAxis => Some(Prs3d_DatumParts::Prs3d_DP_XOZAxis),
            Prs3d_DatumParts_None => Some(Prs3d_DatumParts::Prs3d_DP_None),
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
    }
}