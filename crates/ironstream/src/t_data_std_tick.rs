// FILE: t_data_std_tick.rs
// occt: TDataStd_Tick

/// A Tick attribute - a marker/boolean attribute.
/// If it exists at a label, it represents true.
/// If not present, it represents false.
#[derive(Clone, Debug, Default)]
pub struct TDataStd_Tick {
    id: [u8; 16],
}

impl TDataStd_Tick {
    /// Create a new Tick attribute.
    pub fn new() -> Self {
        Self {
            id: Self::get_id(),
        }
    }

    /// Get the standard GUID for Tick attributes.
    pub fn get_id() -> [u8; 16] {
        // Standard OCCT GUID for TDataStd_Tick
        [
            0x7C, 0x5D, 0x39, 0xF1, 0x4E, 0x76, 0x45, 0x9A, 0x88, 0x9E, 0x6D, 0x33, 0x22, 0x22,
            0x22, 0x22,
        ]
    }

    /// Get the ID of this attribute.
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Check if the tick is set (always true since the attribute exists).
    pub fn is_set(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tick() {
        let tick = TDataStd_Tick::new();
        assert!(tick.is_set());
    }

    #[test]
    fn test_get_id() {
        let id1 = TDataStd_Tick::get_id();
        let id2 = TDataStd_Tick::get_id();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_default() {
        let tick = TDataStd_Tick::default();
        assert!(tick.is_set());
    }

    #[test]
    fn test_id_is_consistent() {
        let tick = TDataStd_Tick::new();
        assert_eq!(tick.id(), &TDataStd_Tick::get_id());
    }
}
