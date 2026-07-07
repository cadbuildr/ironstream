// FILE: if_select_select_control.rs
// occt: IFSelect_SelectControl

#[derive(Clone, Debug)]
pub struct IfSelectSelectControl;

impl IfSelectSelectControl {
    pub fn new() -> Self {
        IfSelectSelectControl
    }

    pub fn control(&self) -> bool {
        true
    }
}

impl Default for IfSelectSelectControl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IfSelectSelectControl::new();
    }
}
