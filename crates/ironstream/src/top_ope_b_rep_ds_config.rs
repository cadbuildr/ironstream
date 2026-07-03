// FILE: top_ope_b_rep_ds_config.rs
// occt: TopOpeBRepDS_Config

/// Configuration of geometry orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Config {
    /// Unshaped geometry
    UnshGeometry = 0,
    /// Same orientation
    SameOriented = 1,
    /// Different orientation
    DiffOriented = 2,
}

impl Config {
    /// Convert from u8 value
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Config::UnshGeometry),
            1 => Some(Config::SameOriented),
            2 => Some(Config::DiffOriented),
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
    fn test_config_variants() {
        assert_eq!(Config::UnshGeometry.as_u8(), 0);
        assert_eq!(Config::SameOriented.as_u8(), 1);
        assert_eq!(Config::DiffOriented.as_u8(), 2);
    }

    #[test]
    fn test_config_from_u8() {
        assert_eq!(Config::from_u8(0), Some(Config::UnshGeometry));
        assert_eq!(Config::from_u8(1), Some(Config::SameOriented));
        assert_eq!(Config::from_u8(2), Some(Config::DiffOriented));
        assert_eq!(Config::from_u8(3), None);
    }

    #[test]
    fn test_config_clone_copy() {
        let c = Config::SameOriented;
        let c2 = c;
        assert_eq!(c, c2);
    }
}
