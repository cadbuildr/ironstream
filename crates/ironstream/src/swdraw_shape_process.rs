// FILE: swdraw_shape_process.rs
// occt: SWDRAW_ShapeProcess

//! Contains commands to activate package ShapeProcess.
//! Faithful port of `SWDRAW_ShapeProcess::InitCommands` (.cxx): guarded by
//! `static bool initactor`, it first registers the ShapeProcess operator
//! library (`ShapeProcess_OperLibrary::Init()`, modeled as a flag on the
//! interpreter) and then adds the single `SPApply` command in the
//! "Shape Healing" group.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecSp {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` (command table only).
#[derive(Default)]
pub struct DrawInterpretorSp {
    commands: HashMap<String, DrawCommandRecSp>,
    registered_inits: HashSet<String>,
    oper_library_initialized: bool,
}

impl DrawInterpretorSp {
    pub fn new() -> Self {
        DrawInterpretorSp::default()
    }

    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecSp {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecSp> {
        self.commands.get(name)
    }

    pub fn nb_commands(&self) -> usize {
        self.commands.len()
    }

    /// Models the C++ `static bool initactor` once-only guard.
    pub fn claim_init_guard(&mut self, module: &str) -> bool {
        self.registered_inits.insert(module.to_string())
    }

    /// ShapeProcess_OperLibrary::Init() effect.
    pub fn init_oper_library(&mut self) {
        self.oper_library_initialized = true;
    }

    pub fn is_oper_library_initialized(&self) -> bool {
        self.oper_library_initialized
    }
}

/// `SWDRAW_ShapeProcess` command registrar.
pub struct SwdrawShapeProcess;

impl SwdrawShapeProcess {
    const SOURCE_FILE: &'static str = "SWDRAW_ShapeProcess.cxx";
    pub const GROUP: &'static str = "Shape Healing";

    /// SWDRAW_ShapeProcess::InitCommands.
    pub fn init_commands(di: &mut DrawInterpretorSp) {
        if !di.claim_init_guard("SWDRAW_ShapeProcess") {
            return;
        }
        di.init_oper_library();
        di.add(
            "SPApply",
            "SPApply result shape rscfilename [sequence]",
            Self::SOURCE_FILE,
            Self::GROUP,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_spapply_and_oper_library() {
        let mut di = DrawInterpretorSp::new();
        assert!(!di.is_oper_library_initialized());
        SwdrawShapeProcess::init_commands(&mut di);
        assert_eq!(di.nb_commands(), 1);
        assert!(di.is_oper_library_initialized());
        let cmd = di.find("SPApply").unwrap();
        assert_eq!(cmd.help, "SPApply result shape rscfilename [sequence]");
        assert_eq!(cmd.group, "Shape Healing");
        assert_eq!(cmd.source_file, "SWDRAW_ShapeProcess.cxx");
    }

    #[test]
    fn once_only_guard() {
        let mut di = DrawInterpretorSp::new();
        SwdrawShapeProcess::init_commands(&mut di);
        di.add("SPApply", "patched", "p.cxx", "p");
        SwdrawShapeProcess::init_commands(&mut di);
        assert_eq!(di.find("SPApply").unwrap().help, "patched");
    }
}
