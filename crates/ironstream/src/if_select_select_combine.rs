// FILE: if_select_select_combine.rs
// occt: IFSelect_SelectCombine

#[derive(Clone, Debug)]
pub struct IfSelectSelectCombine {
    operands: Vec<usize>,
}

impl IfSelectSelectCombine {
    pub fn new() -> Self {
        IfSelectSelectCombine {
            operands: vec![],
        }
    }

    pub fn add_operand(&mut self, op: usize) {
        self.operands.push(op);
    }

    pub fn operand_count(&self) -> usize {
        self.operands.len()
    }
}

impl Default for IfSelectSelectCombine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sc = IfSelectSelectCombine::new();
        assert_eq!(sc.operand_count(), 0);
    }

    #[test]
    fn test_add_operand() {
        let mut sc = IfSelectSelectCombine::new();
        sc.add_operand(1);
        assert_eq!(sc.operand_count(), 1);
    }
}
