// FILE: destl_provider.rs
// occt: DESTL_Provider
#[derive(Clone, Debug)]
pub struct DestlProvider { format: String, vendor: String }
impl DestlProvider {
    pub fn new() -> Self {
        Self { format: "STL".to_string(), vendor: "OCC".to_string() }
    }
    pub fn get_format(&self) -> &str { &self.format }
    pub fn get_vendor(&self) -> &str { &self.vendor }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() { assert_eq!(DestlProvider::new().get_format(), "STL"); }
}
