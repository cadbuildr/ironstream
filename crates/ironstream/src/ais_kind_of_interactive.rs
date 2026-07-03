// FILE: ais_kind_of_interactive.rs
// occt: AIS_KindOfInteractive

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KindOfInteractive {
    None = 0,
    Datum = 1,
    Shape = 2,
    Object = 3,
    Relation = 4,
    Dimension = 5,
    LightSource = 6,
}

impl KindOfInteractive {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(KindOfInteractive::None),
            1 => Some(KindOfInteractive::Datum),
            2 => Some(KindOfInteractive::Shape),
            3 => Some(KindOfInteractive::Object),
            4 => Some(KindOfInteractive::Relation),
            5 => Some(KindOfInteractive::Dimension),
            6 => Some(KindOfInteractive::LightSource),
            _ => None,
        }
    }

    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(KindOfInteractive::None as u32, 0);
    }
}
