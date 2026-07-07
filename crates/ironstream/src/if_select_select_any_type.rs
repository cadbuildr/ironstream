// FILE: if_select_select_any_type.rs
// occt: IFSelect_SelectAnyType

#[derive(Clone, Debug)]
pub struct IfSelectSelectAnyType;

impl IfSelectSelectAnyType {
    pub fn new() -> Self {
        IfSelectSelectAnyType
    }

    pub fn select(&self) -> bool {
        true
    }
}

impl Default for IfSelectSelectAnyType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sat = IfSelectSelectAnyType::new();
        assert!(sat.select());
    }
}
