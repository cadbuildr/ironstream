// FILE: step_repr_representation_map.rs
// occt: StepRepr_RepresentationMap

/// Placeholder for RepresentationItem
#[derive(Clone, Debug, PartialEq)]
pub struct RepresentationItem {
    name: String,
}

/// Placeholder for Representation
#[derive(Clone, Debug, PartialEq)]
pub struct Representation {
    name: String,
}

/// Represents a mapping between a representation item (origin) and a representation.
pub struct RepresentationMap {
    mapping_origin: Option<RepresentationItem>,
    mapped_representation: Option<Representation>,
}

impl RepresentationMap {
    /// Create a new RepresentationMap
    pub fn new() -> Self {
        RepresentationMap {
            mapping_origin: None,
            mapped_representation: None,
        }
    }

    /// Initialize mapping with origin and representation
    pub fn init(
        &mut self,
        mapping_origin: RepresentationItem,
        mapped_representation: Representation,
    ) {
        self.mapping_origin = Some(mapping_origin);
        self.mapped_representation = Some(mapped_representation);
    }

    /// Set the mapping origin
    pub fn set_mapping_origin(&mut self, origin: RepresentationItem) {
        self.mapping_origin = Some(origin);
    }

    /// Get the mapping origin
    pub fn mapping_origin(&self) -> Option<&RepresentationItem> {
        self.mapping_origin.as_ref()
    }

    /// Set the mapped representation
    pub fn set_mapped_representation(&mut self, representation: Representation) {
        self.mapped_representation = Some(representation);
    }

    /// Get the mapped representation
    pub fn mapped_representation(&self) -> Option<&Representation> {
        self.mapped_representation.as_ref()
    }
}

impl Default for RepresentationMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let map = RepresentationMap::new();
        assert_eq!(map.mapping_origin(), None);
        assert_eq!(map.mapped_representation(), None);
    }

    #[test]
    fn test_init() {
        let mut map = RepresentationMap::new();
        let origin = RepresentationItem {
            name: "origin".to_string(),
        };
        let rep = Representation {
            name: "representation".to_string(),
        };
        map.init(origin.clone(), rep.clone());
        assert_eq!(map.mapping_origin(), Some(&origin));
        assert_eq!(map.mapped_representation(), Some(&rep));
    }

    #[test]
    fn test_set_and_get_origin() {
        let mut map = RepresentationMap::new();
        let origin = RepresentationItem {
            name: "test_origin".to_string(),
        };
        map.set_mapping_origin(origin.clone());
        assert_eq!(map.mapping_origin(), Some(&origin));
    }

    #[test]
    fn test_set_and_get_representation() {
        let mut map = RepresentationMap::new();
        let rep = Representation {
            name: "test_rep".to_string(),
        };
        map.set_mapped_representation(rep.clone());
        assert_eq!(map.mapped_representation(), Some(&rep));
    }
}
