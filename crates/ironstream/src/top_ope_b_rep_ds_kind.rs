// FILE: top_ope_b_rep_ds_kind.rs
// occt: TopOpeBRepDS_Kind

/// Different types of objects in DataStructure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Kind {
    /// Point geometry
    Point = 0,
    /// Curve geometry
    Curve = 1,
    /// Surface geometry
    Surface = 2,
    /// Vertex topology
    Vertex = 3,
    /// Edge topology
    Edge = 4,
    /// Wire topology
    Wire = 5,
    /// Face topology
    Face = 6,
    /// Shell topology
    Shell = 7,
    /// Solid topology
    Solid = 8,
    /// Compound solid topology
    CompSolid = 9,
    /// Compound topology
    Compound = 10,
    /// Unknown type
    Unknown = 11,
}

impl Kind {
    /// Convert from u8 value
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Kind::Point),
            1 => Some(Kind::Curve),
            2 => Some(Kind::Surface),
            3 => Some(Kind::Vertex),
            4 => Some(Kind::Edge),
            5 => Some(Kind::Wire),
            6 => Some(Kind::Face),
            7 => Some(Kind::Shell),
            8 => Some(Kind::Solid),
            9 => Some(Kind::CompSolid),
            10 => Some(Kind::Compound),
            11 => Some(Kind::Unknown),
            _ => None,
        }
    }

    /// Convert to u8 value
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_variants() {
        assert_eq!(Kind::Point.as_u8(), 0);
        assert_eq!(Kind::Curve.as_u8(), 1);
        assert_eq!(Kind::Surface.as_u8(), 2);
        assert_eq!(Kind::Vertex.as_u8(), 3);
        assert_eq!(Kind::Edge.as_u8(), 4);
        assert_eq!(Kind::Wire.as_u8(), 5);
        assert_eq!(Kind::Face.as_u8(), 6);
        assert_eq!(Kind::Shell.as_u8(), 7);
        assert_eq!(Kind::Solid.as_u8(), 8);
        assert_eq!(Kind::CompSolid.as_u8(), 9);
        assert_eq!(Kind::Compound.as_u8(), 10);
        assert_eq!(Kind::Unknown.as_u8(), 11);
    }

    #[test]
    fn test_kind_from_u8() {
        assert_eq!(Kind::from_u8(0), Some(Kind::Point));
        assert_eq!(Kind::from_u8(1), Some(Kind::Curve));
        assert_eq!(Kind::from_u8(2), Some(Kind::Surface));
        assert_eq!(Kind::from_u8(3), Some(Kind::Vertex));
        assert_eq!(Kind::from_u8(4), Some(Kind::Edge));
        assert_eq!(Kind::from_u8(11), Some(Kind::Unknown));
        assert_eq!(Kind::from_u8(255), None);
    }

    #[test]
    fn test_kind_ordering() {
        assert!(Kind::Point < Kind::Curve);
        assert!(Kind::Surface < Kind::Vertex);
        assert!(Kind::Unknown > Kind::Compound);
    }

    #[test]
    fn test_kind_clone_copy() {
        let k = Kind::Face;
        let k2 = k;
        assert_eq!(k, k2);
        assert_eq!(k, Kind::Face);
    }
}
