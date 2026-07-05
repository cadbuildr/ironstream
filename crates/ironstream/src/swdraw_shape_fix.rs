// FILE: swdraw_shape_fix.rs
// occt: SWDRAW_ShapeFix

//! Contains commands to activate package ShapeFix.
//! Faithful port of `SWDRAW_ShapeFix::InitCommands` (.cxx): thirteen
//! shape-healing commands registered under "Shape Healing"
//! (SWDRAW::GroupName()) with the `initactor` once-only guard.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecSf {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` (command table only).
#[derive(Default)]
pub struct DrawInterpretorSf {
    commands: HashMap<String, DrawCommandRecSf>,
    registered_inits: HashSet<String>,
}

impl DrawInterpretorSf {
    pub fn new() -> Self {
        DrawInterpretorSf::default()
    }

    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecSf {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecSf> {
        self.commands.get(name)
    }

    pub fn nb_commands(&self) -> usize {
        self.commands.len()
    }

    pub fn claim_init_guard(&mut self, module: &str) -> bool {
        self.registered_inits.insert(module.to_string())
    }
}

/// `SWDRAW_ShapeFix` command registrar.
pub struct SwdrawShapeFix;

impl SwdrawShapeFix {
    const SOURCE_FILE: &'static str = "SWDRAW_ShapeFix.cxx";
    pub const GROUP: &'static str = "Shape Healing";

    /// The thirteen commands of SWDRAW_ShapeFix::InitCommands.
    pub const COMMAND_NAMES: [&'static str; 13] = [
        "edgesameparam",
        "settolerance",
        "stwire",
        "reface",
        "fixshape",
        "testfill",
        "fixwgaps",
        "fixsmall",
        "fixsmalledges",
        "fixsmallfaces",
        "checkoverlapedges",
        "checkfclass2d",
        "connectedges",
    ];

    /// SWDRAW_ShapeFix::InitCommands.
    pub fn init_commands(di: &mut DrawInterpretorSf) {
        if !di.claim_init_guard("SWDRAW_ShapeFix") {
            return;
        }
        let g = Self::GROUP;
        let f = Self::SOURCE_FILE;
        di.add("edgesameparam", "nom shape draw ou * [+ option force]", f, g);
        di.add(
            "settolerance",
            "shape [mode=v-e-f-a] val(fix value) or tolmin tolmax",
            f,
            g,
        );
        di.add("stwire", "stwire tout court pour help complet", f, g);
        di.add("reface", "shape result : controle sens wire", f, g);
        di.add(
            "fixshape",
            "res shape [preci [maxpreci]] [{switches}]\n   [-maxtaila <degrees>] [-maxtailw <width>]",
            f,
            g,
        );
        di.add("testfill", "result edge1 edge2", f, g);
        di.add("fixwgaps", "result shape [toler=0]", f, g);
        di.add("fixsmall", "result shape [toler=1.]", f, g);
        di.add("fixsmalledges", "result shape [toler mode amxangle]", f, g);
        di.add("fixsmallfaces", "result shape [toler=1.]", f, g);
        di.add("checkoverlapedges", "edge1 edge2 [toler domaindist]", f, g);
        di.add("checkfclass2d", "face ucoord vcoord [tol]", f, g);
        di.add("connectedges", "res shape [toler shared]", f, g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_all_thirteen() {
        let mut di = DrawInterpretorSf::new();
        SwdrawShapeFix::init_commands(&mut di);
        assert_eq!(di.nb_commands(), 13);
        for name in SwdrawShapeFix::COMMAND_NAMES {
            let cmd = di.find(name).expect(name);
            assert_eq!(cmd.group, "Shape Healing");
        }
    }

    #[test]
    fn fixshape_multiline_help() {
        let mut di = DrawInterpretorSf::new();
        SwdrawShapeFix::init_commands(&mut di);
        let help = &di.find("fixshape").unwrap().help;
        assert!(help.starts_with("res shape [preci [maxpreci]]"));
        assert!(help.contains("-maxtaila <degrees>"));
    }

    #[test]
    fn defaults_in_help_texts() {
        let mut di = DrawInterpretorSf::new();
        SwdrawShapeFix::init_commands(&mut di);
        assert_eq!(di.find("fixwgaps").unwrap().help, "result shape [toler=0]");
        assert_eq!(di.find("fixsmall").unwrap().help, "result shape [toler=1.]");
    }

    #[test]
    fn once_only_guard() {
        let mut di = DrawInterpretorSf::new();
        SwdrawShapeFix::init_commands(&mut di);
        di.add("reface", "patched", "p.cxx", "p");
        SwdrawShapeFix::init_commands(&mut di);
        assert_eq!(di.find("reface").unwrap().help, "patched");
    }
}
