// FILE: if_select_edit_form.rs
// occt: IFSelect_EditForm

#[derive(Clone, Debug)]
pub struct IfSelectEditForm {
    fields: Vec<String>,
}

impl IfSelectEditForm {
    pub fn new() -> Self {
        IfSelectEditForm {
            fields: vec![],
        }
    }

    pub fn add_field(&mut self, name: &str) {
        self.fields.push(name.to_string());
    }

    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
}

impl Default for IfSelectEditForm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let form = IfSelectEditForm::new();
        assert_eq!(form.field_count(), 0);
    }

    #[test]
    fn test_add_field() {
        let mut form = IfSelectEditForm::new();
        form.add_field("name");
        assert_eq!(form.field_count(), 1);
    }
}
