// FILE: t_obj_draw.rs
// occt: TObjDRAW

//! Provides DRAW commands for work with TObj data structures.
//! Faithful port of `TObjDRAW` (.hxx + .cxx): `Init` registers the twelve
//! TObj* commands in the "TObj general commands" group behind a
//! `static int initactor` once-only guard; `Factory` (the plugin entry
//! point) first defines the Bin/Xml TObj OCAF formats on the
//! TObj_Application instance and then calls Init. The Draw_Interpretor
//! command table and format registry are modeled locally.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecTod {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` plus the application format table
/// touched by Factory.
#[derive(Default)]
pub struct DrawInterpretorTod {
    commands: HashMap<String, DrawCommandRecTod>,
    registered_inits: HashSet<String>,
    /// OCAF storage formats defined on the TObj application.
    defined_formats: Vec<String>,
}

impl DrawInterpretorTod {
    pub fn new() -> Self {
        DrawInterpretorTod::default()
    }

    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecTod {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecTod> {
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

    /// BinTObjDrivers/XmlTObjDrivers::DefineFormat effect (idempotent).
    pub fn define_format(&mut self, format: &str) {
        if !self.defined_formats.iter().any(|f| f == format) {
            self.defined_formats.push(format.to_string());
        }
    }

    pub fn defined_formats(&self) -> &[String] {
        &self.defined_formats
    }
}

/// `TObjDRAW` package entry points.
pub struct TObjDraw;

impl TObjDraw {
    const SOURCE_FILE: &'static str = "TObjDRAW.cxx";
    pub const GROUP: &'static str = "TObj general commands";

    /// The twelve commands of TObjDRAW::Init (name, help), in order.
    pub const COMMANDS: [(&'static str, &'static str); 12] = [
        ("TObjNew", "DocName \t: Create new TObj model with document named DocName"),
        (
            "TObjSave",
            "DocName [Path] [-stream] \t: Save Model with DocName [by file path] [into a file stream]",
        ),
        ("TObjLoad", "DocName Path [-stream] \t: Load model DocName from file Path [file stream]"),
        ("TObjClose", "DocName\t: Close model DocName"),
        ("TObjAddObj", "DocName ObjName \t: Add object to model document"),
        (
            "TObjSetVal",
            "DocName ObjName1 intVal | -r N r1 r2 ... rN \t: Set one integer or set of real values",
        ),
        ("TObjGetVal", "DocName ObjName1 -i | -r \t: Returns one integer or set of real values"),
        ("TObjSetRef", "DocName ObjName1 ObjName2 \t: Set reference from object1 to object2"),
        ("TObjGetRef", "DocName ObjName \t: Returns list of children objects"),
        (
            "TObjAddChild",
            "DocName ObjName chldName \t: Add child object to indicated object",
        ),
        (
            "TObjGetChildren",
            "DocName ObjName [-all]\t: Returns list of children objects (-all to recurse)",
        ),
        (
            "TObjHasModifications",
            "DocName ObjName \t: Returns status of modification of the object (if object has been modified)",
        ),
    ];

    /// TObjDRAW::Init — initializes all the functions (once-only guard).
    pub fn init(di: &mut DrawInterpretorTod) {
        if !di.claim_init_guard("TObjDRAW") {
            return; // initactor already set
        }
        for (name, help) in Self::COMMANDS {
            di.add(name, help, Self::SOURCE_FILE, Self::GROUP);
        }
    }

    /// TObjDRAW::Factory — plugin entry: define Bin/Xml TObj OCAF
    /// formats on the application, then Init.
    pub fn factory(di: &mut DrawInterpretorTod) {
        // Initialize TObj OCAF formats (done twice in the C++ source;
        // DefineFormat is idempotent).
        di.define_format("BinTObj");
        di.define_format("XmlTObj");
        di.define_format("BinTObj");
        di.define_format("XmlTObj");
        Self::init(di);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_registers_twelve_commands() {
        let mut di = DrawInterpretorTod::new();
        TObjDraw::init(&mut di);
        assert_eq!(di.nb_commands(), 12);
        for (name, _) in TObjDraw::COMMANDS {
            let cmd = di.find(name).expect(name);
            assert_eq!(cmd.group, "TObj general commands");
            assert_eq!(cmd.source_file, "TObjDRAW.cxx");
        }
    }

    #[test]
    fn command_help_spot_checks() {
        let mut di = DrawInterpretorTod::new();
        TObjDraw::init(&mut di);
        assert_eq!(
            di.find("TObjNew").unwrap().help,
            "DocName \t: Create new TObj model with document named DocName"
        );
        assert!(di.find("TObjGetChildren").unwrap().help.contains("-all to recurse"));
    }

    #[test]
    fn factory_defines_formats_then_inits() {
        let mut di = DrawInterpretorTod::new();
        TObjDraw::factory(&mut di);
        assert_eq!(di.defined_formats(), &["BinTObj".to_string(), "XmlTObj".to_string()]);
        assert_eq!(di.nb_commands(), 12);
        assert!(di.find("TObjLoad").is_some());
    }

    #[test]
    fn once_only_guard() {
        let mut di = DrawInterpretorTod::new();
        TObjDraw::init(&mut di);
        di.add("TObjNew", "patched", "p.cxx", "p");
        TObjDraw::init(&mut di);
        assert_eq!(di.find("TObjNew").unwrap().help, "patched");
    }

    #[test]
    fn group_listing() {
        let mut di = DrawInterpretorTod::new();
        TObjDraw::init(&mut di);
        let names = di.names_in_group(TObjDraw::GROUP);
        assert_eq!(names.len(), 12);
        assert!(names.contains(&"TObjHasModifications".to_string()));
    }
}
