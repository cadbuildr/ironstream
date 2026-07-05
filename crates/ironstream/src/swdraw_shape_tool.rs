// FILE: swdraw_shape_tool.rs
// occt: SWDRAW_ShapeTool

//! Defines functions to control shapes (imported/exported), styled after
//! the Draw command registrar `SWDRAW_ShapeTool::InitCommands` (.cxx).
//! The Draw_Interpretor command table is modeled locally: registration
//! records name, help text, source file and command group, and lookups
//! resolve registered commands. The C++ function-local `initactor` guard
//! (register-once) is reproduced per interpreter.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecSt {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` (command table only).
#[derive(Default)]
pub struct DrawInterpretorSt {
    commands: HashMap<String, DrawCommandRecSt>,
    registered_inits: HashSet<String>,
}

impl DrawInterpretorSt {
    pub fn new() -> Self {
        DrawInterpretorSt::default()
    }

    /// Draw_Interpretor::Add.
    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecSt {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecSt> {
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

    /// Models the C++ `static int initactor` once-only guard:
    /// true when this init module has not run yet.
    pub fn claim_init_guard(&mut self, module: &str) -> bool {
        self.registered_inits.insert(module.to_string())
    }
}

/// `SWDRAW_ShapeTool` command registrar.
pub struct SwdrawShapeTool;

impl SwdrawShapeTool {
    const SOURCE_FILE: &'static str = "SWDRAW_ShapeTool.cxx";

    /// SWDRAW_ShapeTool::InitCommands.
    pub fn init_commands(di: &mut DrawInterpretorSt) {
        if !di.claim_init_guard("SWDRAW_ShapeTool") {
            return; // initactor already set
        }
        let g = "DE: old";
        di.add("anaedges", "nom shape", Self::SOURCE_FILE, g);
        di.add("expwire", "nom wire [nom face]", Self::SOURCE_FILE, g);
        di.add("ssolid", "nom shell + nouveau nom solid", Self::SOURCE_FILE, g);
        di.add(
            "samerange",
            "{ shape | result curve2d first last newfirst newlast }",
            Self::SOURCE_FILE,
            g,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_all_four_commands() {
        let mut di = DrawInterpretorSt::new();
        SwdrawShapeTool::init_commands(&mut di);
        assert_eq!(di.nb_commands(), 4);
        for name in ["anaedges", "expwire", "ssolid", "samerange"] {
            assert!(di.find(name).is_some(), "command {name} not registered");
        }
    }

    #[test]
    fn help_and_group_recorded() {
        let mut di = DrawInterpretorSt::new();
        SwdrawShapeTool::init_commands(&mut di);
        let cmd = di.find("ssolid").unwrap();
        assert_eq!(cmd.help, "nom shell + nouveau nom solid");
        assert_eq!(cmd.group, "DE: old");
        assert_eq!(cmd.source_file, "SWDRAW_ShapeTool.cxx");
        assert_eq!(
            di.names_in_group("DE: old"),
            vec!["anaedges", "expwire", "samerange", "ssolid"]
        );
    }

    #[test]
    fn init_guard_prevents_double_registration() {
        let mut di = DrawInterpretorSt::new();
        SwdrawShapeTool::init_commands(&mut di);
        // Override one command manually, then re-init: guard must skip.
        di.add("anaedges", "overridden", "elsewhere.cxx", "other");
        SwdrawShapeTool::init_commands(&mut di);
        assert_eq!(di.find("anaedges").unwrap().help, "overridden");
    }

    #[test]
    fn unknown_command_not_found() {
        let mut di = DrawInterpretorSt::new();
        SwdrawShapeTool::init_commands(&mut di);
        assert!(di.find("fixshape").is_none(), "other modules' commands absent");
    }
}
