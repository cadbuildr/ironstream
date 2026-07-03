// FILE: prs3d_datum_mode.rs
// occt: Prs3d_DatumMode

/// Enumeration for Prs3d_DatumMode.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3d_DatumMode {
    Prs3d_DM_WireFrame = 0,
    Prs3d_DM_Shaded = 1,
}

impl Prs3d_DatumMode {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Prs3d_DatumMode::Prs3d_DM_WireFrame),
            0 => Some(Prs3d_DatumMode::Prs3d_DM_Shaded),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_datum_mode_sanity() {
        let v = Prs3d_DatumMode::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}