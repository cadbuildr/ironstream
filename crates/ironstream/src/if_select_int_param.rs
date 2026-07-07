// FILE: if_select_int_param.rs
// occt: IFSelect_IntParam

#[derive(Clone, Debug)]
pub struct IfSelectIntParam {
    value: i32,
}

impl IfSelectIntParam {
    pub fn new(val: i32) -> Self {
        IfSelectIntParam { value: val }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, val: i32) {
        self.value = val;
    }
}

impl Default for IfSelectIntParam {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ip = IfSelectIntParam::new(42);
        assert_eq!(ip.value(), 42);
    }

    #[test]
    fn test_set_value() {
        let mut ip = IfSelectIntParam::new(10);
        ip.set_value(20);
        assert_eq!(ip.value(), 20);
    }
}
