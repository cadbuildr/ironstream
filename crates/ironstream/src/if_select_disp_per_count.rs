// FILE: if_select_disp_per_count.rs
// occt: IFSelect_DispPerCount

#[derive(Clone, Debug)]
pub struct IfSelectDispPerCount;

impl IfSelectDispPerCount {
    pub fn new() -> Self {
        IfSelectDispPerCount
    }
}

impl Default for IfSelectDispPerCount {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IfSelectDispPerCount::new();
    }
}
