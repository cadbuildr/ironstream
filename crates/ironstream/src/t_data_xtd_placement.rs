// FILE: t_data_xtd_placement.rs
// occt: TDataXtd_Placement

/// Placement attribute - indicates that a label carries a placement
pub struct TDataXtdPlacement;

impl TDataXtdPlacement {
    pub fn new() -> Self {
        TDataXtdPlacement
    }

    pub fn get_id() -> [u8; 16] {
        [0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    }

    pub fn id(&self) -> [u8; 16] {
        Self::get_id()
    }

    pub fn dump(&self) -> &'static str {
        "TDataXtd_Placement"
    }
}

impl Default for TDataXtdPlacement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let placement = TDataXtdPlacement::new();
        assert_eq!(placement.dump(), "TDataXtd_Placement");
    }

    #[test]
    fn test_get_id() {
        let id = TDataXtdPlacement::get_id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_instance_id() {
        let placement = TDataXtdPlacement::new();
        let id = placement.id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_default_trait() {
        let placement = TDataXtdPlacement::default();
        assert_eq!(placement.dump(), "TDataXtd_Placement");
    }

    #[test]
    fn test_id_consistency() {
        let id1 = TDataXtdPlacement::get_id();
        let id2 = TDataXtdPlacement::get_id();
        assert_eq!(id1, id2);
    }
}
