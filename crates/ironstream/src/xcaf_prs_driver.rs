// FILE: xcaf_prs_driver.rs
// occt: XCAFPrs_Driver
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

//! Implements a driver for presentation of shapes in a DECAF document.
//! Its only purpose is to initialize and return an XCAFPrs_AISObject
//! on request.
//!
//! Port of OCCT `XCAFPrs_Driver`
//! (src/DataExchange/TKXCAF/XCAFPrs/XCAFPrs_Driver.{hxx,cxx}), which derives
//! from TPrsStd_Driver and provides:
//!   - `Update(label, ais)`: returns false if the label does not carry a
//!     shape, otherwise creates a new XCAFPrs_AISObject on the label and
//!     returns true;
//!   - static `GetID()`: the driver GUID
//!     "5b896afc-3adf-11d4-b9b7-0060b0ee281b".

// ---------------------------------------------------------------------------
// Minimal local models of OCCT dependencies
// ---------------------------------------------------------------------------

/// Minimal stand-in for Standard_GUID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Standard_GUID(pub String);

impl Standard_GUID {
    pub fn new(id: &str) -> Self {
        Standard_GUID(id.to_string())
    }

    pub fn ToCString(&self) -> &str {
        &self.0
    }
}

/// Minimal stand-in for a TDF_Label: it may or may not carry a shape
/// (XCAFDoc_ShapeTool::IsShape in the .cxx).
#[derive(Debug, Clone)]
pub struct TDF_Label {
    pub tag: String,
    pub is_shape: bool,
}

impl TDF_Label {
    pub fn new_shape(tag: &str) -> Self {
        TDF_Label {
            tag: tag.to_string(),
            is_shape: true,
        }
    }

    pub fn new_non_shape(tag: &str) -> Self {
        TDF_Label {
            tag: tag.to_string(),
            is_shape: false,
        }
    }
}

/// Minimal stand-in for XCAFPrs_AISObject (an AIS_InteractiveObject built
/// on a label).
#[derive(Debug, Clone)]
pub struct XCAFPrs_AISObject {
    pub label: TDF_Label,
}

impl XCAFPrs_AISObject {
    pub fn new(label: &TDF_Label) -> Self {
        XCAFPrs_AISObject {
            label: label.clone(),
        }
    }
}

/// Driver for presentation of shapes in DECAF documents.
#[derive(Debug, Clone, Default)]
pub struct XCAFPrs_Driver;

impl XCAFPrs_Driver {
    pub fn new() -> Self {
        XCAFPrs_Driver
    }

    /// Port of XCAFPrs_Driver::Update: returns None when the label is not
    /// a shape label, otherwise builds a new XCAFPrs_AISObject on it.
    /// (The C++ signature returns bool and fills an in-out handle.)
    pub fn Update(&self, label: &TDF_Label) -> Option<XCAFPrs_AISObject> {
        // WARNING! The label can be out of any document
        // (this is a case for reading from a file).
        if !label.is_shape {
            return None;
        }
        Some(XCAFPrs_AISObject::new(label))
    }

    /// Returns the GUID of the driver.
    pub fn GetID() -> Standard_GUID {
        Standard_GUID::new("5b896afc-3adf-11d4-b9b7-0060b0ee281b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id_guid_constant() {
        // GUID string constant from XCAFPrs_Driver.cxx.
        assert_eq!(
            XCAFPrs_Driver::GetID().ToCString(),
            "5b896afc-3adf-11d4-b9b7-0060b0ee281b"
        );
    }

    #[test]
    fn test_get_id_is_stable() {
        assert_eq!(XCAFPrs_Driver::GetID(), XCAFPrs_Driver::GetID());
    }

    #[test]
    fn test_update_on_shape_label() {
        let driver = XCAFPrs_Driver::new();
        let label = TDF_Label::new_shape("0:1:1:1");
        let ais = driver.Update(&label);
        assert!(ais.is_some(), "Update must succeed on a shape label");
        assert_eq!(ais.unwrap().label.tag, "0:1:1:1");
    }

    #[test]
    fn test_update_on_non_shape_label() {
        // XCAFPrs_Driver::Update returns false when IsShape(L) is false.
        let driver = XCAFPrs_Driver::new();
        let label = TDF_Label::new_non_shape("0:1:2");
        assert!(driver.Update(&label).is_none());
    }
}
