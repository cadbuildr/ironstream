// FILE: prs3d_datum_axes.rs
// occt: Prs3d_DatumAxes

/// Enumeration for Prs3d_DatumAxes.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3d_DatumAxes {
    Prs3d_DatumAxes_XAxis = 0x01,
    Prs3d_DatumAxes_YAxis = 0x02,
    Prs3d_DatumAxes_ZAxis = 0x04,
    Prs3d_DatumAxes_XYAxes = Prs3d_DatumAxes_XAxis | Prs3d_DatumAxes_YAxis,
    Prs3d_DatumAxes_YZAxes = Prs3d_DatumAxes_YAxis | Prs3d_DatumAxes_ZAxis,
    Prs3d_DatumAxes_XZAxes = Prs3d_DatumAxes_XAxis | Prs3d_DatumAxes_ZAxis,
    Prs3d_DatumAxes_XYZAxes = Prs3d_DatumAxes_XAxis | Prs3d_DatumAxes_YAxis | Prs3d_DatumAxes_ZAxis,
    Prs3d_DA_XAxis = Prs3d_DatumAxes_XAxis,
    Prs3d_DA_YAxis = Prs3d_DatumAxes_YAxis,
    Prs3d_DA_ZAxis = Prs3d_DatumAxes_ZAxis,
    Prs3d_DA_XYAxis = Prs3d_DatumAxes_XYAxes,
    Prs3d_DA_YZAxis = Prs3d_DatumAxes_YZAxes,
    Prs3d_DA_XZAxis = Prs3d_DatumAxes_XZAxes,
    Prs3d_DA_XYZAxis = Prs3d_DatumAxes_XYZAxes,
}

impl Prs3d_DatumAxes {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0x01 => Some(Prs3d_DatumAxes::Prs3d_DatumAxes_XAxis),
            0x02 => Some(Prs3d_DatumAxes::Prs3d_DatumAxes_YAxis),
            0x04 => Some(Prs3d_DatumAxes::Prs3d_DatumAxes_ZAxis),
            Prs3d_DatumAxes_XAxis | Prs3d_DatumAxes_YAxis => Some(Prs3d_DatumAxes::Prs3d_DatumAxes_XYAxes),
            Prs3d_DatumAxes_YAxis | Prs3d_DatumAxes_ZAxis => Some(Prs3d_DatumAxes::Prs3d_DatumAxes_YZAxes),
            Prs3d_DatumAxes_XAxis | Prs3d_DatumAxes_ZAxis => Some(Prs3d_DatumAxes::Prs3d_DatumAxes_XZAxes),
            Prs3d_DatumAxes_XAxis | Prs3d_DatumAxes_YAxis | Prs3d_DatumAxes_ZAxis => Some(Prs3d_DatumAxes::Prs3d_DatumAxes_XYZAxes),
            Prs3d_DatumAxes_XAxis => Some(Prs3d_DatumAxes::Prs3d_DA_XAxis),
            Prs3d_DatumAxes_YAxis => Some(Prs3d_DatumAxes::Prs3d_DA_YAxis),
            Prs3d_DatumAxes_ZAxis => Some(Prs3d_DatumAxes::Prs3d_DA_ZAxis),
            Prs3d_DatumAxes_XYAxes => Some(Prs3d_DatumAxes::Prs3d_DA_XYAxis),
            Prs3d_DatumAxes_YZAxes => Some(Prs3d_DatumAxes::Prs3d_DA_YZAxis),
            Prs3d_DatumAxes_XZAxes => Some(Prs3d_DatumAxes::Prs3d_DA_XZAxis),
            Prs3d_DatumAxes_XYZAxes => Some(Prs3d_DatumAxes::Prs3d_DA_XYZAxis),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_datum_axes_sanity() {
        let v = Prs3d_DatumAxes::from_u32(0x01).unwrap();
        assert_eq!(v.as_u32(), 0x01);
    }
}