// FILE: std_obj_mgt_attribute.rs
// occt: StdObjMgt_Attribute

/// Root class for a temporary persistent object corresponding to an attribute.
/// This is a template-like structure that manages the lifecycle of transient attributes
/// during persistence read/write operations.
pub struct StdObjMgtAttribute;

impl StdObjMgtAttribute {
    /// Create a new attribute manager
    pub fn new() -> Self {
        StdObjMgtAttribute
    }
}

/// Base class for persistent attribute storage.
/// Manages the transient attribute and its lifecycle.
pub struct StdObjMgtAttributeBase {
    type_num: i32,
    ref_num: i32,
}

impl StdObjMgtAttributeBase {
    /// Create a new base attribute
    pub fn new() -> Self {
        StdObjMgtAttributeBase {
            type_num: 0,
            ref_num: 0,
        }
    }

    /// Get the assigned persistent type number
    pub fn type_num(&self) -> i32 {
        self.type_num
    }

    /// Set the persistent type number
    pub fn set_type_num(&mut self, num: i32) {
        self.type_num = num;
    }

    /// Get the object reference number
    pub fn ref_num(&self) -> i32 {
        self.ref_num
    }

    /// Set the object reference number
    pub fn set_ref_num(&mut self, num: i32) {
        self.ref_num = num;
    }

    /// Returns persistent type name
    pub fn p_name(&self) -> &str {
        "StdObjMgt_Attribute::undefined"
    }
}

impl Default for StdObjMgtAttribute {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StdObjMgtAttributeBase {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple attribute variant for single integer values
pub struct SingleIntAttribute {
    base: StdObjMgtAttributeBase,
    data: i32,
}

impl SingleIntAttribute {
    /// Create a new single-int attribute
    pub fn new(value: i32) -> Self {
        SingleIntAttribute {
            base: StdObjMgtAttributeBase::new(),
            data: value,
        }
    }

    /// Get the stored value
    pub fn value(&self) -> i32 {
        self.data
    }

    /// Set the stored value
    pub fn set_value(&mut self, value: i32) {
        self.data = value;
    }

    /// Returns persistent type name
    pub fn p_name(&self) -> &str {
        "StdObjMgt_Attribute::SingleInt"
    }
}

/// Simple attribute variant for persistent references
pub struct SingleRefAttribute {
    base: StdObjMgtAttributeBase,
    ref_data: Option<i32>,
}

impl SingleRefAttribute {
    /// Create a new single-ref attribute
    pub fn new() -> Self {
        SingleRefAttribute {
            base: StdObjMgtAttributeBase::new(),
            ref_data: None,
        }
    }

    /// Get the reference
    pub fn reference(&self) -> Option<i32> {
        self.ref_data
    }

    /// Set the reference
    pub fn set_reference(&mut self, ref_num: Option<i32>) {
        self.ref_data = ref_num;
    }

    /// Returns persistent type name
    pub fn p_name(&self) -> &str {
        "StdObjMgt_Attribute::SingleRef"
    }
}

impl Default for SingleRefAttribute {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_attribute() {
        let _attr = StdObjMgtAttribute::new();
    }

    #[test]
    fn test_base_attribute() {
        let mut base = StdObjMgtAttributeBase::new();
        assert_eq!(base.type_num(), 0);
        assert_eq!(base.ref_num(), 0);

        base.set_type_num(42);
        base.set_ref_num(99);

        assert_eq!(base.type_num(), 42);
        assert_eq!(base.ref_num(), 99);
    }

    #[test]
    fn test_single_int_attribute() {
        let mut attr = SingleIntAttribute::new(123);
        assert_eq!(attr.value(), 123);
        assert_eq!(attr.p_name(), "StdObjMgt_Attribute::SingleInt");

        attr.set_value(456);
        assert_eq!(attr.value(), 456);
    }

    #[test]
    fn test_single_ref_attribute() {
        let mut attr = SingleRefAttribute::new();
        assert_eq!(attr.reference(), None);

        attr.set_reference(Some(42));
        assert_eq!(attr.reference(), Some(42));

        assert_eq!(attr.p_name(), "StdObjMgt_Attribute::SingleRef");
    }

    #[test]
    fn test_p_name() {
        let base = StdObjMgtAttributeBase::new();
        assert_eq!(base.p_name(), "StdObjMgt_Attribute::undefined");
    }
}
