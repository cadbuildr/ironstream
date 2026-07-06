// FILE: xml_t_obj_drivers_int_sparse_array_driver.rs
// occt: XmlTObjDrivers_IntSparseArrayDriver

use std::collections::BTreeMap;

/// XML driver for sparse integer arrays in TObj persistence.
/// Handles compact serialization/deserialization of sparse integer data
/// by encoding only non-zero or non-default values with their indices.
pub struct XmlTObjDriversIntSparseArrayDriver {
    version: i32,
}

impl XmlTObjDriversIntSparseArrayDriver {
    /// Create a new sparse array driver.
    pub fn new() -> Self {
        XmlTObjDriversIntSparseArrayDriver { version: 1 }
    }

    /// Get the driver version.
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Convert sparse integer array to XML representation.
    /// Encodes (index, value) pairs for non-zero elements.
    pub fn write_sparse_array(&self, sparse_map: &BTreeMap<usize, i32>) -> String {
        let mut xml = String::from("<IntSparseArray>");
        for (&index, &value) in sparse_map.iter() {
            xml.push_str(&format!("<Entry idx=\"{}\" val=\"{}\"/>", index, value));
        }
        xml.push_str("</IntSparseArray>");
        xml
    }

    /// Read sparse integer array from XML representation.
    /// Reconstructs the (index, value) map from XML entries.
    pub fn read_sparse_array(&self, xml: &str) -> BTreeMap<usize, i32> {
        let mut map = BTreeMap::new();

        for line in xml.lines() {
            if line.contains("<Entry") {
                let mut idx = None;
                let mut val = None;

                for part in line.split_whitespace() {
                    if let Some(i_str) = part.strip_prefix("idx=\"").and_then(|s| s.strip_suffix("\"")) {
                        idx = i_str.parse::<usize>().ok();
                    } else if let Some(v_str) = part.strip_prefix("val=\"").and_then(|s| s.strip_suffix("\"")) {
                        val = v_str.parse::<i32>().ok();
                    }
                }

                if let (Some(i), Some(v)) = (idx, val) {
                    map.insert(i, v);
                }
            }
        }
        map
    }

    /// Calculate compression ratio: actual_size / estimated_dense_size.
    /// Returns ratio < 1.0 for truly sparse data.
    pub fn compression_ratio(&self, sparse_count: usize, dense_capacity: usize) -> f64 {
        if dense_capacity == 0 {
            return 1.0;
        }
        sparse_count as f64 / dense_capacity as f64
    }

    /// Estimate XML size for given sparse array size.
    pub fn estimate_xml_size(&self, sparse_count: usize) -> usize {
        // Rough estimate: ~60 bytes per entry (XML tags, attributes)
        60 * sparse_count + 30 // Header/footer overhead
    }

    /// Validate sparse array structure: ensure indices are sorted and non-empty.
    pub fn validate_sparse_array(&self, sparse_map: &BTreeMap<usize, i32>) -> Result<(), String> {
        if sparse_map.is_empty() {
            return Err("Empty sparse array".to_string());
        }
        Ok(())
    }
}

impl Default for XmlTObjDriversIntSparseArrayDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_version() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        assert_eq!(driver.version(), 1);
    }

    #[test]
    fn test_write_sparse_array_empty() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let map = BTreeMap::new();
        let xml = driver.write_sparse_array(&map);
        assert_eq!(xml, "<IntSparseArray></IntSparseArray>");
    }

    #[test]
    fn test_write_sparse_array_single_entry() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let mut map = BTreeMap::new();
        map.insert(5, 42);
        let xml = driver.write_sparse_array(&map);
        assert!(xml.contains("idx=\"5\""));
        assert!(xml.contains("val=\"42\""));
    }

    #[test]
    fn test_write_sparse_array_multiple_entries() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let mut map = BTreeMap::new();
        map.insert(1, 10);
        map.insert(5, 50);
        map.insert(10, 100);
        let xml = driver.write_sparse_array(&map);
        assert!(xml.contains("idx=\"1\""));
        assert!(xml.contains("idx=\"5\""));
        assert!(xml.contains("idx=\"10\""));
    }

    #[test]
    fn test_read_sparse_array_empty() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let map = driver.read_sparse_array("<IntSparseArray></IntSparseArray>");
        assert!(map.is_empty());
    }

    #[test]
    fn test_read_sparse_array_single_entry() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let xml = "<IntSparseArray><Entry idx=\"5\" val=\"42\"/></IntSparseArray>";
        let map = driver.read_sparse_array(xml);
        assert_eq!(map.get(&5), Some(&42));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_roundtrip_multiple_entries() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let mut orig = BTreeMap::new();
        orig.insert(0, 100);
        orig.insert(3, 200);
        orig.insert(7, 300);

        let xml = driver.write_sparse_array(&orig);
        let restored = driver.read_sparse_array(&xml);

        assert_eq!(restored, orig);
    }

    #[test]
    fn test_compression_ratio_very_sparse() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let ratio = driver.compression_ratio(10, 1000);
        assert!(ratio < 0.1);
    }

    #[test]
    fn test_compression_ratio_dense() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let ratio = driver.compression_ratio(900, 1000);
        assert!(ratio > 0.8);
    }

    #[test]
    fn test_estimate_xml_size() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let size = driver.estimate_xml_size(5);
        assert!(size > 0);
        assert!(size > 30);
    }

    #[test]
    fn test_validate_sparse_array_empty() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let map = BTreeMap::new();
        assert!(driver.validate_sparse_array(&map).is_err());
    }

    #[test]
    fn test_validate_sparse_array_valid() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let mut map = BTreeMap::new();
        map.insert(0, 42);
        assert!(driver.validate_sparse_array(&map).is_ok());
    }
}
