// FILE: swdraw_shape_custom.rs
// occt: SWDRAW_ShapeCustom

//! Contains commands to activate package ShapeCustom.
//! Faithful port of `SWDRAW_ShapeCustom::InitCommands` (.cxx): the five
//! shape customization commands registered in the "Shape Healing" group
//! (SWDRAW::GroupName()) with the `initactor` once-only guard.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecSc {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` (command table only).
#[derive(Default)]
pub struct DrawInterpretorSc {
    commands: HashMap<String, DrawCommandRecSc>,
    registered_inits: HashSet<String>,
}

impl DrawInterpretorSc {
    pub fn new() -> Self {
        DrawInterpretorSc::default()
    }

    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecSc {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecSc> {
        self.commands.get(name)
    }

    pub fn nb_commands(&self) -> usize {
        self.commands.len()
    }

    pub fn claim_init_guard(&mut self, module: &str) -> bool {
        self.registered_inits.insert(module.to_string())
    }
}

/// `SWDRAW_ShapeCustom` command registrar.
pub struct SwdrawShapeCustom;

impl SwdrawShapeCustom {
    const SOURCE_FILE: &'static str = "SWDRAW_ShapeCustom.cxx";
    pub const GROUP: &'static str = "Shape Healing";

    /// SWDRAW_ShapeCustom::InitCommands.
    pub fn init_commands(di: &mut DrawInterpretorSc) {
        if !di.claim_init_guard("SWDRAW_ShapeCustom") {
            return;
        }
        let g = Self::GROUP;
        let f = Self::SOURCE_FILE;
        di.add("directfaces", "directfaces result shape", f, g);
        di.add("expshape", "expshape shape maxdegree maxseg [min_continuity]", f, g);
        di.add("scaleshape", "scaleshape result shape scale", f, g);
        di.add(
            "bsplres",
            "BSplineRestriction result shape tol3d tol2d reqdegree reqnbsegments continuity3d continuity2d PriorDeg RationalConvert",
            f,
            g,
        );
        di.add("convtorevol", "convtorevol result shape", f, g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_five_commands() {
        let mut di = DrawInterpretorSc::new();
        SwdrawShapeCustom::init_commands(&mut di);
        assert_eq!(di.nb_commands(), 5);
        for name in ["directfaces", "expshape", "scaleshape", "bsplres", "convtorevol"] {
            let cmd = di.find(name).expect(name);
            assert_eq!(cmd.group, "Shape Healing");
            assert_eq!(cmd.source_file, "SWDRAW_ShapeCustom.cxx");
        }
    }

    #[test]
    fn help_texts() {
        let mut di = DrawInterpretorSc::new();
        SwdrawShapeCustom::init_commands(&mut di);
        assert_eq!(di.find("scaleshape").unwrap().help, "scaleshape result shape scale");
        assert!(di.find("bsplres").unwrap().help.starts_with("BSplineRestriction"));
    }

    #[test]
    fn once_only_guard() {
        let mut di = DrawInterpretorSc::new();
        SwdrawShapeCustom::init_commands(&mut di);
        di.add("expshape", "changed", "y.cxx", "z");
        SwdrawShapeCustom::init_commands(&mut di);
        assert_eq!(di.find("expshape").unwrap().help, "changed");
        assert_eq!(di.nb_commands(), 5);
    }
}
