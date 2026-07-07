// FILE: if_select_select_entity_number.rs
// occt: IFSelect_SelectEntityNumber

#[derive(Clone, Debug)]
pub struct IfSelectSelectEntityNumber {
    number: usize,
}

impl IfSelectSelectEntityNumber {
    pub fn new(num: usize) -> Self {
        IfSelectSelectEntityNumber { number: num }
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn set_number(&mut self, num: usize) {
        self.number = num;
    }
}

impl Default for IfSelectSelectEntityNumber {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sen = IfSelectSelectEntityNumber::new(5);
        assert_eq!(sen.number(), 5);
    }

    #[test]
    fn test_set_number() {
        let mut sen = IfSelectSelectEntityNumber::new(5);
        sen.set_number(10);
        assert_eq!(sen.number(), 10);
    }
}
