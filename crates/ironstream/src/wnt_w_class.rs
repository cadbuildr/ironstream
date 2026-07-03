// FILE: wnt_w_class.rs
// occt: WNT_WClass

#[derive(Clone, Debug, Default)]
pub struct WClass {
    class_name: String,
    style: u32,
}

impl WClass {
    pub fn new(name: String) -> Self { Self { class_name: name, style: 0 } }
    pub fn class_name(&self) -> &str { &self.class_name }
    pub fn style(&self) -> u32 { self.style }
    pub fn set_style(&mut self, s: u32) { self.style = s; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let wc = WClass::new("TestClass".to_string());
        assert_eq!(wc.class_name(), "TestClass");
    }
}
