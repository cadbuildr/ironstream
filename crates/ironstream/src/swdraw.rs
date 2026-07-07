// FILE: swdraw.rs
// occt: SWDRAW

//! Provides DRAW interface to the functionalities of Shape Healing
//! toolkit (SHAPEWORKS Delivery Unit).
//! Faithful port of `SWDRAW::Init` (.cxx): dispatches the InitCommands of
//! the eight Shape* registrars (ShapeTool, ShapeAnalysis, ShapeCustom,
//! ShapeExtend, ShapeFix, ShapeUpgrade, ShapeProcess, ShapeProcessAPI),
//! adds the two location commands (LocSet/LocDump, group "essai"), and
//! registers ShapeProcess operators. `GroupName()` returns
//! "Shape Healing". The `dejadraw` static flag is modeled per interpreter.
//!
//! The sub-registrar command tables are inlined here (self-contained
//! module) exactly as their .cxx Init functions define them.

use std::collections::{HashMap, HashSet};

/// One registered Draw command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawCommandRecSw {
    pub help: String,
    pub source_file: String,
    pub group: String,
}

/// Local model of `Draw_Interpretor` (command table only).
#[derive(Default)]
pub struct DrawInterpretorSw {
    commands: HashMap<String, DrawCommandRecSw>,
    registered_inits: HashSet<String>,
    oper_library_initialized: bool,
    dejadraw: bool,
}

impl DrawInterpretorSw {
    pub fn new() -> Self {
        DrawInterpretorSw::default()
    }

    pub fn add(&mut self, name: &str, help: &str, source_file: &str, group: &str) {
        self.commands.insert(
            name.to_string(),
            DrawCommandRecSw {
                help: help.to_string(),
                source_file: source_file.to_string(),
                group: group.to_string(),
            },
        );
    }

    pub fn find(&self, name: &str) -> Option<&DrawCommandRecSw> {
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

    pub fn is_oper_library_initialized(&self) -> bool {
        self.oper_library_initialized
    }
}

/// `SWDRAW` package entry point.
pub struct Swdraw;

impl Swdraw {
    /// SWDRAW::GroupName — "Shape Healing".
    pub fn group_name() -> &'static str {
        "Shape Healing"
    }

    /// SWDRAW::Init — loads all commands defined in SWDRAW.
    pub fn init(di: &mut DrawInterpretorSw) {
        if !di.dejadraw {
            di.dejadraw = true;
        }

        Self::init_shape_tool(di);
        Self::init_shape_analysis(di);
        Self::init_shape_custom(di);
        Self::init_shape_extend(di);
        Self::init_shape_fix(di);
        Self::init_shape_upgrade(di);
        Self::init_shape_process(di);
        Self::init_shape_process_api(di);

        // locations
        di.add(
            "LocSet",
            "a [b [c]]: set loc b->a; use no args to get help",
            "SWDRAW.cxx",
            "essai",
        );
        di.add("LocDump", "a: dump location of a", "SWDRAW.cxx", "essai");

        // register operators for ShapeProcessing
        di.oper_library_initialized = true;
    }

    fn init_shape_tool(di: &mut DrawInterpretorSw) {
        if !di.claim_init_guard("SWDRAW_ShapeTool") {
            return;
        }
        let g = "DE: old";
        let f = "SWDRAW_ShapeTool.cxx";
        di.add("anaedges", "nom shape", f, g);
        di.add("expwire", "nom wire [nom face]", f, g);
        di.add("ssolid", "nom shell + nouveau nom solid", f, g);
        di.add("samerange", "{ shape | result curve2d first last newfirst newlast }", f, g);
    }

    fn init_shape_analysis(di: &mut DrawInterpretorSw) {
        if !di.claim_init_guard("SWDRAW_ShapeAnalysis") {
            return;
        }
        let g = Self::group_name();
        let f = "SWDRAW_ShapeAnalysis.cxx";
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
        let groupold = "DE: old";
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

    fn init_shape_custom(di: &mut DrawInterpretorSw) {
        if !di.claim_init_guard("SWDRAW_ShapeCustom") {
            return;
        }
        let g = Self::group_name();
        let f = "SWDRAW_ShapeCustom.cxx";
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

    fn init_shape_extend(di: &mut DrawInterpretorSw) {
        if !di.claim_init_guard("SWDRAW_ShapeExtend") {
            return;
        }
        di.add(
            "sortcompound",
            "shape_entree shape_result type=v-e-w-f-s-so [mode=n-e-c-x]",
            "SWDRAW_ShapeExtend.cxx",
            Self::group_name(),
        );
    }

    fn init_shape_fix(di: &mut DrawInterpretorSw) {
        if !di.claim_init_guard("SWDRAW_ShapeFix") {
            return;
        }
        let g = Self::group_name();
        let f = "SWDRAW_ShapeFix.cxx";
        di.add("edgesameparam", "nom shape draw ou * [+ option force]", f, g);
        di.add("settolerance", "shape [mode=v-e-f-a] val(fix value) or tolmin tolmax", f, g);
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

    fn init_shape_upgrade(di: &mut DrawInterpretorSw) {
        if !di.claim_init_guard("SWDRAW_ShapeUpgrade") {
            return;
        }
        let g = Self::group_name();
        let f = "SWDRAW_ShapeUpgrade.cxx";
        let commands: [(&str, &str); 27] = [
            ("DT_ShapeDivide", "DT_ShapeDivide Result Shape Tol: Divides shape with C1 Criterion"),
            ("DT_SplitAngle", "DT_SplitAngle Result Shape [MaxAngle=95]: Divides revolved surfaces on segments less MaxAngle deg"),
            ("DT_ShapeConvert", "DT_ShapeConvert Result Shape convert2d convert3d: Converts curves to beziers"),
            ("DT_ShapeConvertRev", "DT_ShapeConvert Result Shape convert2d convert3d: Converts curves to beziers"),
            ("DT_PlaneDividedFace", "DT_PlaneDividedFace Result Face Tol: Transfer into a plane with boundary divided"),
            ("DT_PlaneGridShell", "DT_PlaneGridShell Result NbU NbV {UKnots} {VKnots} Tol : Create a plane grid Shell"),
            ("DT_PlaneFaceCommon", "DT_PlaneFaceCommon Result Face Shell: Common between a plane Face and a Shell"),
            ("DT_SplitCurve2d", "DT_SplitCurve2d Curve Tol: Splits the curve with C1 criterion"),
            ("DT_SplitCurve", "DT_SplitCurve Curve Tol: Splits the curve with C1 criterion"),
            ("DT_SplitSurface", "DT_SplitSurface Result Surface/GridSurf Tol: Splits the surface with C1 criterion"),
            ("DT_SupportModification", "DT_SupportModification Result Shell Surface 2d3dFactor: Surface will support all the\n     faces"),
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
            ("unifysamedom", "unifysamedom result shape [s1 s2 ...] [-f] [-e] [-nosafe] [+b] [+i] [-t val] [-a val]"),
            ("copytranslate", "result shape dx dy dz"),
            ("reshape", "\n    reshape : result shape [-replace what with] [-remove what] [-until level] \n    Basic utility for topological modification"),
        ];
        for (name, help) in commands {
            di.add(name, help, f, g);
        }
    }

    fn init_shape_process(di: &mut DrawInterpretorSw) {
        if !di.claim_init_guard("SWDRAW_ShapeProcess") {
            return;
        }
        di.oper_library_initialized = true;
        di.add(
            "SPApply",
            "SPApply result shape rscfilename [sequence]",
            "SWDRAW_ShapeProcess.cxx",
            Self::group_name(),
        );
    }

    fn init_shape_process_api(di: &mut DrawInterpretorSw) {
        if !di.claim_init_guard("SWDRAW_ShapeProcessAPI") {
            return;
        }
        di.add(
            "DT_ApplySeq",
            "DT_ApplySeq result shape rscfilename [prefix]",
            "SWDRAW_ShapeProcessAPI.cxx",
            Self::group_name(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_name_is_shape_healing() {
        assert_eq!(Swdraw::group_name(), "Shape Healing");
    }

    #[test]
    fn init_registers_full_command_set() {
        let mut di = DrawInterpretorSw::new();
        Swdraw::init(&mut di);
        // 4 + 15 + 5 + 1 + 13 + 27 + 1 + 1 sub-registrar commands + 2 location commands.
        assert_eq!(di.nb_commands(), 4 + 15 + 5 + 1 + 13 + 27 + 1 + 1 + 2);
        // Spot-checks from each unit.
        for name in [
            "anaedges", "tolerance", "directfaces", "sortcompound", "fixshape",
            "DT_ShapeDivide", "SPApply", "DT_ApplySeq", "LocSet", "LocDump",
        ] {
            assert!(di.find(name).is_some(), "missing {name}");
        }
        assert!(di.is_oper_library_initialized());
    }

    #[test]
    fn location_commands_in_essai_group() {
        let mut di = DrawInterpretorSw::new();
        Swdraw::init(&mut di);
        assert_eq!(di.names_in_group("essai"), vec!["LocDump", "LocSet"]);
        assert_eq!(
            di.find("LocSet").unwrap().help,
            "a [b [c]]: set loc b->a; use no args to get help"
        );
    }

    #[test]
    fn old_group_holds_legacy_commands() {
        let mut di = DrawInterpretorSw::new();
        Swdraw::init(&mut di);
        assert_eq!(
            di.names_in_group("DE: old"),
            vec![
                "anaedges",
                "expwire",
                "fbclose",
                "fbprops",
                "getareacontour",
                "samerange",
                "ssolid"
            ]
        );
    }

    #[test]
    fn double_init_is_stable() {
        let mut di = DrawInterpretorSw::new();
        Swdraw::init(&mut di);
        let n = di.nb_commands();
        Swdraw::init(&mut di);
        assert_eq!(di.nb_commands(), n);
    }
}
