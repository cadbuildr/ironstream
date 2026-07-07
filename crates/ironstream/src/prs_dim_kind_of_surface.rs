// FILE: prs_dim_kind_of_surface.rs
// occt: PrsDim_KindOfSurface

/// Enumeration for PrsDim_KindOfSurface.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrsDim_KindOfSurface {
    PrsDim_KOS_Plane = 0,
    PrsDim_KOS_Cylinder = 1,
    PrsDim_KOS_Cone = 2,
    PrsDim_KOS_Sphere = 3,
    PrsDim_KOS_Torus = 4,
    PrsDim_KOS_Revolution = 5,
    PrsDim_KOS_Extrusion = 6,
    PrsDim_KOS_OtherSurface = 7,
}

impl PrsDim_KindOfSurface {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PrsDim_KindOfSurface::PrsDim_KOS_Plane),
            1 => Some(PrsDim_KindOfSurface::PrsDim_KOS_Cylinder),
            2 => Some(PrsDim_KindOfSurface::PrsDim_KOS_Cone),
            3 => Some(PrsDim_KindOfSurface::PrsDim_KOS_Sphere),
            4 => Some(PrsDim_KindOfSurface::PrsDim_KOS_Torus),
            5 => Some(PrsDim_KindOfSurface::PrsDim_KOS_Revolution),
            6 => Some(PrsDim_KindOfSurface::PrsDim_KOS_Extrusion),
            7 => Some(PrsDim_KindOfSurface::PrsDim_KOS_OtherSurface),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_dim_kind_of_surface_sanity() {
        let v = PrsDim_KindOfSurface::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}