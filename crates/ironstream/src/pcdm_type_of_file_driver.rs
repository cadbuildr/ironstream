// FILE: pcdm_type_of_file_driver.rs
// occt: PCDM_TypeOfFileDriver

/// Type of file driver
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PCDMTypeOfFileDriver {
    File = 0,     // Standard binary file
    CmpFile = 1,  // Compressed file
    XmlFile = 2,  // XML file
    Unknown = 3,  // Unknown type
}

impl Default for PCDMTypeOfFileDriver {
    fn default() -> Self {
        PCDMTypeOfFileDriver::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_driver_values() {
        assert_eq!(PCDMTypeOfFileDriver::File as i32, 0);
        assert_eq!(PCDMTypeOfFileDriver::CmpFile as i32, 1);
        assert_eq!(PCDMTypeOfFileDriver::XmlFile as i32, 2);
        assert_eq!(PCDMTypeOfFileDriver::Unknown as i32, 3);
    }

    #[test]
    fn test_default_type() {
        let typ: PCDMTypeOfFileDriver = Default::default();
        assert_eq!(typ, PCDMTypeOfFileDriver::Unknown);
    }
}
