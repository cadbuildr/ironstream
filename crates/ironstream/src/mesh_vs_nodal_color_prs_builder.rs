// FILE: mesh_vs_nodal_color_prs_builder.rs
// occt: MeshVS_NodalColorPrsBuilder

//! Builder for nodal color presentation in meshes.

#[derive(Clone, Debug)]
pub struct NodalColorPrsBuilder {
    pub builder_id: u32,
    pub priority: i32,
}

impl NodalColorPrsBuilder {
    pub fn new(builder_id: u32, priority: i32) -> Self {
        NodalColorPrsBuilder {
            builder_id,
            priority,
        }
    }

    pub fn default_priority() -> i32 {
        10
    }

    pub fn with_priority(mut self, prio: i32) -> Self {
        self.priority = prio.clamp(0, 100);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodal_color_prs_builder_creation() {
        let builder = NodalColorPrsBuilder::new(1, 10);
        assert_eq!(builder.builder_id, 1);
        assert_eq!(builder.priority, 10);
    }

    #[test]
    fn nodal_color_prs_builder_default_priority() {
        assert_eq!(NodalColorPrsBuilder::default_priority(), 10);
    }
}
