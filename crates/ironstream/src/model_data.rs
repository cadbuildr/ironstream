// FILE: model_data.rs
// occt-ref: TDataStd_Name, TDataStd_Comment, TDataStd_AsciiString, TDataStd_Integer

/// String attribute: stores text on a label.
// occt-ref: TDataStd_AsciiString
#[derive(Clone, Debug)]
pub struct TDataStdAsciiString {
    pub label_id: u32,
    pub value: String,
}

impl TDataStdAsciiString {
    pub fn new(label_id: u32, value: impl Into<String>) -> Self {
        Self {
            label_id,
            value: value.into(),
        }
    }

    pub fn get(&self) -> &str { &self.value }

    pub fn set(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    pub fn is_empty(&self) -> bool { self.value.is_empty() }
}

impl Default for TDataStdAsciiString {
    fn default() -> Self {
        Self {
            label_id: 0,
            value: String::new(),
        }
    }
}

/// Name attribute: identifies a label.
// occt-ref: TDataStd_Name
#[derive(Clone, Debug)]
pub struct TDataStdName {
    pub label_id: u32,
    pub name: String,
}

impl TDataStdName {
    pub fn new(label_id: u32, name: impl Into<String>) -> Self {
        Self {
            label_id,
            name: name.into(),
        }
    }

    pub fn get(&self) -> &str { &self.name }

    pub fn set(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }
}

impl Default for TDataStdName {
    fn default() -> Self {
        Self {
            label_id: 0,
            name: String::new(),
        }
    }
}

/// Comment attribute: descriptive text.
// occt-ref: TDataStd_Comment
#[derive(Clone, Debug)]
pub struct TDataStdComment {
    pub label_id: u32,
    pub comment: String,
}

impl TDataStdComment {
    pub fn new(label_id: u32, comment: impl Into<String>) -> Self {
        Self {
            label_id,
            comment: comment.into(),
        }
    }

    pub fn get(&self) -> &str { &self.comment }

    pub fn set(&mut self, comment: impl Into<String>) {
        self.comment = comment.into();
    }

    pub fn is_empty(&self) -> bool { self.comment.is_empty() }
}

impl Default for TDataStdComment {
    fn default() -> Self {
        Self {
            label_id: 0,
            comment: String::new(),
        }
    }
}

/// Integer attribute.
// occt-ref: TDataStd_Integer
#[derive(Clone, Debug)]
pub struct TDataStdInteger {
    pub label_id: u32,
    pub value: i32,
}

impl TDataStdInteger {
    pub fn new(label_id: u32, value: i32) -> Self {
        Self { label_id, value }
    }

    pub fn get(&self) -> i32 { self.value }

    pub fn set(&mut self, value: i32) {
        self.value = value;
    }
}

impl Default for TDataStdInteger {
    fn default() -> Self {
        Self {
            label_id: 0,
            value: 0,
        }
    }
}

/// Real number attribute.
// occt-ref: TDataStd_Real
#[derive(Clone, Debug)]
pub struct TDataStdReal {
    pub label_id: u32,
    pub value: f64,
}

impl TDataStdReal {
    pub fn new(label_id: u32, value: f64) -> Self {
        Self { label_id, value }
    }

    pub fn get(&self) -> f64 { self.value }

    pub fn set(&mut self, value: f64) {
        self.value = value;
    }
}

impl Default for TDataStdReal {
    fn default() -> Self {
        Self {
            label_id: 0,
            value: 0.0,
        }
    }
}

/// Attribute container: holds multiple attributes per label.
/// occt-note: TDataStd_AttributeDict (simplified)
#[derive(Clone, Debug, Default)]
pub struct TDataStdAttributeDict {
    pub label_id: u32,
    pub string_attrs: Vec<TDataStdAsciiString>,
    pub int_attrs: Vec<TDataStdInteger>,
    pub real_attrs: Vec<TDataStdReal>,
}

impl TDataStdAttributeDict {
    pub fn new(label_id: u32) -> Self {
        Self {
            label_id,
            string_attrs: Vec::new(),
            int_attrs: Vec::new(),
            real_attrs: Vec::new(),
        }
    }

    pub fn add_string(&mut self, s: TDataStdAsciiString) {
        self.string_attrs.push(s);
    }

    pub fn add_integer(&mut self, i: TDataStdInteger) {
        self.int_attrs.push(i);
    }

    pub fn add_real(&mut self, r: TDataStdReal) {
        self.real_attrs.push(r);
    }

    pub fn nb_attributes(&self) -> usize {
        self.string_attrs.len() + self.int_attrs.len() + self.real_attrs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tdatastd_ascii_string() {
        let mut s = TDataStdAsciiString::new(1, "hello");
        assert_eq!(s.get(), "hello");
        s.set("world");
        assert_eq!(s.get(), "world");
    }

    #[test]
    fn tdatastd_name() {
        let mut n = TDataStdName::new(2, "MyName");
        assert_eq!(n.get(), "MyName");
        n.set("NewName");
        assert_eq!(n.get(), "NewName");
    }

    #[test]
    fn tdatastd_comment() {
        let mut c = TDataStdComment::new(3, "A comment");
        assert!(!c.is_empty());
        assert_eq!(c.get(), "A comment");
    }

    #[test]
    fn tdatastd_integer() {
        let mut i = TDataStdInteger::new(4, 42);
        assert_eq!(i.get(), 42);
        i.set(100);
        assert_eq!(i.get(), 100);
    }

    #[test]
    fn tdatastd_real() {
        let mut r = TDataStdReal::new(5, 3.14);
        assert!((r.get() - 3.14).abs() < 0.01);
        r.set(2.71);
        assert!((r.get() - 2.71).abs() < 0.01);
    }

    #[test]
    fn tdatastd_attribute_dict() {
        let mut ad = TDataStdAttributeDict::new(1);
        ad.add_string(TDataStdAsciiString::new(1, "test"));
        ad.add_integer(TDataStdInteger::new(1, 5));
        ad.add_real(TDataStdReal::new(1, 1.5));
        assert_eq!(ad.nb_attributes(), 3);
    }
}
