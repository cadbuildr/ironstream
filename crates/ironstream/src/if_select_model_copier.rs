// FILE: if_select_model_copier.rs
// occt: IFSelect_ModelCopier

#[derive(Clone, Debug)]
pub struct IfSelectModelCopier;

impl IfSelectModelCopier {
    pub fn new() -> Self {
        IfSelectModelCopier
    }

    pub fn copy(&self) -> bool {
        true
    }
}

impl Default for IfSelectModelCopier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mc = IfSelectModelCopier::new();
        assert!(mc.copy());
    }
}
