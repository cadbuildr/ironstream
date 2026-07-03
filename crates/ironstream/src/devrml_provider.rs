// FILE: devrml_provider.rs
// occt: DEVRML_Provider
#[derive(Clone, Debug)]
pub struct DevrmlProvider { format: String, vendor: String }
impl DevrmlProvider {
    pub fn new() -> Self {
        Self { format: "VRML".to_string(), vendor: "OCC".to_string() }
    }
    pub fn get_format(&self) -> &str { &self.format }
    pub fn get_vendor(&self) -> &str { &self.vendor }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() { assert_eq!(DevrmlProvider::new().get_format(), "VRML"); }
}
