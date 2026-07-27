// FILE: standard2.rs
// occt: Standard_Type, Standard_Transient, Standard_GUID

/// A globally unique identifier.
/// occt: Standard_GUID
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StandardGuid {
    pub a: u32,
    pub b: u16,
    pub c: u16,
    pub d: [u8; 8],
}

impl StandardGuid {
    pub const fn new(a: u32, b: u16, c: u16, d: [u8; 8]) -> Self {
        Self { a, b, c, d }
    }

    pub const ZERO: Self = Self { a: 0, b: 0, c: 0, d: [0u8; 8] };

    pub fn is_zero(&self) -> bool { *self == Self::ZERO }

    pub fn to_hex_string(&self) -> String {
        format!("{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.a, self.b, self.c,
            self.d[0], self.d[1],
            self.d[2], self.d[3], self.d[4], self.d[5], self.d[6], self.d[7])
    }
}

impl Default for StandardGuid {
    fn default() -> Self { Self::ZERO }
}

/// Describes a persistent OCCT type (RTTI stub).
/// occt: Standard_Type
#[derive(Clone, Debug)]
pub struct StandardType {
    pub type_name: &'static str,
    pub size: usize,
    pub parent: Option<&'static str>,
}

impl StandardType {
    pub const fn new(type_name: &'static str, size: usize) -> Self {
        Self { type_name, size, parent: None }
    }

    pub const fn with_parent(mut self, parent: &'static str) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn name(&self) -> &str { self.type_name }
    pub fn size_of(&self) -> usize { self.size }

    pub fn is_kind_of(&self, type_name: &str) -> bool {
        self.type_name == type_name || self.parent.map_or(false, |p| p == type_name)
    }
}

/// Base class for reference-counted OCCT objects (stub).
/// occt: Standard_Transient
#[derive(Clone, Debug)]
pub struct StandardTransient {
    pub type_name: &'static str,
    ref_count: i32,
}

impl StandardTransient {
    pub fn new(type_name: &'static str) -> Self {
        Self { type_name, ref_count: 0 }
    }

    pub fn increment_ref(&mut self) { self.ref_count += 1; }
    pub fn decrement_ref(&mut self) -> bool {
        if self.ref_count > 0 { self.ref_count -= 1; }
        self.ref_count == 0
    }
    pub fn ref_count(&self) -> i32 { self.ref_count }
    pub fn dynamic_type(&self) -> &str { self.type_name }
    pub fn is_kind_of(&self, name: &str) -> bool { self.type_name == name }
    pub fn is_instance_of(&self, name: &str) -> bool { self.type_name == name }
}

/// Handle: a reference-counted smart pointer (stub).
/// occt-note: Handle(Standard_Transient) aka opencascade_sys Handle
#[derive(Clone, Debug)]
pub struct Handle<T> {
    pub inner: Option<T>,
}

impl<T> Handle<T> {
    pub fn new(val: T) -> Self { Self { inner: Some(val) } }
    pub fn null() -> Self { Self { inner: None } }
    pub fn is_null(&self) -> bool { self.inner.is_none() }
    pub fn as_ref(&self) -> Option<&T> { self.inner.as_ref() }
    pub fn as_mut(&mut self) -> Option<&mut T> { self.inner.as_mut() }
}

impl<T> Default for Handle<T> {
    fn default() -> Self { Self::null() }
}

/// Global type registry (singleton stub).
/// occt: Standard_Type // registry
#[derive(Clone, Debug, Default)]
pub struct StandardTypeRegistry {
    pub types: Vec<StandardType>,
}

impl StandardTypeRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, t: StandardType) {
        if !self.types.iter().any(|x| x.type_name == t.type_name) {
            self.types.push(t);
        }
    }

    pub fn find(&self, name: &str) -> Option<&StandardType> {
        self.types.iter().find(|t| t.type_name == name)
    }

    pub fn nb_types(&self) -> usize { self.types.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_format() {
        let g = StandardGuid::new(0x12345678, 0xabcd, 0xef01, [1,2,3,4,5,6,7,8]);
        let s = g.to_hex_string();
        assert!(s.starts_with("12345678-abcd-ef01"));
        assert!(!g.is_zero());
    }

    #[test]
    fn guid_zero() {
        let g = StandardGuid::default();
        assert!(g.is_zero());
    }

    #[test]
    fn standard_type_kind_of() {
        let t = StandardType::new("Geom_Curve", 64).with_parent("Standard_Transient");
        assert!(t.is_kind_of("Geom_Curve"));
        assert!(t.is_kind_of("Standard_Transient"));
        assert!(!t.is_kind_of("Geom_Surface"));
    }

    #[test]
    fn transient_ref_count() {
        let mut t = StandardTransient::new("AIS_Shape");
        assert_eq!(t.ref_count(), 0);
        t.increment_ref();
        t.increment_ref();
        assert_eq!(t.ref_count(), 2);
        assert!(!t.decrement_ref());
        assert!(t.decrement_ref());
    }

    #[test]
    fn handle_null() {
        let h: Handle<i32> = Handle::null();
        assert!(h.is_null());
        let h2 = Handle::new(42i32);
        assert!(!h2.is_null());
        assert_eq!(h2.as_ref(), Some(&42));
    }

    #[test]
    fn type_registry() {
        let mut reg = StandardTypeRegistry::new();
        reg.register(StandardType::new("Geom_Curve", 64));
        reg.register(StandardType::new("Geom_Surface", 80));
        reg.register(StandardType::new("Geom_Curve", 64)); // dup
        assert_eq!(reg.nb_types(), 2);
        assert!(reg.find("Geom_Curve").is_some());
        assert!(reg.find("Unknown").is_none());
    }
}
