// FILE: tdata_std.rs
// occt: TDataStd_Integer, TDataStd_Real, TDataStd_AsciiString,
//       TDataStd_Name, TDataStd_ExtStringList, TDataStd_BooleanList

/// A named integer attribute on a CAF label.
/// occt: TDataStd_Integer
#[derive(Clone, Debug)]
pub struct TDataStdInteger {
    pub label_id: u32,
    pub value: i32,
    pub is_attached: bool,
}

impl TDataStdInteger {
    pub fn new(label_id: u32, value: i32) -> Self {
        Self { label_id, value, is_attached: true }
    }

    pub fn get(&self) -> i32 { self.value }
    pub fn set(&mut self, v: i32) { self.value = v; }
    pub fn is_attached(&self) -> bool { self.is_attached }
    pub fn label_id(&self) -> u32 { self.label_id }
}

/// A real (f64) attribute on a CAF label.
/// occt: TDataStd_Real
#[derive(Clone, Debug)]
pub struct TDataStdReal {
    pub label_id: u32,
    pub value: f64,
    pub dimension: TDataStdRealDimension,
    pub is_attached: bool,
}

/// The kind of physical quantity.
/// occt: TDataStd_RealEnum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TDataStdRealDimension {
    #[default]
    Dimensionless,
    Length,
    Angle,
    Area,
    Volume,
    Mass,
    Time,
}

impl TDataStdReal {
    pub fn new(label_id: u32, value: f64) -> Self {
        Self { label_id, value, dimension: TDataStdRealDimension::Dimensionless, is_attached: true }
    }

    pub fn get(&self) -> f64 { self.value }
    pub fn set(&mut self, v: f64) { self.value = v; }
    pub fn set_dimension(&mut self, d: TDataStdRealDimension) { self.dimension = d; }
    pub fn dimension(&self) -> TDataStdRealDimension { self.dimension }
    pub fn is_attached(&self) -> bool { self.is_attached }
}

/// An ASCII string attribute.
/// occt: TDataStd_AsciiString
#[derive(Clone, Debug)]
pub struct TDataStdAsciiString {
    pub label_id: u32,
    pub value: String,
    pub is_attached: bool,
}

impl TDataStdAsciiString {
    pub fn new(label_id: u32, value: &str) -> Self {
        Self { label_id, value: value.to_string(), is_attached: true }
    }

    pub fn get(&self) -> &str { &self.value }
    pub fn set(&mut self, v: &str) { self.value = v.to_string(); }
    pub fn length(&self) -> usize { self.value.len() }
    pub fn is_empty(&self) -> bool { self.value.is_empty() }
    pub fn is_attached(&self) -> bool { self.is_attached }
}

/// A name (wide string) attribute.
/// occt: TDataStd_Name
#[derive(Clone, Debug)]
pub struct TDataStdName {
    pub label_id: u32,
    pub value: String,
    pub is_attached: bool,
}

impl TDataStdName {
    pub fn new(label_id: u32, value: &str) -> Self {
        Self { label_id, value: value.to_string(), is_attached: true }
    }

    pub fn get(&self) -> &str { &self.value }
    pub fn set(&mut self, v: &str) { self.value = v.to_string(); }
    pub fn is_attached(&self) -> bool { self.is_attached }
}

/// A list of extended (Unicode) strings attribute.
/// occt: TDataStd_ExtStringList
#[derive(Clone, Debug)]
pub struct TDataStdExtStringList {
    pub label_id: u32,
    pub values: Vec<String>,
    pub is_attached: bool,
}

impl TDataStdExtStringList {
    pub fn new(label_id: u32) -> Self {
        Self { label_id, values: Vec::new(), is_attached: true }
    }

    pub fn append(&mut self, s: &str) { self.values.push(s.to_string()); }
    pub fn prepend(&mut self, s: &str) { self.values.insert(0, s.to_string()); }
    pub fn remove_first(&mut self) { if !self.values.is_empty() { self.values.remove(0); } }
    pub fn remove_last(&mut self) { self.values.pop(); }
    pub fn clear(&mut self) { self.values.clear(); }
    pub fn size(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
    pub fn value(&self, i: usize) -> Option<&str> { self.values.get(i).map(|s| s.as_str()) }
    pub fn contains(&self, s: &str) -> bool { self.values.iter().any(|v| v == s) }
}

/// A list of boolean values attribute.
/// occt: TDataStd_BooleanList
#[derive(Clone, Debug)]
pub struct TDataStdBooleanList {
    pub label_id: u32,
    pub values: Vec<bool>,
    pub is_attached: bool,
}

impl TDataStdBooleanList {
    pub fn new(label_id: u32) -> Self {
        Self { label_id, values: Vec::new(), is_attached: true }
    }

    pub fn append(&mut self, v: bool) { self.values.push(v); }
    pub fn prepend(&mut self, v: bool) { self.values.insert(0, v); }
    pub fn remove_first(&mut self) { if !self.values.is_empty() { self.values.remove(0); } }
    pub fn clear(&mut self) { self.values.clear(); }
    pub fn size(&self) -> usize { self.values.len() }
    pub fn value(&self, i: usize) -> Option<bool> { self.values.get(i).copied() }
    pub fn nb_true(&self) -> usize { self.values.iter().filter(|&&v| v).count() }
    pub fn nb_false(&self) -> usize { self.values.iter().filter(|&&v| !v).count() }
}

/// An integer array attribute.
/// occt: TDataStd_IntegerArray
#[derive(Clone, Debug)]
pub struct TDataStdIntegerArray {
    pub label_id: u32,
    pub lower: usize,
    pub upper: usize,
    data: Vec<i32>,
    pub is_attached: bool,
}

impl TDataStdIntegerArray {
    pub fn new(label_id: u32, lower: usize, upper: usize) -> Self {
        let n = if upper >= lower { upper - lower + 1 } else { 0 };
        Self { label_id, lower, upper, data: vec![0; n], is_attached: true }
    }

    pub fn value(&self, i: usize) -> Option<i32> {
        if i < self.lower || i > self.upper { return None; }
        Some(self.data[i - self.lower])
    }

    pub fn set_value(&mut self, i: usize, v: i32) {
        if i >= self.lower && i <= self.upper { self.data[i - self.lower] = v; }
    }

    pub fn length(&self) -> usize { self.data.len() }
    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_attr() {
        let mut a = TDataStdInteger::new(1, 42);
        assert_eq!(a.get(), 42);
        a.set(-7);
        assert_eq!(a.get(), -7);
        assert!(a.is_attached());
    }

    #[test]
    fn real_attr_dimension() {
        let mut r = TDataStdReal::new(2, 3.14);
        assert!((r.get() - 3.14).abs() < 1e-10);
        r.set_dimension(TDataStdRealDimension::Length);
        assert_eq!(r.dimension(), TDataStdRealDimension::Length);
    }

    #[test]
    fn ascii_string_attr() {
        let mut s = TDataStdAsciiString::new(3, "hello");
        assert_eq!(s.get(), "hello");
        assert_eq!(s.length(), 5);
        s.set("world");
        assert_eq!(s.get(), "world");
        assert!(!s.is_empty());
    }

    #[test]
    fn ext_string_list_ops() {
        let mut l = TDataStdExtStringList::new(4);
        l.append("a");
        l.append("b");
        l.prepend("z");
        assert_eq!(l.size(), 3);
        assert_eq!(l.value(0), Some("z"));
        assert!(l.contains("b"));
        l.remove_first();
        assert_eq!(l.size(), 2);
    }

    #[test]
    fn bool_list_counts() {
        let mut b = TDataStdBooleanList::new(5);
        b.append(true); b.append(false); b.append(true); b.append(true);
        assert_eq!(b.nb_true(), 3);
        assert_eq!(b.nb_false(), 1);
        assert_eq!(b.value(1), Some(false));
    }

    #[test]
    fn integer_array() {
        let mut a = TDataStdIntegerArray::new(6, 1, 3);
        a.set_value(1, 10);
        a.set_value(2, 20);
        a.set_value(3, 30);
        assert_eq!(a.value(1), Some(10));
        assert_eq!(a.value(4), None);
        assert_eq!(a.length(), 3);
    }
}
