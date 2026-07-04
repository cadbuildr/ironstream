// FILE: if_select_modif_reorder.rs
// occt: IFSelect_ModifReorder

#[derive(Clone, Debug)]
pub struct IfSelectModifReorder;

impl IfSelectModifReorder {
    pub fn new() -> Self {
        IfSelectModifReorder
    }

    pub fn reorder(&self) {}
}

impl Default for IfSelectModifReorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IfSelectModifReorder::new();
    }
}
