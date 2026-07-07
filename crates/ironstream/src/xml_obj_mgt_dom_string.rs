// FILE: xml_obj_mgt_dom_string.rs
// occt: XmlObjMgt_DOMString

/// XmlObjMgt_DOMString is a typedef alias for DOM string representation.
/// In OCCT, it wraps LDOMString for XML DOM operations.
pub type XmlObjMgt_DOMString = String;

/// Helper to create a DOMString from a string slice.
pub fn dom_string(s: &str) -> XmlObjMgt_DOMString {
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_string_creation() {
        let s = dom_string("test");
        assert_eq!(s, "test");
    }

    #[test]
    fn test_dom_string_type() {
        let s: XmlObjMgt_DOMString = "hello".to_string();
        assert_eq!(s, "hello");
    }
}
