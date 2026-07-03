// FILE: mesh_vs_elemental_color_prs_builder.rs
// occt: MeshVS_ElementalColorPrsBuilder

//! Builder for elemental color presentation in meshes.

#[derive(Clone, Debug)]
pub struct ElementalColorPrsBuilder {
    pub builder_id: u32,
    pub priority: i32,
}

impl ElementalColorPrsBuilder {
    pub fn new(builder_id: u32, priority: i32) -> Self {
        ElementalColorPrsBuilder {
            builder_id,
            priority,
        }
    }

    pub fn default_priority() -> i32 {
        15
    }

    pub fn is_valid_priority(prio: i32) -> bool {
        prio >= 0 && prio <= 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elemental_color_prs_builder_creation() {
        let builder = ElementalColorPrsBuilder::new(1, 15);
        assert_eq!(builder.builder_id, 1);
        assert_eq!(builder.priority, 15);
    }

    #[test]
    fn elemental_color_prs_builder_priority() {
        assert_eq!(ElementalColorPrsBuilder::default_priority(), 15);
        assert!(ElementalColorPrsBuilder::is_valid_priority(15));
        assert!(!ElementalColorPrsBuilder::is_valid_priority(-1));
    }
}
