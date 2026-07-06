// FILE: vrml_group.rs
// occt: Vrml_Group
//
// Faithful port of OCCT Vrml_Group (DataExchange/TKDEVRML/Vrml/Vrml_Group.hxx):
// the VRML 1.0 `Group` node, a simple container for child nodes.
// Print outputs the opening `Group {` with no fields (always empty in VRML 1.0).

/// Port of Vrml_Group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VrmlGroup;

impl VrmlGroup {
    /// Vrml_Group(): no-op constructor.
    pub fn new() -> Self {
        VrmlGroup
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    /// Outputs the VRML 1.0 Group node header.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("Group {\n");
        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_prints_standard_node() {
        let group = VrmlGroup::new();
        let mut out = String::new();
        group.print(&mut out);
        assert_eq!(out, "Group {\n}\n");
    }

    #[test]
    fn default_equals_new() {
        let group1 = VrmlGroup::new();
        let group2 = VrmlGroup::default();
        assert_eq!(group1, group2);
    }
}
