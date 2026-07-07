// FILE: step_fea_freedoms_list.rs
// occt: StepFEA_FreedomsList

/// Representation of STEP entity FreedomsList
#[derive(Debug, Clone)]
pub struct StepFeaFreedomsList {
    freedoms: Vec<i32>,
}

impl StepFeaFreedomsList {
    /// Creates a new empty FreedomsList
    pub fn new() -> Self {
        StepFeaFreedomsList {
            freedoms: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, freedoms: Vec<i32>) {
        self.freedoms = freedoms;
    }

    /// Returns field Freedoms
    pub fn freedoms(&self) -> &[i32] {
        &self.freedoms
    }

    /// Set field Freedoms
    pub fn set_freedoms(&mut self, freedoms: Vec<i32>) {
        self.freedoms = freedoms;
    }
}

impl Default for StepFeaFreedomsList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freedoms_list_creation() {
        let list = StepFeaFreedomsList::new();
        assert_eq!(list.freedoms().len(), 0);
    }

    #[test]
    fn test_freedoms_list_init() {
        let mut list = StepFeaFreedomsList::new();
        let freedoms = vec![1, 2, 3];
        list.init(freedoms);

        assert_eq!(list.freedoms(), &[1, 2, 3]);
    }

    #[test]
    fn test_freedoms_list_setters() {
        let mut list = StepFeaFreedomsList::new();
        list.set_freedoms(vec![4, 5, 6]);

        assert_eq!(list.freedoms(), &[4, 5, 6]);
    }
}
