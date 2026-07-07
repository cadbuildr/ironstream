// FILE: if_select_select_flag.rs
// occt: IFSelect_SelectFlag

#[derive(Clone, Debug)]
pub struct IfSelectSelectFlag {
    flag: bool,
}

impl IfSelectSelectFlag {
    pub fn new(flag: bool) -> Self {
        IfSelectSelectFlag { flag }
    }

    pub fn flag(&self) -> bool {
        self.flag
    }

    pub fn set_flag(&mut self, f: bool) {
        self.flag = f;
    }
}

impl Default for IfSelectSelectFlag {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sf = IfSelectSelectFlag::new(true);
        assert!(sf.flag());
    }

    #[test]
    fn test_set_flag() {
        let mut sf = IfSelectSelectFlag::new(false);
        sf.set_flag(true);
        assert!(sf.flag());
    }
}
