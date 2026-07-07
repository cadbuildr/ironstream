// FILE: iges_defs_generic_data.rs
// occt: IGESDefs_GenericData

//! Generic data entity for IGES definitions.

#[derive(Clone, Debug)]
pub struct GenericData {
    data: Vec<String>,
}

impl GenericData {
    pub fn new() -> Self {
        GenericData {
            data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, item: &str) {
        self.data.push(item.to_string());
    }

    pub fn data(&self) -> &[String] {
        &self.data
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }
}

impl Default for GenericData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = GenericData::new();
        assert_eq!(data.count(), 0);
    }

    #[test]
    fn test_add_data() {
        let mut data = GenericData::new();
        data.add_data("item1");
        data.add_data("item2");
        assert_eq!(data.count(), 2);
    }
}
