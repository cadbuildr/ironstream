// FILE: stepcaf_control_data_map_of_label_extern_file.rs
// occt: STEPCAFControl_DataMapOfLabelExternFile

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_DataMap<TDF_Label, opencascade::handle<STEPCAFControl_ExternFile>>
//!    STEPCAFControl_DataMapOfLabelExternFile;`
//! plus the companion `STEPCAFControl_DataMapIteratorOfDataMapOfLabelExternFile`.
//!
//! The OCAF label and the extern-file record are modeled as small local
//! helper types holding exactly what the map manipulates: a label is its
//! tag path in the data framework tree; an extern file carries the file
//! name, the load status and the root label of the transferred shape.

use std::collections::HashMap;
use std::rc::Rc;

/// Local stand-in for `TDF_Label`: identified by its tag path from root.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CafLabelPathLef {
    /// Tag list from the document root, e.g. `[0, 1, 3]` for label "0:1:3".
    pub tags: Vec<i32>,
}

impl CafLabelPathLef {
    pub fn new(tags: &[i32]) -> Self {
        CafLabelPathLef {
            tags: tags.to_vec(),
        }
    }

    /// Entry format used by TDF_Tool::Entry ("0:1:3").
    pub fn entry(&self) -> String {
        self.tags
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(":")
    }

    pub fn is_null(&self) -> bool {
        self.tags.is_empty()
    }
}

/// Return statuses mirroring `IFSelect_ReturnStatus` used by ExternFile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternFileStatusLef {
    RetVoid,
    RetDone,
    RetError,
    RetFail,
    RetStop,
}

/// Local stand-in for `STEPCAFControl_ExternFile` (handle-managed record).
#[derive(Clone, Debug)]
pub struct StepcafExternFileRecLef {
    pub name: String,
    pub load_status: ExternFileStatusLef,
    pub transfer_status: ExternFileStatusLef,
    pub write_status: ExternFileStatusLef,
    /// Root label of the shape corresponding to this extern file.
    pub label: CafLabelPathLef,
}

impl StepcafExternFileRecLef {
    pub fn new(name: &str) -> Self {
        StepcafExternFileRecLef {
            name: name.to_string(),
            load_status: ExternFileStatusLef::RetVoid,
            transfer_status: ExternFileStatusLef::RetVoid,
            write_status: ExternFileStatusLef::RetVoid,
            label: CafLabelPathLef { tags: Vec::new() },
        }
    }
}

/// Handle alias (OCCT `opencascade::handle<STEPCAFControl_ExternFile>`).
pub type HandleStepcafExternFileLef = Rc<StepcafExternFileRecLef>;

/// `STEPCAFControl_DataMapOfLabelExternFile`: NCollection_DataMap semantics
/// over std HashMap (Bind returns true when the key is new).
#[derive(Default)]
pub struct StepcafControlDataMapOfLabelExternFile {
    inner: HashMap<CafLabelPathLef, HandleStepcafExternFileLef>,
}

impl StepcafControlDataMapOfLabelExternFile {
    pub fn new() -> Self {
        StepcafControlDataMapOfLabelExternFile {
            inner: HashMap::new(),
        }
    }

    /// NCollection_DataMap::Bind — returns true if the key was not bound yet.
    pub fn bind(&mut self, key: CafLabelPathLef, item: HandleStepcafExternFileLef) -> bool {
        self.inner.insert(key, item).is_none()
    }

    /// NCollection_DataMap::IsBound.
    pub fn is_bound(&self, key: &CafLabelPathLef) -> bool {
        self.inner.contains_key(key)
    }

    /// NCollection_DataMap::Find (Seek-style: None instead of throwing).
    pub fn find(&self, key: &CafLabelPathLef) -> Option<&HandleStepcafExternFileLef> {
        self.inner.get(key)
    }

    /// NCollection_DataMap::UnBind — returns true if the key was removed.
    pub fn un_bind(&mut self, key: &CafLabelPathLef) -> bool {
        self.inner.remove(key).is_some()
    }

    /// NCollection_DataMap::Extent.
    pub fn extent(&self) -> usize {
        self.inner.len()
    }

    /// NCollection_DataMap::IsEmpty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// NCollection_DataMap::Clear.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// `STEPCAFControl_DataMapIteratorOfDataMapOfLabelExternFile`.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&CafLabelPathLef, &HandleStepcafExternFileLef)> {
        self.inner.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_semantics_new_vs_rebind() {
        let mut map = StepcafControlDataMapOfLabelExternFile::new();
        let lab = CafLabelPathLef::new(&[0, 1, 3]);
        let f1 = Rc::new(StepcafExternFileRecLef::new("wheel.stp"));
        assert!(map.bind(lab.clone(), f1));
        // Rebinding the same label replaces the item and returns false.
        let f2 = Rc::new(StepcafExternFileRecLef::new("wheel_v2.stp"));
        assert!(!map.bind(lab.clone(), f2));
        assert_eq!(map.extent(), 1);
        assert_eq!(map.find(&lab).unwrap().name, "wheel_v2.stp");
    }

    #[test]
    fn label_entry_and_unbind() {
        let mut map = StepcafControlDataMapOfLabelExternFile::new();
        let lab = CafLabelPathLef::new(&[0, 1]);
        assert_eq!(lab.entry(), "0:1");
        assert!(!map.is_bound(&lab));
        map.bind(lab.clone(), Rc::new(StepcafExternFileRecLef::new("a.stp")));
        assert!(map.is_bound(&lab));
        assert!(map.un_bind(&lab));
        assert!(!map.un_bind(&lab));
        assert!(map.is_empty());
    }

    #[test]
    fn distinct_labels_are_distinct_keys() {
        let mut map = StepcafControlDataMapOfLabelExternFile::new();
        map.bind(
            CafLabelPathLef::new(&[0, 1]),
            Rc::new(StepcafExternFileRecLef::new("a.stp")),
        );
        map.bind(
            CafLabelPathLef::new(&[0, 2]),
            Rc::new(StepcafExternFileRecLef::new("b.stp")),
        );
        assert_eq!(map.extent(), 2);
        let names: Vec<String> = map.iter().map(|(_, v)| v.name.clone()).collect();
        assert!(names.contains(&"a.stp".to_string()));
        assert!(names.contains(&"b.stp".to_string()));
    }

    #[test]
    fn extern_file_default_statuses() {
        let f = StepcafExternFileRecLef::new("x.stp");
        assert_eq!(f.load_status, ExternFileStatusLef::RetVoid);
        assert!(f.label.is_null());
    }
}
