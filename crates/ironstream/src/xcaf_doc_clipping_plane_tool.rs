// FILE: xcaf_doc_clipping_plane_tool.rs
// occt: XCAFDoc_ClippingPlaneTool
//
// Tool to store, retrieve, update and remove clipping planes in the
// ClippingPlane section of an XDE document. Labels, plane / name /
// capping attributes are modeled locally; the geometric plane and the
// duplicate detection follow the OCCT implementation.

use std::collections::BTreeMap;

/// GUID of the XCAFDoc_ClippingPlaneTool attribute (from OCCT).
pub const CLIPPING_PLANE_TOOL_GUID: &str = "efd213ea-6dfd-11d4-b9c8-0060b0ee281b";

/// Precision::Angular from OCCT.
pub const PRECISION_ANGULAR: f64 = 1.0e-12;

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let n = dot(v, v).sqrt();
    assert!(n > 0.0, "null direction");
    [v[0] / n, v[1] / n, v[2] / n]
}

/// Angle between two directions, as gp_Dir::Angle.
fn angle(a: [f64; 3], b: [f64; 3]) -> f64 {
    dot(a, b).clamp(-1.0, 1.0).acos()
}

/// Local model of gp_Pln: origin, normal (main axis) and X direction.
#[derive(Debug, Clone, PartialEq)]
pub struct GpPln {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
    pub xdir: [f64; 3],
}

impl GpPln {
    pub fn new(origin: [f64; 3], normal: [f64; 3], xdir: [f64; 3]) -> Self {
        GpPln {
            origin,
            normal: normalize(normal),
            xdir: normalize(xdir),
        }
    }

    pub fn ydir(&self) -> [f64; 3] {
        cross(self.normal, self.xdir)
    }

    /// Same orientation test used by AddClippingPlane duplicate detection.
    pub fn same_orientation(&self, other: &GpPln) -> bool {
        angle(self.normal, other.normal) <= PRECISION_ANGULAR
            && angle(self.xdir, other.xdir) <= PRECISION_ANGULAR
            && angle(self.ydir(), other.ydir()) <= PRECISION_ANGULAR
    }
}

/// Label identifier of a clipping-plane entry (child tag of the tool label).
pub type Label = u32;

/// One clipping-plane child label: plane attribute, optional name
/// attribute, optional capping (TDataStd_Integer) attribute, and an
/// optional view reference (XCAFDoc::ViewRefPlaneGUID tree node).
#[derive(Debug, Clone)]
struct PlaneEntry {
    plane: GpPln,
    name: Option<String>,
    capping: Option<i32>,
    view_ref: bool,
}

/// XCAFDoc_ClippingPlaneTool: manages the ClippingPlane section.
pub struct XCAFDocClippingPlaneTool {
    entries: BTreeMap<Label, PlaneEntry>,
    next_tag: Label,
}

impl XCAFDocClippingPlaneTool {
    /// OCCT GetID.
    pub fn get_id() -> &'static str {
        CLIPPING_PLANE_TOOL_GUID
    }

    /// OCCT ID (dynamic).
    pub fn id(&self) -> &'static str {
        Self::get_id()
    }

    /// OCCT Set/ctor: creates an empty tool.
    pub fn new() -> Self {
        XCAFDocClippingPlaneTool {
            entries: BTreeMap::new(),
            next_tag: 1,
        }
    }

    /// OCCT IsClippingPlane: label is a child of the tool holding a plane.
    pub fn is_clipping_plane(&self, label: Label) -> bool {
        self.entries.contains_key(&label)
    }

    /// OCCT GetClippingPlane: plane, name and capping flag of a label.
    /// Name defaults to empty, capping to false when absent (OCCT leaves
    /// the outputs untouched; callers initialize them so).
    pub fn get_clipping_plane(&self, label: Label) -> Option<(GpPln, String, bool)> {
        self.entries.get(&label).map(|e| {
            (
                e.plane.clone(),
                e.name.clone().unwrap_or_default(),
                e.capping == Some(1),
            )
        })
    }

    /// OCCT AddClippingPlane(plane, name): reuses an existing label if a
    /// plane with equal name and orientation already exists.
    pub fn add_clipping_plane(&mut self, plane: &GpPln, name: &str) -> Label {
        for (lbl, e) in &self.entries {
            let e_name = e.name.clone().unwrap_or_default();
            if e_name != name {
                continue;
            }
            if !e.plane.same_orientation(plane) {
                continue;
            }
            return *lbl;
        }
        let label = self.next_tag;
        self.next_tag += 1;
        self.entries.insert(
            label,
            PlaneEntry {
                plane: plane.clone(),
                name: if name.is_empty() {
                    None
                } else {
                    Some(name.to_string())
                },
                capping: None,
                view_ref: false,
            },
        );
        label
    }

    /// OCCT AddClippingPlane(plane, name, capping).
    pub fn add_clipping_plane_with_capping(
        &mut self,
        plane: &GpPln,
        name: &str,
        capping: bool,
    ) -> Label {
        let label = self.add_clipping_plane(plane, name);
        if let Some(e) = self.entries.get_mut(&label) {
            e.capping = Some(if capping { 1 } else { 0 });
        }
        label
    }

    /// OCCT RemoveClippingPlane: refuses when the label is not a clipping
    /// plane or is referenced by a view (ViewRefPlaneGUID tree node).
    pub fn remove_clipping_plane(&mut self, label: Label) -> bool {
        match self.entries.get(&label) {
            None => false,
            Some(e) if e.view_ref => false,
            Some(_) => {
                self.entries.remove(&label);
                true
            }
        }
    }

    /// OCCT GetClippingPlanes: labels of all clipping planes.
    pub fn get_clipping_planes(&self) -> Vec<Label> {
        self.entries.keys().copied().collect()
    }

    /// OCCT UpdateClippingPlane: replaces plane and name; no-op for
    /// labels that are not clipping planes of this tool.
    pub fn update_clipping_plane(&mut self, label: Label, plane: &GpPln, name: &str) {
        if let Some(e) = self.entries.get_mut(&label) {
            e.plane = plane.clone();
            e.name = Some(name.to_string());
        }
    }

    /// OCCT SetCapping.
    pub fn set_capping(&mut self, label: Label, capping: bool) {
        if let Some(e) = self.entries.get_mut(&label) {
            e.capping = Some(if capping { 1 } else { 0 });
        }
    }

    /// OCCT GetCapping(label): false when absent or not a child.
    pub fn get_capping(&self, label: Label) -> bool {
        self.entries
            .get(&label)
            .map(|e| e.capping == Some(1))
            .unwrap_or(false)
    }

    /// OCCT GetCapping(label, &capping): returns whether the capping
    /// attribute exists, along with its value.
    pub fn get_capping_ex(&self, label: Label) -> Option<bool> {
        self.entries
            .get(&label)
            .and_then(|e| e.capping.map(|c| c == 1))
    }

    /// Marks a plane as referenced by a view (models the
    /// XCAFDoc::ViewRefPlaneGUID tree node set by XCAFDoc_ViewTool).
    pub fn set_view_reference(&mut self, label: Label, referenced: bool) {
        if let Some(e) = self.entries.get_mut(&label) {
            e.view_ref = referenced;
        }
    }
}

impl Default for XCAFDocClippingPlaneTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xy_plane() -> GpPln {
        GpPln::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
    }

    fn yz_plane() -> GpPln {
        GpPln::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
    }

    #[test]
    fn test_guid() {
        assert_eq!(
            XCAFDocClippingPlaneTool::get_id(),
            "efd213ea-6dfd-11d4-b9c8-0060b0ee281b"
        );
        let t = XCAFDocClippingPlaneTool::new();
        assert_eq!(t.id(), CLIPPING_PLANE_TOOL_GUID);
    }

    #[test]
    fn test_add_and_get() {
        let mut t = XCAFDocClippingPlaneTool::new();
        let l = t.add_clipping_plane(&xy_plane(), "section A");
        assert!(t.is_clipping_plane(l));
        assert!(!t.is_clipping_plane(l + 100));

        let (pln, name, capping) = t.get_clipping_plane(l).unwrap();
        assert_eq!(pln, xy_plane());
        assert_eq!(name, "section A");
        assert!(!capping); // no capping attribute yet -> false

        assert_eq!(t.get_clipping_planes(), vec![l]);
    }

    #[test]
    fn test_add_duplicate_reuses_label() {
        let mut t = XCAFDocClippingPlaneTool::new();
        let l1 = t.add_clipping_plane(&xy_plane(), "p");
        let l2 = t.add_clipping_plane(&xy_plane(), "p");
        assert_eq!(l1, l2);
        assert_eq!(t.get_clipping_planes().len(), 1);

        // Same plane, different name -> new label.
        let l3 = t.add_clipping_plane(&xy_plane(), "q");
        assert_ne!(l1, l3);
        // Same name, different orientation -> new label.
        let l4 = t.add_clipping_plane(&yz_plane(), "p");
        assert_ne!(l1, l4);
        assert_eq!(t.get_clipping_planes().len(), 3);
    }

    #[test]
    fn test_capping() {
        let mut t = XCAFDocClippingPlaneTool::new();
        let l = t.add_clipping_plane_with_capping(&xy_plane(), "c", true);
        assert!(t.get_capping(l));
        assert_eq!(t.get_capping_ex(l), Some(true));

        t.set_capping(l, false);
        assert!(!t.get_capping(l));
        assert_eq!(t.get_capping_ex(l), Some(false));

        // Label without capping attribute.
        let l2 = t.add_clipping_plane(&yz_plane(), "n");
        assert!(!t.get_capping(l2));
        assert_eq!(t.get_capping_ex(l2), None);

        // Unknown label.
        assert!(!t.get_capping(9999));
        assert_eq!(t.get_capping_ex(9999), None);
    }

    #[test]
    fn test_update_clipping_plane() {
        let mut t = XCAFDocClippingPlaneTool::new();
        let l = t.add_clipping_plane(&xy_plane(), "old");
        t.update_clipping_plane(l, &yz_plane(), "new");
        let (pln, name, _) = t.get_clipping_plane(l).unwrap();
        assert_eq!(pln, yz_plane());
        assert_eq!(name, "new");

        // Update of a non-existing label is a no-op.
        t.update_clipping_plane(777, &xy_plane(), "x");
        assert!(!t.is_clipping_plane(777));
    }

    #[test]
    fn test_remove_clipping_plane() {
        let mut t = XCAFDocClippingPlaneTool::new();
        let l = t.add_clipping_plane(&xy_plane(), "r");
        assert!(t.remove_clipping_plane(l));
        assert!(!t.is_clipping_plane(l));
        // Removing again fails.
        assert!(!t.remove_clipping_plane(l));
    }

    #[test]
    fn test_remove_refused_when_referenced_by_view() {
        let mut t = XCAFDocClippingPlaneTool::new();
        let l = t.add_clipping_plane(&xy_plane(), "v");
        t.set_view_reference(l, true);
        assert!(!t.remove_clipping_plane(l));
        assert!(t.is_clipping_plane(l));

        t.set_view_reference(l, false);
        assert!(t.remove_clipping_plane(l));
    }

    #[test]
    fn test_labels_are_stable_and_ordered() {
        let mut t = XCAFDocClippingPlaneTool::new();
        let l1 = t.add_clipping_plane(&xy_plane(), "a");
        let l2 = t.add_clipping_plane(&xy_plane(), "b");
        let l3 = t.add_clipping_plane(&xy_plane(), "c");
        t.remove_clipping_plane(l2);
        assert_eq!(t.get_clipping_planes(), vec![l1, l3]);
        // New labels never reuse removed tags (TDF_TagSource behavior).
        let l4 = t.add_clipping_plane(&xy_plane(), "d");
        assert!(l4 > l3);
    }
}
