// FILE: std_l_persistent_value.rs
// occt: StdLPersistent_Value

/// Persistent simple value attributes
pub struct StdLPersistentValue;

impl StdLPersistentValue {
    /// Import integer value
    pub fn import_integer(value: i32) -> i32 {
        value
    }

    /// Import real value
    pub fn import_real(value: f64) -> f64 {
        value
    }

    /// Import string value
    pub fn import_string(value: &str) -> String {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_integer() {
        assert_eq!(StdLPersistentValue::import_integer(42), 42);
    }

    #[test]
    fn test_import_real() {
        assert_eq!(StdLPersistentValue::import_real(3.14), 3.14);
    }

    #[test]
    fn test_import_string() {
        assert_eq!(StdLPersistentValue::import_string("hello"), "hello");
    }
}
