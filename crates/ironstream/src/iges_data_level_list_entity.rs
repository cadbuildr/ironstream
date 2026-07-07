// FILE: iges_data_level_list_entity.rs
// occt: IGESData_LevelListEntity

//! Level list entity for IGES.

#[derive(Clone, Debug)]
pub struct LevelListEntity {
    levels: Vec<i32>,
}

impl LevelListEntity {
    pub fn new() -> Self {
        LevelListEntity {
            levels: Vec::new(),
        }
    }

    pub fn add_level(&mut self, level: i32) {
        self.levels.push(level);
    }

    pub fn levels(&self) -> &[i32] {
        &self.levels
    }

    pub fn nb_levels(&self) -> usize {
        self.levels.len()
    }
}

impl Default for LevelListEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = LevelListEntity::new();
        assert_eq!(entity.nb_levels(), 0);
    }

    #[test]
    fn test_add_level() {
        let mut entity = LevelListEntity::new();
        entity.add_level(1);
        entity.add_level(2);
        assert_eq!(entity.nb_levels(), 2);
        assert_eq!(entity.levels(), &[1, 2]);
    }
}
