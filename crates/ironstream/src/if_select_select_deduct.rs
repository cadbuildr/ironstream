// FILE: if_select_select_deduct.rs
// occt: IFSelect_SelectDeduct

#[derive(Clone, Debug)]
pub struct IfSelectSelectDeduct {
    input: Option<usize>,
}

impl IfSelectSelectDeduct {
    pub fn new() -> Self {
        IfSelectSelectDeduct { input: None }
    }

    pub fn set_input(&mut self, inp: usize) {
        self.input = Some(inp);
    }

    pub fn input(&self) -> Option<usize> {
        self.input
    }
}

impl Default for IfSelectSelectDeduct {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sd = IfSelectSelectDeduct::new();
        assert_eq!(sd.input(), None);
    }

    #[test]
    fn test_set_input() {
        let mut sd = IfSelectSelectDeduct::new();
        sd.set_input(1);
        assert_eq!(sd.input(), Some(1));
    }
}
