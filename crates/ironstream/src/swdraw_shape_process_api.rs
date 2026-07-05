// FILE: swdraw_shape_process_api.rs
// occt: SWDRAW_ShapeProcessAPI

//! Contains commands to activate package ShapeProcessAPI.
//! Faithful port of `SWDRAW_ShapeProcessAPI::InitCommands` (.cxx):
//! guarded by `static bool initactor`, registers the single
//! `DT_ApplySeq` command ("DT_ApplySeq result shape rscfilename
//! [prefix]") in the "Shape Healing" group.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecSpa {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` (command table only).
#[derive(Default)]
pub struct DrawInterpretorSpa {
    commands: HashMap<String, DrawCommandRecSpa>,
    registered_inits: HashSet<String>,
}

impl DrawInterpretorSpa {
    pub fn new() -> Self {
        DrawInterpretorSpa::default()
    }

    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecSpa {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecSpa> {
        self.commands.get(name)
    }

    pub fn nb_commands(&self) -> usize {
        self.commands.len()
    }

    pub fn claim_init_guard(&mut self, module: &str) -> bool {
        self.registered_inits.insert(module.to_string())
    }
}

/// `SWDRAW_ShapeProcessAPI` command registrar.
pub struct SwdrawShapeProcessApi;

impl SwdrawShapeProcessApi {
    const SOURCE_FILE: &'static str = "SWDRAW_ShapeProcessAPI.cxx";
    pub const GROUP: &'static str = "Shape Healing";

    /// SWDRAW_ShapeProcessAPI::InitCommands.
    pub fn init_commands(di: &mut DrawInterpretorSpa) {
        if !di.claim_init_guard("SWDRAW_ShapeProcessAPI") {
            return;
        }
        di.add(
            "DT_ApplySeq",
            "DT_ApplySeq result shape rscfilename [prefix]",
            Self::SOURCE_FILE,
            Self::GROUP,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_dt_applyseq() {
        let mut di = DrawInterpretorSpa::new();
        SwdrawShapeProcessApi::init_commands(&mut di);
        assert_eq!(di.nb_commands(), 1);
        let cmd = di.find("DT_ApplySeq").unwrap();
        assert_eq!(cmd.help, "DT_ApplySeq result shape rscfilename [prefix]");
        assert_eq!(cmd.group, "Shape Healing");
        assert_eq!(cmd.source_file, "SWDRAW_ShapeProcessAPI.cxx");
    }

    #[test]
    fn once_only_guard() {
        let mut di = DrawInterpretorSpa::new();
        SwdrawShapeProcessApi::init_commands(&mut di);
        di.add("DT_ApplySeq", "patched", "p.cxx", "p");
        SwdrawShapeProcessApi::init_commands(&mut di);
        assert_eq!(di.find("DT_ApplySeq").unwrap().help, "patched");
    }

    #[test]
    fn lookup_of_absent_command() {
        let mut di = DrawInterpretorSpa::new();
        SwdrawShapeProcessApi::init_commands(&mut di);
        assert!(di.find("SPApply").is_none());
    }
}
