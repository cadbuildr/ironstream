// FILE: if_select_param_editor.rs
// occt: IFSelect_ParamEditor

#[derive(Clone, Debug)]
pub struct IfSelectParamEditor;

impl IfSelectParamEditor {
    pub fn new() -> Self {
        IfSelectParamEditor
    }

    pub fn edit(&self) -> bool {
        true
    }
}

impl Default for IfSelectParamEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let pe = IfSelectParamEditor::new();
        assert!(pe.edit());
    }
}
