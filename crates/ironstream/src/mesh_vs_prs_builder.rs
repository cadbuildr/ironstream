// FILE: mesh_vs_prs_builder.rs
// occt: MeshVS_PrsBuilder

//! Base presentation builder for mesh visualization.

#[derive(Clone, Debug)]
pub struct PrsBuilder {
    pub builder_id: u32,
    pub priority: i32,
    pub is_active: bool,
}

impl PrsBuilder {
    pub fn new(builder_id: u32, priority: i32) -> Self {
        PrsBuilder {
            builder_id,
            priority,
            is_active: true,
        }
    }

    pub fn default_priority() -> i32 {
        5
    }

    pub fn with_priority(mut self, prio: i32) -> Self {
        self.priority = prio.clamp(0, 100);
        self
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_builder_creation() {
        let builder = PrsBuilder::new(1, 5);
        assert_eq!(builder.builder_id, 1);
        assert_eq!(builder.priority, 5);
        assert!(builder.is_active());
    }

    #[test]
    fn prs_builder_default_priority() {
        assert_eq!(PrsBuilder::default_priority(), 5);
    }

    #[test]
    fn prs_builder_activity() {
        let mut builder = PrsBuilder::new(1, 5);
        builder.deactivate();
        assert!(!builder.is_active());
        builder.activate();
        assert!(builder.is_active());
    }
}
