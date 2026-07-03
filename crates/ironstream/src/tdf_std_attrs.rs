// FILE: tdf_std_attrs.rs
// occt: TDataStd_Name, TDataStd_Integer, TDataStd_Real, TDataStd_IntegerArray,
//       TDataStd_RealArray, TDataStd_ByteArray, TDataStd_ExtStringArray,
//       TDataStd_Comment, TDataStd_UAttribute

/// Base trait for typed OCAF attributes.
pub trait DataStdAttr: std::fmt::Debug {
    fn attr_type() -> &'static str where Self: Sized;
    fn get_id(&self) -> &'static str;
}

// occt: TDataStd_Name
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TDataStdName {
    pub value: String,
}

impl TDataStdName {
    pub const ID: &'static str = "TDataStd_Name";
    pub fn new(name: impl Into<String>) -> Self { Self { value: name.into() } }
    pub fn value(&self) -> &str { &self.value }
    pub fn set(&mut self, name: impl Into<String>) { self.value = name.into(); }
    pub fn length(&self) -> usize { self.value.len() }
    pub fn is_empty(&self) -> bool { self.value.is_empty() }
}

impl DataStdAttr for TDataStdName {
    fn attr_type() -> &'static str { Self::ID }
    fn get_id(&self) -> &'static str { Self::ID }
}

// occt: TDataStd_Comment
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TDataStdComment {
    pub value: String,
}

impl TDataStdComment {
    pub const ID: &'static str = "TDataStd_Comment";
    pub fn new(comment: impl Into<String>) -> Self { Self { value: comment.into() } }
    pub fn value(&self) -> &str { &self.value }
    pub fn set(&mut self, c: impl Into<String>) { self.value = c.into(); }
}

// occt: TDataStd_Integer
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TDataStdInteger {
    pub value: i32,
}

impl TDataStdInteger {
    pub const ID: &'static str = "TDataStd_Integer";
    pub fn new(v: i32) -> Self { Self { value: v } }
    pub fn value(&self) -> i32 { self.value }
    pub fn set(&mut self, v: i32) { self.value = v; }
    pub fn increment(&mut self) { self.value += 1; }
    pub fn decrement(&mut self) { self.value -= 1; }
}

impl DataStdAttr for TDataStdInteger {
    fn attr_type() -> &'static str { Self::ID }
    fn get_id(&self) -> &'static str { Self::ID }
}

// occt: TDataStd_Real
#[derive(Clone, Debug, PartialEq)]
pub struct TDataStdReal {
    pub value: f64,
}

impl TDataStdReal {
    pub const ID: &'static str = "TDataStd_Real";
    pub fn new(v: f64) -> Self { Self { value: v } }
    pub fn value(&self) -> f64 { self.value }
    pub fn set(&mut self, v: f64) { self.value = v; }
    pub fn is_finite(&self) -> bool { self.value.is_finite() }
}

impl DataStdAttr for TDataStdReal {
    fn attr_type() -> &'static str { Self::ID }
    fn get_id(&self) -> &'static str { Self::ID }
}

// occt: TDataStd_IntegerArray
#[derive(Clone, Debug)]
pub struct TDataStdIntegerArray {
    pub lower: i32,
    pub upper: i32,
    pub values: Vec<i32>,
    pub delta_flag: bool,
}

impl TDataStdIntegerArray {
    pub const ID: &'static str = "TDataStd_IntegerArray";

    pub fn new(lower: i32, upper: i32) -> Self {
        let n = ((upper - lower + 1).max(0)) as usize;
        Self { lower, upper, values: vec![0; n], delta_flag: false }
    }

    pub fn set_value(&mut self, index: i32, val: i32) {
        let i = (index - self.lower) as usize;
        if i < self.values.len() { self.values[i] = val; }
    }

    pub fn value(&self, index: i32) -> Option<i32> {
        let i = (index - self.lower) as usize;
        self.values.get(i).copied()
    }

    pub fn length(&self) -> usize { self.values.len() }

    pub fn with_delta(mut self, flag: bool) -> Self { self.delta_flag = flag; self }

    pub fn sum(&self) -> i64 { self.values.iter().map(|&x| x as i64).sum() }
}

// occt: TDataStd_RealArray
#[derive(Clone, Debug)]
pub struct TDataStdRealArray {
    pub lower: i32,
    pub upper: i32,
    pub values: Vec<f64>,
    pub delta_flag: bool,
}

impl TDataStdRealArray {
    pub const ID: &'static str = "TDataStd_RealArray";

    pub fn new(lower: i32, upper: i32) -> Self {
        let n = ((upper - lower + 1).max(0)) as usize;
        Self { lower, upper, values: vec![0.0; n], delta_flag: false }
    }

    pub fn set_value(&mut self, index: i32, val: f64) {
        let i = (index - self.lower) as usize;
        if i < self.values.len() { self.values[i] = val; }
    }

    pub fn value(&self, index: i32) -> Option<f64> {
        let i = (index - self.lower) as usize;
        self.values.get(i).copied()
    }

    pub fn length(&self) -> usize { self.values.len() }

    pub fn min(&self) -> f64 { self.values.iter().cloned().fold(f64::INFINITY, f64::min) }
    pub fn max(&self) -> f64 { self.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max) }
    pub fn sum(&self) -> f64 { self.values.iter().sum() }
    pub fn mean(&self) -> f64 {
        if self.values.is_empty() { return 0.0; }
        self.sum() / self.values.len() as f64
    }
}

// occt: TDataStd_ByteArray
#[derive(Clone, Debug)]
pub struct TDataStdByteArray {
    pub lower: i32,
    pub upper: i32,
    pub values: Vec<u8>,
    pub delta_flag: bool,
}

impl TDataStdByteArray {
    pub const ID: &'static str = "TDataStd_ByteArray";

    pub fn new(lower: i32, upper: i32) -> Self {
        let n = ((upper - lower + 1).max(0)) as usize;
        Self { lower, upper, values: vec![0u8; n], delta_flag: false }
    }

    pub fn set_value(&mut self, index: i32, val: u8) {
        let i = (index - self.lower) as usize;
        if i < self.values.len() { self.values[i] = val; }
    }

    pub fn value(&self, index: i32) -> Option<u8> {
        let i = (index - self.lower) as usize;
        self.values.get(i).copied()
    }

    pub fn length(&self) -> usize { self.values.len() }
}

// occt: TDataStd_ExtStringArray
#[derive(Clone, Debug)]
pub struct TDataStdExtStringArray {
    pub lower: i32,
    pub upper: i32,
    pub values: Vec<String>,
}

impl TDataStdExtStringArray {
    pub const ID: &'static str = "TDataStd_ExtStringArray";

    pub fn new(lower: i32, upper: i32) -> Self {
        let n = ((upper - lower + 1).max(0)) as usize;
        Self { lower, upper, values: vec![String::new(); n] }
    }

    pub fn set_value(&mut self, index: i32, val: impl Into<String>) {
        let i = (index - self.lower) as usize;
        if i < self.values.len() { self.values[i] = val.into(); }
    }

    pub fn value(&self, index: i32) -> Option<&str> {
        let i = (index - self.lower) as usize;
        self.values.get(i).map(|s| s.as_str())
    }

    pub fn length(&self) -> usize { self.values.len() }
}

// occt: TDataStd_UAttribute (unique attribute marker)
#[derive(Clone, Debug)]
pub struct TDataStdUAttribute {
    pub guid: String,
}

impl TDataStdUAttribute {
    pub const ID: &'static str = "TDataStd_UAttribute";
    pub fn new(guid: impl Into<String>) -> Self { Self { guid: guid.into() } }
    pub fn id(&self) -> &str { &self.guid }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_attr() {
        let mut n = TDataStdName::new("Part-1");
        assert_eq!(n.value(), "Part-1");
        n.set("Part-2");
        assert_eq!(n.length(), 6);
    }

    #[test]
    fn integer_attr() {
        let mut i = TDataStdInteger::new(5);
        i.increment();
        assert_eq!(i.value(), 6);
        i.decrement();
        assert_eq!(i.value(), 5);
    }

    #[test]
    fn real_attr() {
        let r = TDataStdReal::new(3.14);
        assert!(r.is_finite());
        assert!((r.value() - 3.14).abs() < 1e-10);
    }

    #[test]
    fn integer_array() {
        let mut arr = TDataStdIntegerArray::new(1, 3);
        arr.set_value(1, 10);
        arr.set_value(2, 20);
        arr.set_value(3, 30);
        assert_eq!(arr.value(2), Some(20));
        assert_eq!(arr.sum(), 60);
        assert_eq!(arr.length(), 3);
    }

    #[test]
    fn real_array_stats() {
        let mut arr = TDataStdRealArray::new(0, 3);
        arr.set_value(0, 1.0);
        arr.set_value(1, 2.0);
        arr.set_value(2, 3.0);
        arr.set_value(3, 4.0);
        assert!((arr.mean() - 2.5).abs() < 1e-10);
        assert!((arr.min() - 1.0).abs() < 1e-10);
        assert!((arr.max() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn ext_string_array() {
        let mut arr = TDataStdExtStringArray::new(0, 1);
        arr.set_value(0, "hello");
        arr.set_value(1, "world");
        assert_eq!(arr.value(0), Some("hello"));
        assert_eq!(arr.length(), 2);
    }
}
