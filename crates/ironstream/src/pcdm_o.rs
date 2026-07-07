// FILE: pcdm_o.rs
// occt: PCDM

/// PCDM (Persistent Component Data Model) utility class
pub struct PCDM;

impl PCDM {
    /// Determine the file driver type
    pub fn file_driver_type(file_name: &str) -> i32 {
        if file_name.ends_with(".xml") {
            3 // PCDM_TOFD_XmlFile
        } else if file_name.ends_with(".cmp") {
            1 // PCDM_TOFD_CmpFile
        } else if file_name.ends_with(".std") || file_name.ends_with(".stp") {
            0 // PCDM_TOFD_File
        } else {
            3 // PCDM_TOFD_Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_file_type() {
        assert_eq!(PCDM::file_driver_type("document.xml"), 3);
    }

    #[test]
    fn test_cmp_file_type() {
        assert_eq!(PCDM::file_driver_type("document.cmp"), 1);
    }

    #[test]
    fn test_std_file_type() {
        assert_eq!(PCDM::file_driver_type("document.std"), 0);
    }

    #[test]
    fn test_unknown_file_type() {
        assert_eq!(PCDM::file_driver_type("document.xyz"), 3);
    }
}
