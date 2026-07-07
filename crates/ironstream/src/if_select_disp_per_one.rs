// FILE: if_select_disp_per_one.rs
// occt: IFSelect_DispPerOne

#[derive(Clone, Debug)]
pub struct IfSelectDispPerOne;

impl IfSelectDispPerOne {
    pub fn new() -> Self {
        IfSelectDispPerOne
    }
}

impl Default for IfSelectDispPerOne {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IfSelectDispPerOne::new();
    }
}
