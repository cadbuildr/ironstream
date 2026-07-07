// FILE: vrml_switch.rs
// occt: Vrml_Switch
//
// Faithful port of OCCT Vrml_Switch (DataExchange/TKDEVRML/Vrml/
// Vrml_Switch.hxx/.cxx): the VRML 1.0 `Switch` grouping node that traverses
// only the chosen child. Default whichChild is -1 (traverse none); Print
// emits the whichChild field only when it differs from -1.

/// Port of Vrml_Switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VrmlSwitch {
    my_which_child: i32,
}

impl VrmlSwitch {
    /// Vrml_Switch(const int aWhichChild = -1).
    pub fn new(a_which_child: i32) -> Self {
        VrmlSwitch {
            my_which_child: a_which_child,
        }
    }

    pub fn set_which_child(&mut self, a_which_child: i32) {
        self.my_which_child = a_which_child;
    }

    pub fn which_child(&self) -> i32 {
        self.my_which_child
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("Switch {\n");
        if self.my_which_child != -1 {
            an_ostream.push_str("    whichChild\t");
            an_ostream.push_str(&self.my_which_child.to_string());
            an_ostream.push('\n');
        }
        an_ostream.push_str("}\n");
    }
}

impl Default for VrmlSwitch {
    fn default() -> Self {
        Self::new(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_prints_empty_node() {
        let s = VrmlSwitch::default();
        assert_eq!(s.which_child(), -1);
        let mut out = String::new();
        s.print(&mut out);
        assert_eq!(out, "Switch {\n}\n");
    }

    #[test]
    fn which_child_field_printed() {
        let s = VrmlSwitch::new(2);
        let mut out = String::new();
        s.print(&mut out);
        assert_eq!(out, "Switch {\n    whichChild\t2\n}\n");
    }

    #[test]
    fn setter_updates_output() {
        let mut s = VrmlSwitch::default();
        s.set_which_child(0);
        assert_eq!(s.which_child(), 0);
        let mut out = String::new();
        s.print(&mut out);
        assert_eq!(out, "Switch {\n    whichChild\t0\n}\n");
    }
}
