// FILE: iges_appli_reference_designator.rs
// occt: IGESAppli_ReferenceDesignator

/// Represents component reference designator (e.g., R1, C2, U3).
///
/// IGES Type 406 Form 7
/// Identifies component positions on a PCB or assembly.
#[derive(Clone, Debug)]
pub struct IgesAppliReferenceDesignator {
    designator: String,
}

impl IgesAppliReferenceDesignator {
    /// Creates a new ReferenceDesignator entity.
    pub fn new() -> Self {
        Self {
            designator: String::new(),
        }
    }

    /// Initializes with the designator string (e.g., "R1", "U5").
    pub fn init(&mut self, desig: String) {
        self.designator = desig;
    }

    /// Returns the designator string.
    pub fn designator(&self) -> &str {
        &self.designator
    }
}

impl Default for IgesAppliReferenceDesignator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let desig = IgesAppliReferenceDesignator::new();
        assert_eq!(desig.designator(), "");
    }

    #[test]
    fn test_init() {
        let mut desig = IgesAppliReferenceDesignator::new();
        desig.init("R42".to_string());

        assert_eq!(desig.designator(), "R42");
    }

    #[test]
    fn test_clone() {
        let mut desig1 = IgesAppliReferenceDesignator::new();
        desig1.init("U7".to_string());

        let desig2 = desig1.clone();
        assert_eq!(desig2.designator(), "U7");
    }
}
