// FILE: if_select_edit_value.rs
// occt: IFSelect_EditValue

#[derive(Clone, Debug)]
pub struct IfSelectEditValue {
    value: String,
}

impl IfSelectEditValue {
    pub fn new(val: &str) -> Self {
        IfSelectEditValue {
            value: val.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, val: &str) {
        self.value = val.to_string();
    }
}

impl Default for IfSelectEditValue {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ev = IfSelectEditValue::new("test");
        assert_eq!(ev.value(), "test");
    }

    #[test]
    fn test_set_value() {
        let mut ev = IfSelectEditValue::new("old");
        ev.set_value("new");
        assert_eq!(ev.value(), "new");
    }
}
