// FILE: if_select_print_count.rs
// occt: IFSelect_PrintCount

#[derive(Clone, Debug)]
pub struct IfSelectPrintCount;

impl IfSelectPrintCount {
    pub fn new() -> Self {
        IfSelectPrintCount
    }

    pub fn print(&self) {}
}

impl Default for IfSelectPrintCount {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IfSelectPrintCount::new();
    }
}
