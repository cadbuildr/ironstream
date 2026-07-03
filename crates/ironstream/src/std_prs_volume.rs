// FILE: std_prs_volume.rs
// occt: StdPrs_Volume

//! Defines how to interpret input shapes for visualization.
//! - Volume_Autodetection: perform auto-detection (splits input shape into closed/open groups)
//! - Volume_Closed: as closed volumes (enables back-face culling and capping plane algorithms)
//! - Volume_Opened: as open volumes (shells or solids with holes)

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Volume {
    #[default]
    Autodetection,
    Closed,
    Opened,
}

impl Volume {
    pub fn as_u32(&self) -> u32 {
        match self {
            Volume::Autodetection => 0,
            Volume::Closed => 1,
            Volume::Opened => 2,
        }
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Volume::Autodetection),
            1 => Some(Volume::Closed),
            2 => Some(Volume::Opened),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_as_u32() {
        assert_eq!(Volume::Autodetection.as_u32(), 0);
        assert_eq!(Volume::Closed.as_u32(), 1);
        assert_eq!(Volume::Opened.as_u32(), 2);
    }

    #[test]
    fn volume_from_u32() {
        assert_eq!(Volume::from_u32(0), Some(Volume::Autodetection));
        assert_eq!(Volume::from_u32(1), Some(Volume::Closed));
        assert_eq!(Volume::from_u32(2), Some(Volume::Opened));
        assert_eq!(Volume::from_u32(99), None);
    }
}
