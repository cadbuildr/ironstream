// FILE: iges_data_default_specific.rs
// occt: IGESData_DefaultSpecific

//! Specific IGES Services for UndefinedEntity, FreeFormatEntity.
//! Provides dump functionality for undefined IGES entities.

/// IGESData_DefaultSpecific handles default specific module behavior
/// for undefined IGES entities and free format entities.
#[derive(Clone, Debug)]
pub struct DefaultSpecific;

impl DefaultSpecific {
    /// Creates a DefaultSpecific and puts it into SpecificLib
    pub fn new() -> Self {
        DefaultSpecific
    }

    /// Specific Dump for UndefinedEntity and FreeFormatEntity
    /// It concerns only own parameters; the general data (Directory Part, Lists)
    /// are taken into account by the IGESDumper
    pub fn own_dump(&self, cn: i32, own: i32) -> String {
        format!("DefaultSpecific dump for case {} with own={}", cn, own)
    }
}

impl Default for DefaultSpecific {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ds = DefaultSpecific::new();
        let ds2 = DefaultSpecific::default();
        assert_eq!(format!("{:?}", ds), format!("{:?}", ds2));
    }

    #[test]
    fn test_own_dump() {
        let ds = DefaultSpecific::new();
        let dump1 = ds.own_dump(1, 0);
        assert!(dump1.contains("case 1"));
        assert!(dump1.contains("own=0"));

        let dump2 = ds.own_dump(2, 42);
        assert!(dump2.contains("case 2"));
        assert!(dump2.contains("own=42"));
    }
}
