// FILE: step_repr_quantified_assembly_component_usage.rs
// occt: StepRepr_QuantifiedAssemblyComponentUsage

/// StepRepr_QuantifiedAssemblyComponentUsage: Representation of STEP entity QuantifiedAssemblyComponentUsage
/// Inherits from StepRepr_AssemblyComponentUsage
#[derive(Clone, Debug)]
pub struct StepReprQuantifiedAssemblyComponentUsage {
    id: String,
    name: String,
    quantity: String,  // Simplified: storing identifier
}

impl StepReprQuantifiedAssemblyComponentUsage {
    /// Empty constructor
    pub fn new() -> Self {
        StepReprQuantifiedAssemblyComponentUsage {
            id: String::new(),
            name: String::new(),
            quantity: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, id: String, name: String, quantity: String) {
        self.id = id;
        self.name = name;
        self.quantity = quantity;
    }

    /// Returns field Quantity
    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    /// Set field Quantity
    pub fn set_quantity(&mut self, quantity: String) {
        self.quantity = quantity;
    }

    /// Get id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set id
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepReprQuantifiedAssemblyComponentUsage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let qacu = StepReprQuantifiedAssemblyComponentUsage::new();
        assert_eq!(qacu.id(), "");
        assert_eq!(qacu.name(), "");
        assert_eq!(qacu.quantity(), "");
    }

    #[test]
    fn test_init() {
        let mut qacu = StepReprQuantifiedAssemblyComponentUsage::new();
        qacu.init("id1".to_string(), "name1".to_string(), "qty1".to_string());
        assert_eq!(qacu.id(), "id1");
        assert_eq!(qacu.name(), "name1");
        assert_eq!(qacu.quantity(), "qty1");
    }

    #[test]
    fn test_set_quantity() {
        let mut qacu = StepReprQuantifiedAssemblyComponentUsage::new();
        qacu.set_quantity("newqty".to_string());
        assert_eq!(qacu.quantity(), "newqty");
    }

    #[test]
    fn test_set_id_and_name() {
        let mut qacu = StepReprQuantifiedAssemblyComponentUsage::new();
        qacu.set_id("newid".to_string());
        qacu.set_name("newname".to_string());
        assert_eq!(qacu.id(), "newid");
        assert_eq!(qacu.name(), "newname");
    }
}
