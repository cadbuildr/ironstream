// FILE: dnaming.rs
// occt-ref: DNaming_NamedShape, DNaming_Naming, DNaming_Builder

/// Named shape for construction history.
// occt-ref: DNaming_NamedShape
#[derive(Clone, Debug)]
pub struct DNamingNamedShape {
    pub name: String,
    pub shape_id: u32,
    pub naming_type: u8,  // 0=Feature 1=Generated 2=Deleted
}

impl DNamingNamedShape {
    pub fn new(name: impl Into<String>, shape_id: u32, name_type: u8) -> Self {
        Self {
            name: name.into(),
            shape_id,
            naming_type: name_type.min(2),
        }
    }

    pub fn is_feature(&self) -> bool { self.naming_type == 0 }
    pub fn is_generated(&self) -> bool { self.naming_type == 1 }
    pub fn is_deleted(&self) -> bool { self.naming_type == 2 }
}

impl Default for DNamingNamedShape {
    fn default() -> Self {
        Self::new("", 0, 0)
    }
}

/// Naming database: maps shapes to names.
// occt-ref: DNaming_Naming
#[derive(Clone, Debug)]
pub struct DNamingNaming {
    pub label_id: u32,
    pub named_shapes: Vec<DNamingNamedShape>,
}

impl DNamingNaming {
    pub fn new(label_id: u32) -> Self {
        Self {
            label_id,
            named_shapes: Vec::new(),
        }
    }

    pub fn add_name(&mut self, shape_name: DNamingNamedShape) {
        self.named_shapes.push(shape_name);
    }

    pub fn find_shape(&self, name: &str) -> Option<u32> {
        self.named_shapes
            .iter()
            .find(|ns| ns.name == name)
            .map(|ns| ns.shape_id)
    }

    pub fn nb_names(&self) -> usize { self.named_shapes.len() }
}

impl Default for DNamingNaming {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Builder for construction naming.
// occt-ref: DNaming_Builder
#[derive(Clone, Debug)]
pub struct DNamingBuilder {
    pub label_id: u32,
    pub operations: Vec<String>,
}

impl DNamingBuilder {
    pub fn new(label_id: u32) -> Self {
        Self {
            label_id,
            operations: Vec::new(),
        }
    }

    pub fn add_operation(&mut self, op: impl Into<String>) {
        self.operations.push(op.into());
    }

    pub fn build(&self) -> bool { !self.operations.is_empty() }
    pub fn nb_operations(&self) -> usize { self.operations.len() }
}

impl Default for DNamingBuilder {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dnaming_named_shape() {
        let ns = DNamingNamedShape::new("MyShape", 42, 0);
        assert_eq!(ns.name, "MyShape");
        assert_eq!(ns.shape_id, 42);
        assert!(ns.is_feature());
    }

    #[test]
    fn dnaming_naming() {
        let mut n = DNamingNaming::new(1);
        n.add_name(DNamingNamedShape::new("Face1", 10, 1));
        n.add_name(DNamingNamedShape::new("Edge1", 20, 0));
        assert_eq!(n.nb_names(), 2);
        assert_eq!(n.find_shape("Face1"), Some(10));
    }

    #[test]
    fn dnaming_builder() {
        let mut b = DNamingBuilder::new(5);
        b.add_operation("Extrude");
        b.add_operation("Fillet");
        assert!(b.build());
        assert_eq!(b.nb_operations(), 2);
    }

    #[test]
    fn dnaming_naming_types() {
        let feature = DNamingNamedShape::new("F", 1, 0);
        let generated = DNamingNamedShape::new("G", 2, 1);
        let deleted = DNamingNamedShape::new("D", 3, 2);
        assert!(feature.is_feature());
        assert!(generated.is_generated());
        assert!(deleted.is_deleted());
    }

    #[test]
    fn dnaming_lookup() {
        let mut n = DNamingNaming::new(1);
        n.add_name(DNamingNamedShape::new("Base", 100, 0));
        assert!(n.find_shape("Base").is_some());
        assert!(n.find_shape("Nonexistent").is_none());
    }
}
