// FILE: ais_trihedron_owner.rs
// occt: AIS_TrihedronOwner

#[derive(Clone, Debug)]
pub struct AisTrihedronOwner {
    pub owner_id: u32,
    pub parent_id: u32,
    pub part_index: u32,
}

impl AisTrihedronOwner {
    pub fn new(owner_id: u32, parent_id: u32, part_index: u32) -> Self {
        Self { owner_id, parent_id, part_index }
    }
}

impl Default for AisTrihedronOwner {
    fn default() -> Self { Self::new(0, 0, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let owner = AisTrihedronOwner::new(1, 10, 2);
        assert_eq!(owner.owner_id, 1);
    }
}
