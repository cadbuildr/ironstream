// FILE: iges_basic_subfigure_def.rs
// occt: IGESBasic_SubfigureDef

/// SubfigureDef, Type <308> Form <0>
/// This Entity permits a single definition of a detail to
/// be utilized in multiple instances in the creation of the whole picture.
pub struct IgesBasicSubfigureDef {
    depth: i32,
    name: String,
    associated_entities: Vec<String>,
}

impl IgesBasicSubfigureDef {
    /// Create a new SubfigureDef with default values.
    pub fn new() -> Self {
        Self {
            depth: 0,
            name: String::new(),
            associated_entities: Vec::new(),
        }
    }

    /// Set the fields of the class SubfigureDef.
    /// - depth: It indicates the amount of nesting
    /// - name: the subfigure name
    /// - all_assoc_entities: the associated entities
    pub fn init(&mut self, depth: i32, name: String, all_assoc_entities: Vec<String>) {
        self.depth = depth;
        self.name = name;
        self.associated_entities = all_assoc_entities;
    }

    /// Returns depth of the Subfigure.
    /// If depth = 0 - No reference to any subfigure instance.
    pub fn depth(&self) -> i32 {
        self.depth
    }

    /// Returns the name of Subfigure.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns number of entities. Is greater than or equal to zero.
    pub fn nb_entities(&self) -> i32 {
        self.associated_entities.len() as i32
    }

    /// Returns the specific entity as indicated by Index.
    /// Raises exception if Index <= 0 or Index > NbEntities().
    pub fn associated_entity(&self, index: i32) -> Option<&str> {
        if index <= 0 || index > self.nb_entities() {
            return None;
        }
        Some(&self.associated_entities[(index - 1) as usize])
    }

    /// Returns the specific entity as indicated by Index.
    /// Raises exception if Index <= 0 or Index > NbEntities().
    pub fn value(&self, index: i32) -> Option<&str> {
        self.associated_entity(index)
    }
}

impl Default for IgesBasicSubfigureDef {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sfd = IgesBasicSubfigureDef::new();
        assert_eq!(sfd.depth(), 0);
        assert_eq!(sfd.name(), "");
        assert_eq!(sfd.nb_entities(), 0);
    }

    #[test]
    fn test_init() {
        let mut sfd = IgesBasicSubfigureDef::new();
        let entities = vec!["entity1".to_string(), "entity2".to_string()];
        sfd.init(1, "subfig".to_string(), entities);
        assert_eq!(sfd.depth(), 1);
        assert_eq!(sfd.name(), "subfig");
        assert_eq!(sfd.nb_entities(), 2);
        assert_eq!(sfd.associated_entity(1), Some("entity1"));
        assert_eq!(sfd.associated_entity(2), Some("entity2"));
    }

    #[test]
    fn test_boundary_checks() {
        let mut sfd = IgesBasicSubfigureDef::new();
        let entities = vec!["entity1".to_string()];
        sfd.init(1, "subfig".to_string(), entities);
        assert_eq!(sfd.associated_entity(0), None);
        assert_eq!(sfd.associated_entity(2), None);
    }

    #[test]
    fn test_value() {
        let mut sfd = IgesBasicSubfigureDef::new();
        let entities = vec!["entity1".to_string()];
        sfd.init(1, "subfig".to_string(), entities);
        assert_eq!(sfd.value(1), Some("entity1"));
        assert_eq!(sfd.value(2), None);
    }

    #[test]
    fn test_multiple_entities() {
        let mut sfd = IgesBasicSubfigureDef::new();
        let entities = vec![
            "entity1".to_string(),
            "entity2".to_string(),
            "entity3".to_string(),
        ];
        sfd.init(2, "complex_subfig".to_string(), entities);
        assert_eq!(sfd.depth(), 2);
        assert_eq!(sfd.nb_entities(), 3);
        assert_eq!(sfd.associated_entity(1), Some("entity1"));
        assert_eq!(sfd.associated_entity(3), Some("entity3"));
    }

    #[test]
    fn test_default() {
        let sfd = IgesBasicSubfigureDef::default();
        assert_eq!(sfd.depth(), 0);
        assert_eq!(sfd.name(), "");
        assert_eq!(sfd.nb_entities(), 0);
    }
}
