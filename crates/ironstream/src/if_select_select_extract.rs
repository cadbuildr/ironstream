// FILE: if_select_select_extract.rs
// occt: IFSelect_SelectExtract

#[derive(Clone, Debug)]
pub struct IfSelectSelectExtract {
    input: Option<usize>,
}

impl IfSelectSelectExtract {
    pub fn new() -> Self {
        IfSelectSelectExtract { input: None }
    }

    pub fn set_input(&mut self, inp: usize) {
        self.input = Some(inp);
    }

    pub fn input(&self) -> Option<usize> {
        self.input
    }

    pub fn extract(&self) -> bool {
        self.input.is_some()
    }
}

impl Default for IfSelectSelectExtract {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let se = IfSelectSelectExtract::new();
        assert!(!se.extract());
    }

    #[test]
    fn test_set_input() {
        let mut se = IfSelectSelectExtract::new();
        se.set_input(1);
        assert!(se.extract());
    }
}
