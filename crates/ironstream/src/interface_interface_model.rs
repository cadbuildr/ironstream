// FILE: interface_interface_model.rs
// occt: Interface_InterfaceModel

use std::sync::Arc;
use std::collections::HashMap;

/// Defines an (Indexed) Set of data corresponding to a complete
/// Transfer by a File Interface
pub struct InterfaceInterfaceModel {
    entities: Vec<Option<Arc<dyn std::any::Any>>>,
    labels: HashMap<usize, String>,
    protocol: Option<Arc<dyn std::any::Any>>,
    gtool: Option<Arc<dyn std::any::Any>>,
    dispatch_status: bool,
}

impl InterfaceInterfaceModel {
    /// Creates a new InterfaceModel
    pub fn new() -> Self {
        InterfaceInterfaceModel {
            entities: Vec::new(),
            labels: HashMap::new(),
            protocol: None,
            gtool: None,
            dispatch_status: false,
        }
    }

    /// Sets a Protocol for this Model
    pub fn set_protocol(&mut self, proto: Arc<dyn std::any::Any>) {
        self.protocol = Some(proto);
    }

    /// Returns the Protocol which has been set
    pub fn protocol(&self) -> Option<Arc<dyn std::any::Any>> {
        self.protocol.clone()
    }

    /// Sets a GTool for this model
    pub fn set_gtool(&mut self, gtool: Arc<dyn std::any::Any>) {
        self.gtool = Some(gtool);
    }

    /// Returns the GTool
    pub fn gtool(&self) -> Option<Arc<dyn std::any::Any>> {
        self.gtool.clone()
    }

    /// Returns the Dispatch Status
    pub fn dispatch_status(&mut self) -> &mut bool {
        &mut self.dispatch_status
    }

    /// Erases contained data
    pub fn clear(&mut self) {
        self.entities.clear();
        self.labels.clear();
    }

    /// Clears the entities
    pub fn clear_entities(&mut self) {
        self.entities.clear();
    }

    /// Returns count of entities
    pub fn nb_entities(&self) -> usize {
        self.entities.len()
    }

    /// Adds an entity to the model
    pub fn add_entity(&mut self, entity: Arc<dyn std::any::Any>) -> usize {
        let index = self.entities.len();
        self.entities.push(Some(entity));
        index
    }

    /// Returns an entity by index
    pub fn entity(&self, index: usize) -> Option<Arc<dyn std::any::Any>> {
        if index < self.entities.len() {
            self.entities[index].clone()
        } else {
            None
        }
    }

    /// Sets a label for an entity
    pub fn set_label(&mut self, index: usize, label: String) {
        self.labels.insert(index, label);
    }

    /// Returns the label for an entity
    pub fn label(&self, index: usize) -> Option<&String> {
        self.labels.get(&index)
    }
}

impl Default for InterfaceInterfaceModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let model = InterfaceInterfaceModel::new();
        assert_eq!(model.nb_entities(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut model = InterfaceInterfaceModel::new();
        let entity = Arc::new(42);
        let idx = model.add_entity(entity.clone());
        assert_eq!(idx, 0);
        assert_eq!(model.nb_entities(), 1);
        assert!(model.entity(0).is_some());
    }

    #[test]
    fn test_set_protocol() {
        let mut model = InterfaceInterfaceModel::new();
        let proto = Arc::new("protocol");
        model.set_protocol(proto);
        assert!(model.protocol().is_some());
    }

    #[test]
    fn test_labels() {
        let mut model = InterfaceInterfaceModel::new();
        model.set_label(0, "entity0".to_string());
        assert_eq!(model.label(0), Some(&"entity0".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut model = InterfaceInterfaceModel::new();
        model.add_entity(Arc::new(1));
        model.clear();
        assert_eq!(model.nb_entities(), 0);
    }

    #[test]
    fn test_dispatch_status() {
        let mut model = InterfaceInterfaceModel::new();
        *model.dispatch_status() = true;
        assert!(*model.dispatch_status());
    }
}
