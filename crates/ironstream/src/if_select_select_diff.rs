// FILE: if_select_select_diff.rs
// occt: IFSelect_SelectDiff

#[derive(Clone, Debug)]
pub struct IfSelectSelectDiff {
    main: Option<usize>,
    sub: Option<usize>,
}

impl IfSelectSelectDiff {
    pub fn new() -> Self {
        IfSelectSelectDiff {
            main: None,
            sub: None,
        }
    }

    pub fn set_main(&mut self, m: usize) {
        self.main = Some(m);
    }

    pub fn set_sub(&mut self, s: usize) {
        self.sub = Some(s);
    }

    pub fn difference(&self) -> bool {
        self.main.is_some() && self.sub.is_some()
    }
}

impl Default for IfSelectSelectDiff {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sd = IfSelectSelectDiff::new();
        assert!(!sd.difference());
    }

    #[test]
    fn test_set_operands() {
        let mut sd = IfSelectSelectDiff::new();
        sd.set_main(1);
        sd.set_sub(2);
        assert!(sd.difference());
    }
}
