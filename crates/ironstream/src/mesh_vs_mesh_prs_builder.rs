// FILE: mesh_vs_mesh_prs_builder.rs
// occt: MeshVS_MeshPrsBuilder

//! Builder for mesh presentations.

#[derive(Clone, Debug)]
pub struct MeshPrsBuilder {
    pub builder_id: u32,
    pub priority: i32,
    pub is_enabled: bool,
}

impl MeshPrsBuilder {
    pub fn new(builder_id: u32, priority: i32) -> Self {
        MeshPrsBuilder {
            builder_id,
            priority,
            is_enabled: true,
        }
    }

    pub fn with_priority(mut self, prio: i32) -> Self {
        self.priority = prio.clamp(0, 100);
        self
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_prs_builder_creation() {
        let builder = MeshPrsBuilder::new(1, 5);
        assert_eq!(builder.builder_id, 1);
        assert_eq!(builder.priority, 5);
        assert!(builder.is_enabled());
    }

    #[test]
    fn mesh_prs_builder_control() {
        let mut builder = MeshPrsBuilder::new(1, 5);
        builder.disable();
        assert!(!builder.is_enabled());
        builder.enable();
        assert!(builder.is_enabled());
    }
}
