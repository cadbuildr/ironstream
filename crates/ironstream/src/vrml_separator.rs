// FILE: vrml_separator.rs
// occt: Vrml_Separator
//
// Faithful port of OCCT Vrml_Separator (DataExchange/TKDEVRML/Vrml/
// Vrml_Separator.hxx/.cxx): the VRML 1.0 `Separator` grouping node.
// Print is stateful: the first call opens the node (`Separator {` plus the
// optional renderCulling field when it is not AUTO), the second call closes
// it with `}` — mirroring the myFlagPrint toggle of the C++ class.

/// Local model of the Vrml_SeparatorRenderCulling enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlSeparatorRenderCulling {
    Off,
    On,
    Auto,
}

/// Port of Vrml_Separator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrmlSeparator {
    my_render_culling: VrmlSeparatorRenderCulling,
    my_flag_print: bool,
}

impl VrmlSeparator {
    /// Vrml_Separator(): renderCulling defaults to Vrml_AUTO.
    pub fn new() -> Self {
        VrmlSeparator {
            my_render_culling: VrmlSeparatorRenderCulling::Auto,
            my_flag_print: false,
        }
    }

    /// Vrml_Separator(const Vrml_SeparatorRenderCulling aRenderCulling).
    pub fn with_render_culling(a_render_culling: VrmlSeparatorRenderCulling) -> Self {
        VrmlSeparator {
            my_render_culling: a_render_culling,
            my_flag_print: false,
        }
    }

    pub fn set_render_culling(&mut self, a_render_culling: VrmlSeparatorRenderCulling) {
        self.my_render_culling = a_render_culling;
    }

    pub fn render_culling(&self) -> VrmlSeparatorRenderCulling {
        self.my_render_culling
    }

    /// Standard_OStream& Print(Standard_OStream&) — non-const in OCCT:
    /// alternates between opening and closing the Separator node.
    pub fn print(&mut self, an_ostream: &mut String) {
        if !self.my_flag_print {
            an_ostream.push_str("Separator {\n");
            if self.my_render_culling != VrmlSeparatorRenderCulling::Auto {
                if self.my_render_culling == VrmlSeparatorRenderCulling::On {
                    an_ostream.push_str("    renderCulling\tON\n");
                } else {
                    an_ostream.push_str("    renderCulling\tOFF\n");
                }
            }
            self.my_flag_print = true;
        } else {
            an_ostream.push_str("}\n");
            self.my_flag_print = false;
        }
    }
}

impl Default for VrmlSeparator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_open_then_close() {
        let mut sep = VrmlSeparator::new();
        assert_eq!(sep.render_culling(), VrmlSeparatorRenderCulling::Auto);
        let mut out = String::new();
        sep.print(&mut out);
        assert_eq!(out, "Separator {\n");
        sep.print(&mut out);
        assert_eq!(out, "Separator {\n}\n");
        // toggle resets: a third call opens again
        sep.print(&mut out);
        assert_eq!(out, "Separator {\n}\nSeparator {\n");
    }

    #[test]
    fn render_culling_on_field() {
        let mut sep = VrmlSeparator::with_render_culling(VrmlSeparatorRenderCulling::On);
        let mut out = String::new();
        sep.print(&mut out);
        assert_eq!(out, "Separator {\n    renderCulling\tON\n");
        sep.print(&mut out);
        assert_eq!(out, "Separator {\n    renderCulling\tON\n}\n");
    }

    #[test]
    fn render_culling_off_field() {
        let mut sep = VrmlSeparator::new();
        sep.set_render_culling(VrmlSeparatorRenderCulling::Off);
        let mut out = String::new();
        sep.print(&mut out);
        assert_eq!(out, "Separator {\n    renderCulling\tOFF\n");
    }
}
