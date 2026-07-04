// FILE: if_select_modif_edit_form.rs
// occt: IFSelect_ModifEditForm

#[derive(Clone, Debug)]
pub struct IfSelectModifEditForm {
    fields: Vec<String>,
}

impl IfSelectModifEditForm {
    pub fn new() -> Self {
        IfSelectModifEditForm {
            fields: vec![],
        }
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
}

impl Default for IfSelectModifEditForm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mef = IfSelectModifEditForm::new();
        assert_eq!(mef.field_count(), 0);
    }
}
