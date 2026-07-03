// FILE: rust/ironstream/crates/ironstream/src/tdf_attribute.rs

//! `TDF_Attribute` — attribute types for the OCAF (OCCT Application Framework)
//! label/attribute data model, mirroring OpenCascade's `TDF` and `TDataStd`
//! packages. Zero dependencies; std only.

// occt: TDF_Attribute / TDataStd

/// Discriminant for the concrete attribute kind stored on a TDF label.
///
// occt: attribute type enum — String, Integer, Real, BooleanArray, ExtString, Guid
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdfAttrType {
    /// Corresponds to `TDataStd_Name` / `TDataStd_AsciiString`.
    String,
    /// Corresponds to `TDataStd_Integer`.
    Integer,
    /// Corresponds to `TDataStd_Real`.
    Real,
    /// Corresponds to `TDataStd_BooleanArray`.
    BooleanArray,
    /// Corresponds to `TDataStd_ExtStringArray` / extended string.
    ExtString,
    /// Corresponds to `TDataStd_UAttribute` (GUID-keyed attribute).
    Guid,
}

// ---------------------------------------------------------------------------
// TdfStringAttr
// ---------------------------------------------------------------------------

/// A string attribute stored on a TDF label.
///
// occt: TDataStd_Name / TDataStd_AsciiString — string attribute; fields: value String
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TdfStringAttr {
    value: String,
}

impl TdfStringAttr {
    /// Create a new string attribute with the given value.
    pub fn new(value: &str) -> Self {
        TdfStringAttr {
            value: value.to_owned(),
        }
    }

    /// Return a reference to the stored string value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Replace the stored string value.
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_owned();
    }

    /// Return the attribute type discriminant.
    pub fn attr_type(&self) -> TdfAttrType {
        TdfAttrType::String
    }
}

// ---------------------------------------------------------------------------
// TdfIntegerAttr
// ---------------------------------------------------------------------------

/// An integer attribute stored on a TDF label.
///
// occt: TDataStd_Integer — integer attribute; fields: value i64
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TdfIntegerAttr {
    value: i64,
}

impl TdfIntegerAttr {
    /// Create a new integer attribute with the given value.
    pub fn new(value: i64) -> Self {
        TdfIntegerAttr { value }
    }

    /// Return the stored integer value.
    pub fn value(&self) -> i64 {
        self.value
    }

    /// Replace the stored integer value.
    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }

    /// Return the attribute type discriminant.
    pub fn attr_type(&self) -> TdfAttrType {
        TdfAttrType::Integer
    }
}

// ---------------------------------------------------------------------------
// TdfRealAttr
// ---------------------------------------------------------------------------

/// A real-number attribute stored on a TDF label.
///
// occt: TDataStd_Real — real number attribute; fields: value f64
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TdfRealAttr {
    value: f64,
}

impl TdfRealAttr {
    /// Create a new real attribute with the given value.
    pub fn new(value: f64) -> Self {
        TdfRealAttr { value }
    }

    /// Return the stored real value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Replace the stored real value.
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    /// Return the attribute type discriminant.
    pub fn attr_type(&self) -> TdfAttrType {
        TdfAttrType::Real
    }
}

// ---------------------------------------------------------------------------
// TdfAttribute — tagged union
// ---------------------------------------------------------------------------

/// A tagged union of the concrete attribute types that can be attached to a
/// TDF label.
///
// occt: TDF_Attribute — union of the above
#[derive(Clone, Debug, PartialEq)]
pub enum TdfAttribute {
    /// A string attribute (`TDataStd_Name` / `TDataStd_AsciiString`).
    StringAttr(TdfStringAttr),
    /// An integer attribute (`TDataStd_Integer`).
    IntegerAttr(TdfIntegerAttr),
    /// A real-number attribute (`TDataStd_Real`).
    RealAttr(TdfRealAttr),
}

impl TdfAttribute {
    /// Return the attribute type discriminant of the contained value.
    pub fn attr_type(&self) -> TdfAttrType {
        match self {
            TdfAttribute::StringAttr(a) => a.attr_type(),
            TdfAttribute::IntegerAttr(a) => a.attr_type(),
            TdfAttribute::RealAttr(a) => a.attr_type(),
        }
    }

    /// Return a reference to the inner [`TdfStringAttr`], or `None`.
    pub fn as_string(&self) -> Option<&TdfStringAttr> {
        match self {
            TdfAttribute::StringAttr(a) => Some(a),
            _ => None,
        }
    }

    /// Return a reference to the inner [`TdfIntegerAttr`], or `None`.
    pub fn as_integer(&self) -> Option<&TdfIntegerAttr> {
        match self {
            TdfAttribute::IntegerAttr(a) => Some(a),
            _ => None,
        }
    }

    /// Return a reference to the inner [`TdfRealAttr`], or `None`.
    pub fn as_real(&self) -> Option<&TdfRealAttr> {
        match self {
            TdfAttribute::RealAttr(a) => Some(a),
            _ => None,
        }
    }
}
