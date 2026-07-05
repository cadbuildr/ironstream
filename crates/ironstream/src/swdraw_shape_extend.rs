// FILE: swdraw_shape_extend.rs
// occt: SWDRAW_ShapeExtend

//! Contains commands to activate package ShapeExtend.
//! Faithful port of `SWDRAW_ShapeExtend::InitCommands` (.cxx): a single
//! command `sortcompound` registered in the "Shape Healing" group with
//! the `initactor` once-only guard.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecSe {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` (command table only).
#[derive(Default)]
pub struct DrawInterpretorSe {
    commands: HashMap<String, DrawCommandRecSe>,
    registered_inits: HashSet<String>,
}

impl DrawInterpretorSe {
    pub fn new() -> Self {
        DrawInterpretorSe::default()
    }

    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecSe {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecSe> {
        self.commands.get(name)
    }

    pub fn nb_commands(&self) -> usize {
        self.commands.len()
    }

    pub fn claim_init_guard(&mut self, module: &str) -> bool {
        self.registered_inits.insert(module.to_string())
    }
}

/// `SWDRAW_ShapeExtend` command registrar.
pub struct SwdrawShapeExtend;

impl SwdrawShapeExtend {
    const SOURCE_FILE: &'static str = "SWDRAW_ShapeExtend.cxx";
    pub const GROUP: &'static str = "Shape Healing";

    /// SWDRAW_ShapeExtend::InitCommands.
    pub fn init_commands(di: &mut DrawInterpretorSe) {
        if !di.claim_init_guard("SWDRAW_ShapeExtend") {
            return;
        }
        di.add(
            "sortcompound",
            "shape_entree shape_result type=v-e-w-f-s-so [mode=n-e-c-x]",
            Self::SOURCE_FILE,
            Self::GROUP,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_sortcompound_only() {
        let mut di = DrawInterpretorSe::new();
        SwdrawShapeExtend::init_commands(&mut di);
        assert_eq!(di.nb_commands(), 1);
        let cmd = di.find("sortcompound").unwrap();
        assert_eq!(cmd.help, "shape_entree shape_result type=v-e-w-f-s-so [mode=n-e-c-x]");
        assert_eq!(cmd.group, "Shape Healing");
        assert_eq!(cmd.source_file, "SWDRAW_ShapeExtend.cxx");
    }

    #[test]
    fn guard_is_once_only() {
        let mut di = DrawInterpretorSe::new();
        SwdrawShapeExtend::init_commands(&mut di);
        di.add("sortcompound", "patched", "p.cxx", "p");
        SwdrawShapeExtend::init_commands(&mut di);
        assert_eq!(di.find("sortcompound").unwrap().help, "patched");
    }
}
