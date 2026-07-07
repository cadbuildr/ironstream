// FILE: if_select_functions.rs
// occt: IFSelect_Functions

pub struct IfSelectFunctions;

impl IfSelectFunctions {
    pub fn new() -> Self {
        IfSelectFunctions
    }

    pub fn define_functions() {}
}

impl Default for IfSelectFunctions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IfSelectFunctions::new();
    }
}
