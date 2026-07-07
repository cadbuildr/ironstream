// FILE: swdraw_shape_analysis.rs
// occt: SWDRAW_ShapeAnalysis

//! Contains commands to activate package ShapeAnalysis.
//! Faithful port of the Draw command registrar
//! `SWDRAW_ShapeAnalysis::InitCommands` (.cxx): 15 commands, most in the
//! "Shape Healing" group (SWDRAW::GroupName()), with fbprops / fbclose /
//! getareacontour registered under the legacy "DE: old" group, and the
//! `initactor` once-only guard.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecSa {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` (command table only).
#[derive(Default)]
pub struct DrawInterpretorSa {
    commands: HashMap<String, DrawCommandRecSa>,
    registered_inits: HashSet<String>,
}

impl DrawInterpretorSa {
    pub fn new() -> Self {
        DrawInterpretorSa::default()
    }

    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecSa {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecSa> {
        self.commands.get(name)
    }

    pub fn nb_commands(&self) -> usize {
        self.commands.len()
    }

    pub fn names_in_group(&self, group: &str) -> Vec<String> {
        let mut names: Vec<String> = self
            .commands
            .iter()
            .filter(|(_, c)| c.group == group)
            .map(|(n, _)| n.clone())
            .collect();
        names.sort();
        names
    }

    pub fn claim_init_guard(&mut self, module: &str) -> bool {
        self.registered_inits.insert(module.to_string())
    }
}

/// `SWDRAW_ShapeAnalysis` command registrar.
pub struct SwdrawShapeAnalysis;

impl SwdrawShapeAnalysis {
    const SOURCE_FILE: &'static str = "SWDRAW_ShapeAnalysis.cxx";

    /// SWDRAW::GroupName() constant used by most commands.
    pub const GROUP: &'static str = "Shape Healing";
    /// Legacy group of fbprops/fbclose/getareacontour.
    pub const GROUP_OLD: &'static str = "DE: old";

    /// SWDRAW_ShapeAnalysis::InitCommands.
    pub fn init_commands(di: &mut DrawInterpretorSa) {
        if !di.claim_init_guard("SWDRAW_ShapeAnalysis") {
            return;
        }
        let g = Self::GROUP;
        let f = Self::SOURCE_FILE;
        di.add("tolerance", "shape [tolmin tolmax:real]", f, g);
        di.add(
            "projface",
            "nom_face X Y [Z] - returns the closest orthogonal projection if exists",
            f,
            g,
        );
        di.add("projcurve", "nom_edge | curve3d | curve3d first last + X Y Z", f, g);
        di.add("projpcurve", "edge face tol x y z [start_param]", f, g);
        di.add("anaface", "nomface", f, g);
        di.add("statshape", "shape [particul] : stats/particularites", f, g);
        di.add("comptol", "shape [nbpoints]", f, g);
        di.add(
            "freebounds",
            "shp toler [splitclosed [splitopen]] - free bounds; toler <= 0 for shells (no sewing call)",
            f,
            g,
        );

        let groupold = Self::GROUP_OLD;
        di.add(
            "fbprops",
            "shp [toler [splitclosed [splitopen]]] - free bounds properties; toler <= 0 or not specified - for shells (no sewing call)",
            f,
            groupold,
        );
        di.add(
            "fbclose",
            "shp sewtoler closetoler [splitclosed [splitopen]] - closes free bounds; use sewtoler <= 0 for shells (no sewing call)",
            f,
            groupold,
        );
        di.add("getareacontour", "wire ", f, groupold);
        di.add("checkselfintersection", "wire [face]", f, g);
        di.add("checkedge", "edge [face]", f, g);
        di.add("getanasurf", "getanasurf res shape [target [tol [sample]]] ", f, g);
        di.add("getanacurve", "getanacurve res shape [target [tol]]", f, g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_fifteen_commands() {
        let mut di = DrawInterpretorSa::new();
        SwdrawShapeAnalysis::init_commands(&mut di);
        assert_eq!(di.nb_commands(), 15);
        for name in [
            "tolerance",
            "projface",
            "projcurve",
            "projpcurve",
            "anaface",
            "statshape",
            "comptol",
            "freebounds",
            "fbprops",
            "fbclose",
            "getareacontour",
            "checkselfintersection",
            "checkedge",
            "getanasurf",
            "getanacurve",
        ] {
            assert!(di.find(name).is_some(), "missing {name}");
        }
    }

    #[test]
    fn legacy_group_split() {
        let mut di = DrawInterpretorSa::new();
        SwdrawShapeAnalysis::init_commands(&mut di);
        assert_eq!(
            di.names_in_group(SwdrawShapeAnalysis::GROUP_OLD),
            vec!["fbclose", "fbprops", "getareacontour"]
        );
        assert_eq!(di.names_in_group(SwdrawShapeAnalysis::GROUP).len(), 12);
    }

    #[test]
    fn tolerance_help_text() {
        let mut di = DrawInterpretorSa::new();
        SwdrawShapeAnalysis::init_commands(&mut di);
        assert_eq!(di.find("tolerance").unwrap().help, "shape [tolmin tolmax:real]");
        assert_eq!(di.find("statshape").unwrap().group, "Shape Healing");
    }

    #[test]
    fn guard_blocks_second_init() {
        let mut di = DrawInterpretorSa::new();
        SwdrawShapeAnalysis::init_commands(&mut di);
        di.add("comptol", "patched", "x.cxx", "test");
        SwdrawShapeAnalysis::init_commands(&mut di);
        assert_eq!(di.find("comptol").unwrap().help, "patched");
    }
}
