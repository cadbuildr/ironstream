// FILE: bin_mdf_type_a_driver_map.rs
// occt: BinMDF_TypeADriverMap

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Driver {
    driver_id: usize,
    driver_type: String,
}

impl Driver {
    pub fn new(driver_id: usize, driver_type: String) -> Self {
        Driver { driver_id, driver_type }
    }
}

pub struct BinmdfTypeADriverMap {
    data: HashMap<String, Driver>,
}

impl BinmdfTypeADriverMap {
    pub fn new() -> Self {
        BinmdfTypeADriverMap {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, type_name: String, driver: Driver) {
        self.data.insert(type_name, driver);
    }

    pub fn get(&self, type_name: &str) -> Option<&Driver> {
        self.data.get(type_name)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for BinmdfTypeADriverMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = Driver::new(1, "int".to_string());
        assert_eq!(driver.driver_id, 1);
    }

    #[test]
    fn test_map_add_get() {
        let mut map = BinmdfTypeADriverMap::new();
        let driver = Driver::new(1, "int".to_string());
        map.add("Integer".to_string(), driver);
        assert!(map.get("Integer").is_some());
    }
}
