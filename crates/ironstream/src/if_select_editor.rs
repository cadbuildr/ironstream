// FILE: if_select_editor.rs
// occt: IFSelect_Editor

#[derive(Clone, Debug)]
pub struct IfSelectEditor {
    name: String,
}

impl IfSelectEditor {
    pub fn new(name: &str) -> Self {
        IfSelectEditor {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn label(&self) -> &str {
        &self.name
    }

    pub fn edit_count(&self) -> usize {
        0
    }
}

impl Default for IfSelectEditor {
    fn default() -> Self {
        Self::new("editor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ed = IfSelectEditor::new("myeditor");
        assert_eq!(ed.name(), "myeditor");
    }
}
