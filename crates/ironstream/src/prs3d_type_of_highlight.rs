// FILE: prs3d_type_of_highlight.rs
// occt: Prs3d_TypeOfHighlight

/// Enumeration for Prs3d_TypeOfHighlight.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3d_TypeOfHighlight {
    Prs3d_TypeOfHighlight_None = 0,
    Prs3d_TypeOfHighlight_Selected = 1,
    Prs3d_TypeOfHighlight_Dynamic = 2,
    Prs3d_TypeOfHighlight_LocalSelected = 3,
    Prs3d_TypeOfHighlight_LocalDynamic = 4,
    Prs3d_TypeOfHighlight_SubIntensity = 5,
    Prs3d_TypeOfHighlight_NB = 6,
}

impl Prs3d_TypeOfHighlight {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Prs3d_TypeOfHighlight::Prs3d_TypeOfHighlight_None),
            0 => Some(Prs3d_TypeOfHighlight::Prs3d_TypeOfHighlight_Selected),
            1 => Some(Prs3d_TypeOfHighlight::Prs3d_TypeOfHighlight_Dynamic),
            2 => Some(Prs3d_TypeOfHighlight::Prs3d_TypeOfHighlight_LocalSelected),
            3 => Some(Prs3d_TypeOfHighlight::Prs3d_TypeOfHighlight_LocalDynamic),
            4 => Some(Prs3d_TypeOfHighlight::Prs3d_TypeOfHighlight_SubIntensity),
            5 => Some(Prs3d_TypeOfHighlight::Prs3d_TypeOfHighlight_NB),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_type_of_highlight_sanity() {
        let v = Prs3d_TypeOfHighlight::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}