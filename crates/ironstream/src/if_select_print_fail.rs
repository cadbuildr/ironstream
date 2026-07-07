// FILE: if_select_print_fail.rs
// occt: IFSelect_PrintFail

#[derive(Clone, Debug)]
pub struct IfSelectPrintFail;

impl IfSelectPrintFail {
    pub fn new() -> Self {
        IfSelectPrintFail
    }

    pub fn print(&self) {}
}

impl Default for IfSelectPrintFail {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IfSelectPrintFail::new();
    }
}
