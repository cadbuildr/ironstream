// FILE: prs3d_vertex_draw_mode.rs
// occt: Prs3d_VertexDrawMode

/// Enumeration for Prs3d_VertexDrawMode.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Prs3d_VertexDrawMode {
    Prs3d_VDM_Isolated = 0,
    Prs3d_VDM_All = 1,
    Prs3d_VDM_Inherited = 2,
}

impl Prs3d_VertexDrawMode {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Prs3d_VertexDrawMode::Prs3d_VDM_Isolated),
            1 => Some(Prs3d_VertexDrawMode::Prs3d_VDM_All),
            2 => Some(Prs3d_VertexDrawMode::Prs3d_VDM_Inherited),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_vertex_draw_mode_sanity() {
        let v = Prs3d_VertexDrawMode::from_u32(0).unwrap();
        assert_eq!(v.as_u32(), 0);
    }
}