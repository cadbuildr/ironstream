// FILE: step_shape_dimensional_characteristic.rs
// occt: StepShape_DimensionalCharacteristic

//! Representation of STEP SELECT type DimensionalCharacteristic

#[derive(Clone, Debug)]
pub enum DimensionalCharacteristic {
    /// DimensionalLocation
    DimensionalLocation(String),
    /// DimensionalSize
    DimensionalSize(String),
}

impl DimensionalCharacteristic {
    /// Empty constructor
    pub fn new() -> Option<Self> {
        None
    }

    /// Recognizes a kind of DimensionalCharacteristic select type
    /// 1 -> DimensionalLocation from StepShape
    /// 2 -> DimensionalSize from StepShape
    /// 0 else
    pub fn case_num(entity_type: &str) -> i32 {
        match entity_type {
            "DimensionalLocation" => 1,
            "DimensionalSize" => 2,
            _ => 0,
        }
    }

    /// Returns Value as DimensionalLocation (or None if another type)
    pub fn dimensional_location(&self) -> Option<&str> {
        if let DimensionalCharacteristic::DimensionalLocation(dl) = self {
            Some(dl)
        } else {
            None
        }
    }

    /// Returns Value as DimensionalSize (or None if another type)
    pub fn dimensional_size(&self) -> Option<&str> {
        if let DimensionalCharacteristic::DimensionalSize(ds) = self {
            Some(ds)
        } else {
            None
        }
    }
}

impl Default for DimensionalCharacteristic {
    fn default() -> Self {
        DimensionalCharacteristic::DimensionalLocation(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num() {
        assert_eq!(DimensionalCharacteristic::case_num("DimensionalLocation"), 1);
        assert_eq!(DimensionalCharacteristic::case_num("DimensionalSize"), 2);
        assert_eq!(DimensionalCharacteristic::case_num("Unknown"), 0);
    }

    #[test]
    fn test_dimensional_location() {
        let dc = DimensionalCharacteristic::DimensionalLocation("loc1".to_string());
        assert_eq!(dc.dimensional_location(), Some("loc1"));
        assert!(dc.dimensional_size().is_none());
    }

    #[test]
    fn test_dimensional_size() {
        let dc = DimensionalCharacteristic::DimensionalSize("size1".to_string());
        assert_eq!(dc.dimensional_size(), Some("size1"));
        assert!(dc.dimensional_location().is_none());
    }
}
