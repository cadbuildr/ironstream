// FILE: iges_to_b_rep_b_rep_entity.rs
// occt: IGESToBRep_BRepEntity

#[derive(Default, Clone, Debug)]
pub struct IgesToBRepBRepEntity;

impl IgesToBRepBRepEntity {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _entity = IgesToBRepBRepEntity::new();
    }
}
