// FILE: swdraw_shape_upgrade.rs
// occt: SWDRAW_ShapeUpgrade

//! Contains commands to activate package ShapeUpgrade.
//! Faithful port of `SWDRAW_ShapeUpgrade::InitCommands` (.cxx): the 27
//! divide/convert/split commands (DT_* legacy testing commands plus
//! shellsolid/offset/split/unify utilities) registered under
//! "Shape Healing" with the `initactor` once-only guard.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecSu {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` (command table only).
#[derive(Default)]
pub struct DrawInterpretorSu {
    commands: HashMap<String, DrawCommandRecSu>,
    registered_inits: HashSet<String>,
}

impl DrawInterpretorSu {
    pub fn new() -> Self {
        DrawInterpretorSu::default()
    }

    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecSu {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecSu> {
        self.commands.get(name)
    }

    pub fn nb_commands(&self) -> usize {
        self.commands.len()
    }

    pub fn claim_init_guard(&mut self, module: &str) -> bool {
        self.registered_inits.insert(module.to_string())
    }
}

/// `SWDRAW_ShapeUpgrade` command registrar.
pub struct SwdrawShapeUpgrade;

impl SwdrawShapeUpgrade {
    const SOURCE_FILE: &'static str = "SWDRAW_ShapeUpgrade.cxx";
    pub const GROUP: &'static str = "Shape Healing";

    /// SWDRAW_ShapeUpgrade::InitCommands (name, help) pairs, in order.
    pub const COMMANDS: [(&'static str, &'static str); 27] = [
        ("DT_ShapeDivide", "DT_ShapeDivide Result Shape Tol: Divides shape with C1 Criterion"),
        (
            "DT_SplitAngle",
            "DT_SplitAngle Result Shape [MaxAngle=95]: Divides revolved surfaces on segments less MaxAngle deg",
        ),
        (
            "DT_ShapeConvert",
            "DT_ShapeConvert Result Shape convert2d convert3d: Converts curves to beziers",
        ),
        (
            "DT_ShapeConvertRev",
            "DT_ShapeConvert Result Shape convert2d convert3d: Converts curves to beziers",
        ),
        (
            "DT_PlaneDividedFace",
            "DT_PlaneDividedFace Result Face Tol: Transfer into a plane with boundary divided",
        ),
        (
            "DT_PlaneGridShell",
            "DT_PlaneGridShell Result NbU NbV {UKnots} {VKnots} Tol : Create a plane grid Shell",
        ),
        (
            "DT_PlaneFaceCommon",
            "DT_PlaneFaceCommon Result Face Shell: Common between a plane Face and a Shell",
        ),
        ("DT_SplitCurve2d", "DT_SplitCurve2d Curve Tol: Splits the curve with C1 criterion"),
        ("DT_SplitCurve", "DT_SplitCurve Curve Tol: Splits the curve with C1 criterion"),
        (
            "DT_SplitSurface",
            "DT_SplitSurface Result Surface/GridSurf Tol: Splits the surface with C1 criterion",
        ),
        (
            "DT_SupportModification",
            "DT_SupportModification Result Shell Surface 2d3dFactor: Surface will support all the\n     faces",
        ),
        ("DT_SpltWire", "DT_SpltWire Result Wire Tol"),
        ("DT_SplitFace", "DT_SplitFace Result Face Tol"),
        ("DT_Debug", "DT_Debug 0/1 : activation/deactivation of the debug messages"),
        ("shellsolid", "option[a-b-c-f] shape result"),
        ("offset2dcurve", "result curve offset"),
        ("offsetcurve", "result curve offset dir"),
        ("splitface", "result face [u usplit1 usplit2...] [v vsplit1 vsplit2 ...]"),
        ("DT_ToBspl", "result shape [options=erop]"),
        ("DT_ClosedSplit", "result shape"),
        ("DT_SplitByArea", "result shape maxarea [preci]"),
        ("DT_SplitByNumber", "result face number [number2]"),
        ("RemoveIntWires", "result minarea wholeshape [faces or wires] [moderemoveface ]"),
        ("removeloc", "result shape [remove_level(see ShapeEnum)]"),
        (
            "unifysamedom",
            "unifysamedom result shape [s1 s2 ...] [-f] [-e] [-nosafe] [+b] [+i] [-t val] [-a val]",
        ),
        ("copytranslate", "result shape dx dy dz"),
        (
            "reshape",
            "\n    reshape : result shape [-replace what with] [-remove what] [-until level] \n    Basic utility for topological modification",
        ),
    ];

    /// SWDRAW_ShapeUpgrade::InitCommands.
    pub fn init_commands(di: &mut DrawInterpretorSu) {
        if !di.claim_init_guard("SWDRAW_ShapeUpgrade") {
            return;
        }
        for (name, help) in Self::COMMANDS {
            di.add(name, help, Self::SOURCE_FILE, Self::GROUP);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_twenty_seven_commands() {
        let mut di = DrawInterpretorSu::new();
        SwdrawShapeUpgrade::init_commands(&mut di);
        assert_eq!(di.nb_commands(), 27);
        for (name, _) in SwdrawShapeUpgrade::COMMANDS {
            assert!(di.find(name).is_some(), "missing {name}");
        }
    }

    #[test]
    fn dt_commands_present_with_help() {
        let mut di = DrawInterpretorSu::new();
        SwdrawShapeUpgrade::init_commands(&mut di);
        assert_eq!(
            di.find("DT_ShapeDivide").unwrap().help,
            "DT_ShapeDivide Result Shape Tol: Divides shape with C1 Criterion"
        );
        assert_eq!(di.find("DT_SpltWire").unwrap().help, "DT_SpltWire Result Wire Tol");
        assert_eq!(di.find("copytranslate").unwrap().help, "result shape dx dy dz");
    }

    #[test]
    fn all_in_shape_healing_group() {
        let mut di = DrawInterpretorSu::new();
        SwdrawShapeUpgrade::init_commands(&mut di);
        for (name, _) in SwdrawShapeUpgrade::COMMANDS {
            assert_eq!(di.find(name).unwrap().group, "Shape Healing");
        }
    }

    #[test]
    fn once_only_guard() {
        let mut di = DrawInterpretorSu::new();
        SwdrawShapeUpgrade::init_commands(&mut di);
        di.add("DT_Debug", "patched", "p.cxx", "p");
        SwdrawShapeUpgrade::init_commands(&mut di);
        assert_eq!(di.find("DT_Debug").unwrap().help, "patched");
    }
}
