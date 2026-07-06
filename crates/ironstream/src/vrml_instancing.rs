// FILE: vrml_instancing.rs
// occt: Vrml_Instancing
//
// Faithful port of OCCT Vrml_Instancing (DataExchange/TKDEVRML/Vrml/
// Vrml_Instancing.hxx): the VRML 1.0 instancing reference node (USE),
// storing the name of an already-defined node to reference it. The Print
// method outputs "USE <name>" in VRML 1.0 syntax.

/// Port of Vrml_Instancing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrmlInstancing {
    my_string: String,
}

impl VrmlInstancing {
    /// Vrml_Instancing(const TCollection_AsciiString& aString): the node name to USE.
    pub fn new(a_string: String) -> Self {
        VrmlInstancing {
            my_string: a_string,
        }
    }

    pub fn set_string(&mut self, a_string: String) {
        self.my_string = a_string;
    }

    pub fn string(&self) -> &str {
        &self.my_string
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("USE ");
        an_ostream.push_str(&self.my_string);
        an_ostream.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instancing_prints_use_keyword() {
        let inst = VrmlInstancing::new("MyNode".to_string());
        assert_eq!(inst.string(), "MyNode");
        let mut out = String::new();
        inst.print(&mut out);
        assert_eq!(out, "USE MyNode\n");
    }

    #[test]
    fn setter_updates_string() {
        let mut inst = VrmlInstancing::new("First".to_string());
        inst.set_string("Second".to_string());
        assert_eq!(inst.string(), "Second");
        let mut out = String::new();
        inst.print(&mut out);
        assert_eq!(out, "USE Second\n");
    }

    #[test]
    fn empty_name() {
        let inst = VrmlInstancing::new(String::new());
        let mut out = String::new();
        inst.print(&mut out);
        assert_eq!(out, "USE \n");
    }
}
