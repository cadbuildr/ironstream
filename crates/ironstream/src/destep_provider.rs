// FILE: destep_provider.rs
// occt: DESTEP_Provider
#[derive(Clone, Debug)]
pub struct DestepProvider {
    format: String,
    vendor: String,
}
impl DestepProvider {
    pub fn new() -> Self {
        Self {
            format: "STEP".to_string(),
            vendor: "OCC".to_string(),
        }
    }
    pub fn get_format(&self) -> &str { &self.format }
    pub fn get_vendor(&self) -> &str { &self.vendor }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let p = DestepProvider::new();
        assert_eq!(p.get_format(), "STEP");
    }
}
