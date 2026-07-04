// FILE: prs3d_type_of_line_picking.rs
// occt: Prs3d_TypeOfLinePicking

/// Enumeration for Prs3d_TypeOfLinePicking.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3d_TypeOfLinePicking {
    Prs3d_TOLP_Point = 0,
    Prs3d_TOLP_Segment = 1,
}

impl Prs3d_TypeOfLinePicking {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Prs3d_TypeOfLinePicking::Prs3d_TOLP_Point),
            1 => Some(Prs3d_TypeOfLinePicking::Prs3d_TOLP_Segment),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_type_of_line_picking_sanity() {
        let v = Prs3d_TypeOfLinePicking::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}