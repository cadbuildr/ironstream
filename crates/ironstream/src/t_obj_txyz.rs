// FILE: t_obj_txyz.rs
// occt: TObj_TXYZ

//! Attribute for storing gp_XYZ.
//! Faithful port of `TObj_TXYZ` (.hxx + .cxx): an OCAF attribute holding
//! a gp_XYZ with the standard attribute protocol — GetID (fixed GUID),
//! Set-on-label (find-or-create), Backup on modification, NewEmpty /
//! Restore / Paste and Dump. The OCAF label is modeled locally as an
//! attribute slot keyed by GUID.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// The GUID of TObj_TXYZ ("3bbefb50-e618-11d4-ba38-0060b0ee18ea").
pub const TOBJ_TXYZ_GUID: &str = "3bbefb50-e618-11d4-ba38-0060b0ee18ea";

/// Local stand-in for gp_XYZ.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpXyzTx {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GpXyzTx {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpXyzTx { x, y, z }
    }
}

/// Attribute for storing gp_XYZ.
#[derive(Debug)]
pub struct TObjTxyz {
    xyz: RefCell<GpXyzTx>,
    /// Backup copy made before modification (transaction abort support).
    backup: RefCell<Option<GpXyzTx>>,
}

pub type HandleTObjTxyz = Rc<TObjTxyz>;

impl Default for TObjTxyz {
    fn default() -> Self {
        TObjTxyz::new()
    }
}

impl TObjTxyz {
    /// Empty constructor.
    pub fn new() -> Self {
        TObjTxyz {
            xyz: RefCell::new(GpXyzTx::default()),
            backup: RefCell::new(None),
        }
    }

    /// TObj_TXYZ::GetID.
    pub fn get_id() -> &'static str {
        TOBJ_TXYZ_GUID
    }

    /// TObj_TXYZ::ID.
    pub fn id(&self) -> &'static str {
        Self::get_id()
    }

    /// Static Set: finds the attribute on the label or creates and adds
    /// it, then sets the XYZ.
    pub fn set_on_label(label: &mut OcafLabelSlotTx, xyz: GpXyzTx) -> HandleTObjTxyz {
        let attr = if let Some(existing) = label.find_attribute(Self::get_id()) {
            existing
        } else {
            let a: HandleTObjTxyz = Rc::new(TObjTxyz::new());
            label.add_attribute(a.clone());
            a
        };
        attr.set(xyz);
        attr
    }

    /// Sets the XYZ (Backup() first, as in the C++ implementation).
    pub fn set(&self, xyz: GpXyzTx) {
        *self.backup.borrow_mut() = Some(*self.xyz.borrow());
        *self.xyz.borrow_mut() = xyz;
    }

    /// Returns the XYZ.
    pub fn get(&self) -> GpXyzTx {
        *self.xyz.borrow()
    }

    /// NewEmpty — fresh attribute for the copy algorithm.
    pub fn new_empty(&self) -> HandleTObjTxyz {
        Rc::new(TObjTxyz::new())
    }

    /// Restore — takes the value back from `with` (transaction abort).
    pub fn restore(&self, with: &TObjTxyz) {
        *self.xyz.borrow_mut() = with.get();
    }

    /// Paste — copies the value into `into`.
    pub fn paste(&self, into: &TObjTxyz) {
        into.set(self.get());
    }

    /// Dump — "X: x\tY: y\tZ: z" like the C++ stream output.
    pub fn dump(&self) -> String {
        let a = self.get();
        format!("X: {}\tY: {}\tZ: {}", a.x, a.y, a.z)
    }

    /// The value saved by the last Backup() (test observability).
    pub fn backup_value(&self) -> Option<GpXyzTx> {
        *self.backup.borrow()
    }
}

/// Local stand-in for a `TDF_Label`'s attribute set (GUID -> attribute).
#[derive(Default)]
pub struct OcafLabelSlotTx {
    attributes: HashMap<String, HandleTObjTxyz>,
}

impl OcafLabelSlotTx {
    pub fn new() -> Self {
        OcafLabelSlotTx::default()
    }

    pub fn find_attribute(&self, guid: &str) -> Option<HandleTObjTxyz> {
        self.attributes.get(guid).cloned()
    }

    pub fn add_attribute(&mut self, attr: HandleTObjTxyz) {
        self.attributes.insert(attr.id().to_string(), attr);
    }

    pub fn nb_attributes(&self) -> usize {
        self.attributes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_is_fixed() {
        let a = TObjTxyz::new();
        assert_eq!(a.id(), "3bbefb50-e618-11d4-ba38-0060b0ee18ea");
        assert_eq!(TObjTxyz::get_id(), a.id());
    }

    #[test]
    fn set_on_label_creates_then_reuses() {
        let mut label = OcafLabelSlotTx::new();
        let a1 = TObjTxyz::set_on_label(&mut label, GpXyzTx::new(1.0, 2.0, 3.0));
        assert_eq!(label.nb_attributes(), 1);
        let a2 = TObjTxyz::set_on_label(&mut label, GpXyzTx::new(4.0, 5.0, 6.0));
        assert!(Rc::ptr_eq(&a1, &a2), "existing attribute reused");
        assert_eq!(label.nb_attributes(), 1);
        assert_eq!(a1.get(), GpXyzTx::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn set_backs_up_previous_value() {
        let a = TObjTxyz::new();
        a.set(GpXyzTx::new(1.0, 1.0, 1.0));
        a.set(GpXyzTx::new(2.0, 2.0, 2.0));
        assert_eq!(a.backup_value(), Some(GpXyzTx::new(1.0, 1.0, 1.0)));
        assert_eq!(a.get(), GpXyzTx::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn restore_and_paste_roundtrip() {
        let src = TObjTxyz::new();
        src.set(GpXyzTx::new(7.0, 8.0, 9.0));
        let dst = src.new_empty();
        assert_eq!(dst.get(), GpXyzTx::default());
        src.paste(&dst);
        assert_eq!(dst.get(), GpXyzTx::new(7.0, 8.0, 9.0));
        let restored = TObjTxyz::new();
        restored.restore(&src);
        assert_eq!(restored.get(), GpXyzTx::new(7.0, 8.0, 9.0));
    }

    #[test]
    fn dump_format() {
        let a = TObjTxyz::new();
        a.set(GpXyzTx::new(1.5, -2.0, 0.0));
        assert_eq!(a.dump(), "X: 1.5\tY: -2\tZ: 0");
    }
}
