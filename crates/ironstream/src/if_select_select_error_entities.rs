// FILE: if_select_select_error_entities.rs
// occt: IFSelect_SelectErrorEntities

#[derive(Clone, Debug)]
pub struct IfSelectSelectErrorEntities {
    errors: Vec<usize>,
}

impl IfSelectSelectErrorEntities {
    pub fn new() -> Self {
        IfSelectSelectErrorEntities {
            errors: vec![],
        }
    }

    pub fn add_error(&mut self, entity: usize) {
        self.errors.push(entity);
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}

impl Default for IfSelectSelectErrorEntities {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let see = IfSelectSelectErrorEntities::new();
        assert_eq!(see.error_count(), 0);
    }

    #[test]
    fn test_add_error() {
        let mut see = IfSelectSelectErrorEntities::new();
        see.add_error(1);
        assert_eq!(see.error_count(), 1);
    }
}
