// FILE: xsdrawstl.rs
// occt: XSDRAWSTL
//
// Faithful port of OCCT XSDRAWSTL (Draw/TKXSDRAWSTL/XSDRAWSTL.cxx/.hxx),
// the plugin class registering the STL / MeshVS Draw commands under group
// "XSTEP-STL/VRML":
//   writestl, readstl, meshfromstl, mesh3delem, meshshadcolor,
//   meshlinkcolor, meshmat, meshshrcoef, meshclosed, meshshow, meshhide,
//   meshhidesel, meshshowsel, meshshowall, meshcolors, meshvectors,
//   meshtext, meshdeform, mesh_edge_width, meshinfo
//
// The Draw_Interpretor is modelled by a small local command registry; the
// registration payload (command names, help strings, group, once-only
// guard) and the `readstl` option parsing (-brep / -multi / -mergeAngle
// with its [0,90] degree validation and radian conversion) are real and
// tested.

/// Local model of one registered Draw command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StlDrawCommandEntry {
    pub name: String,
    pub help: String,
    pub group: String,
}

/// Local model of Draw_Interpretor for XSDRAWSTL registration.
#[derive(Debug, Default)]
pub struct StlDrawInterpretor {
    commands: Vec<StlDrawCommandEntry>,
}

impl StlDrawInterpretor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: &str, help: &str, group: &str) {
        self.commands.push(StlDrawCommandEntry {
            name: name.to_string(),
            help: help.to_string(),
            group: group.to_string(),
        });
    }

    pub fn commands(&self) -> &[StlDrawCommandEntry] {
        &self.commands
    }

    pub fn find(&self, name: &str) -> Option<&StlDrawCommandEntry> {
        self.commands.iter().find(|c| c.name == name)
    }
}

/// Options parsed by the `readstl` command argument loop.
#[derive(Debug, Clone, PartialEq)]
pub struct StlReadOptions {
    pub shape_name: String,
    pub file_path: String,
    /// -brep: create a compound of per-triangle faces via StlAPI::Read.
    pub create_comp_of_tris: bool,
    /// -multi: one face per solid in multi-domain files.
    pub multi: bool,
    /// Merge angle in radians; PI/2 means "disabled" (default).
    pub merge_angle: f64,
}

/// Parses on/off tokens like Draw::ParseOnOff ("on"/"1" true, "off"/"0" false).
fn stl_parse_on_off(token: &str) -> Option<bool> {
    let lower = token.to_ascii_lowercase();
    match lower.as_str() {
        "on" | "1" => Some(true),
        "off" | "0" => Some(false),
        _ => None,
    }
}

/// Port of the XSDRAWSTL plugin class.
#[derive(Debug, Default)]
pub struct Xsdrawstl {
    activated: bool,
}

impl Xsdrawstl {
    pub const GROUP: &'static str = "XSTEP-STL/VRML";

    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors the argument loop of the static `readstl` command:
    /// first two positional args are shape name and file path; then
    /// `-brep [on/off]`, `-multi [on/off]`,
    /// `-mergeangle/-smoothangle [angleDeg]` (angle within [0,90], stored
    /// in radians; default when flag given without value is PI/4) and
    /// `-nomergeangle/-nosmoothangle` (reset to PI/2). Unknown arguments or
    /// a missing file path are syntax errors (Err with the message text).
    pub fn readstl_parse_args(args: &[&str]) -> Result<StlReadOptions, String> {
        let mut opts = StlReadOptions {
            shape_name: String::new(),
            file_path: String::new(),
            create_comp_of_tris: false,
            multi: false,
            merge_angle: std::f64::consts::PI / 2.0,
        };
        let mut i = 0usize;
        while i < args.len() {
            let arg_lower = args[i].to_ascii_lowercase();
            if opts.shape_name.is_empty() {
                opts.shape_name = args[i].to_string();
            } else if opts.file_path.is_empty() {
                opts.file_path = args[i].to_string();
            } else if arg_lower == "-brep" {
                opts.create_comp_of_tris = true;
                if i + 1 < args.len() {
                    if let Some(v) = stl_parse_on_off(args[i + 1]) {
                        opts.create_comp_of_tris = v;
                        i += 1;
                    }
                }
            } else if arg_lower == "-multi" {
                opts.multi = true;
                if i + 1 < args.len() {
                    if let Some(v) = stl_parse_on_off(args[i + 1]) {
                        opts.multi = v;
                        i += 1;
                    }
                }
            } else if arg_lower == "-mergeangle"
                || arg_lower == "-smoothangle"
                || arg_lower == "-nomergeangle"
                || arg_lower == "-nosmoothangle"
            {
                if arg_lower.starts_with("-no") {
                    opts.merge_angle = std::f64::consts::PI / 2.0;
                } else {
                    opts.merge_angle = std::f64::consts::PI / 4.0;
                    if i + 1 < args.len() {
                        if let Ok(angle_deg) = args[i + 1].parse::<f64>() {
                            if !(0.0..=90.0).contains(&angle_deg) {
                                return Err(
                                    "Syntax error: angle should be within [0,90] range".to_string()
                                );
                            }
                            i += 1;
                            opts.merge_angle = angle_deg * std::f64::consts::PI / 180.0;
                        }
                    }
                }
            } else {
                return Err(format!("Syntax error: unknown argument '{}'", args[i]));
            }
            i += 1;
        }
        if opts.file_path.is_empty() {
            return Err("Syntax error: not enough arguments".to_string());
        }
        Ok(opts)
    }

    /// Mirrors the `writestl` ASCII-mode selection: the optional third
    /// argument selects ASCII when it parses to 0, binary otherwise
    /// (binary/1 by default).
    pub fn writestl_is_ascii_mode(optional_mode_arg: Option<&str>) -> bool {
        match optional_mode_arg {
            Some(tok) => tok.trim().parse::<i32>().unwrap_or(0) == 0,
            None => false,
        }
    }

    /// Mirrors XSDRAWSTL::Factory(Draw_Interpretor&): registers the twenty
    /// STL/MeshVS commands under group "XSTEP-STL/VRML". The
    /// `static bool aIsActivated` guard is modelled as instance state.
    pub fn factory(&mut self, di: &mut StlDrawInterpretor) {
        if self.activated {
            return;
        }
        self.activated = true;

        let a_group = Self::GROUP;
        di.add(
            "writestl",
            "shape file [ascii/binary (0/1) : 1 by default] [InParallel (0/1) : 0 by default]",
            a_group,
        );
        di.add(
            "readstl",
            "readstl shape file [-brep] [-mergeAngle Angle] [-multi]\
             \n\t\t: Reads STL file and creates a new shape with specified name.\
             \n\t\t: When -brep is specified, creates a Compound of per-triangle Faces.\
             \n\t\t: Single triangulation-only Face is created otherwise (default).\
             \n\t\t: -mergeAngle specifies maximum angle in degrees between triangles to merge equal \
             nodes; disabled by default.\
             \n\t\t: -multi creates a face per solid in multi-domain files; ignored when -brep is set.",
            a_group,
        );
        di.add("meshfromstl", "creates MeshVS_Mesh from STL file", a_group);
        di.add("mesh3delem", "creates 3d element mesh to test", a_group);
        di.add("meshshadcolor", "change MeshVS_Mesh shading color", a_group);
        di.add("meshlinkcolor", "change MeshVS_Mesh line color", a_group);
        di.add(
            "meshmat",
            "change MeshVS_Mesh material and transparency",
            a_group,
        );
        di.add("meshshrcoef", "change MeshVS_Mesh shrink coeff", a_group);
        di.add(
            "meshclosed",
            "meshclosed meshname (0/1) \nChange MeshVS_Mesh drawing mode. 0 - not closed object, 1 \
             - closed object",
            a_group,
        );
        di.add("meshshow", "display MeshVS_Mesh object", a_group);
        di.add("meshhide", "erase MeshVS_Mesh object", a_group);
        di.add("meshhidesel", "hide selected entities", a_group);
        di.add("meshshowsel", "show only selected entities", a_group);
        di.add("meshshowall", "show all entities", a_group);
        di.add("meshcolors", "display color presentation", a_group);
        di.add("meshvectors", "display sample vectors", a_group);
        di.add("meshtext", "display text labels", a_group);
        di.add("meshdeform", "display deformed mesh", a_group);
        di.add("mesh_edge_width", "set width of edges", a_group);
        di.add(
            "meshinfo",
            "displays the number of nodes and triangles",
            a_group,
        );
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factory_registers_all_twenty_commands() {
        let mut di = StlDrawInterpretor::new();
        let mut plugin = Xsdrawstl::new();
        plugin.factory(&mut di);
        let names: Vec<&str> = di.commands().iter().map(|c| c.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "writestl",
                "readstl",
                "meshfromstl",
                "mesh3delem",
                "meshshadcolor",
                "meshlinkcolor",
                "meshmat",
                "meshshrcoef",
                "meshclosed",
                "meshshow",
                "meshhide",
                "meshhidesel",
                "meshshowsel",
                "meshshowall",
                "meshcolors",
                "meshvectors",
                "meshtext",
                "meshdeform",
                "mesh_edge_width",
                "meshinfo"
            ]
        );
        assert!(di.commands().iter().all(|c| c.group == "XSTEP-STL/VRML"));
        // Second Factory call is a no-op (static guard).
        plugin.factory(&mut di);
        assert_eq!(di.commands().len(), 20);
    }

    #[test]
    fn readstl_defaults() {
        let opts = Xsdrawstl::readstl_parse_args(&["s", "part.stl"]).unwrap();
        assert_eq!(opts.shape_name, "s");
        assert_eq!(opts.file_path, "part.stl");
        assert!(!opts.create_comp_of_tris);
        assert!(!opts.multi);
        assert!((opts.merge_angle - std::f64::consts::PI / 2.0).abs() < 1e-15);
    }

    #[test]
    fn readstl_brep_and_multi_flags() {
        let opts = Xsdrawstl::readstl_parse_args(&["s", "f.stl", "-brep", "-multi"]).unwrap();
        assert!(opts.create_comp_of_tris);
        assert!(opts.multi);
        // explicit off value is consumed
        let opts2 = Xsdrawstl::readstl_parse_args(&["s", "f.stl", "-brep", "0"]).unwrap();
        assert!(!opts2.create_comp_of_tris);
    }

    #[test]
    fn readstl_merge_angle_parsing() {
        // flag without value: PI/4 default
        let opts = Xsdrawstl::readstl_parse_args(&["s", "f.stl", "-mergeangle"]).unwrap();
        assert!((opts.merge_angle - std::f64::consts::PI / 4.0).abs() < 1e-15);
        // explicit 45 degrees converted to radians
        let opts45 = Xsdrawstl::readstl_parse_args(&["s", "f.stl", "-mergeAngle", "45"]).unwrap();
        assert!((opts45.merge_angle - 45.0 * std::f64::consts::PI / 180.0).abs() < 1e-15);
        // out of range is a syntax error
        let err = Xsdrawstl::readstl_parse_args(&["s", "f.stl", "-mergeangle", "91"]).unwrap_err();
        assert_eq!(err, "Syntax error: angle should be within [0,90] range");
        // -nomergeangle resets to PI/2
        let optsno = Xsdrawstl::readstl_parse_args(&["s", "f.stl", "-noMergeAngle"]).unwrap();
        assert!((optsno.merge_angle - std::f64::consts::PI / 2.0).abs() < 1e-15);
    }

    #[test]
    fn readstl_error_cases() {
        assert_eq!(
            Xsdrawstl::readstl_parse_args(&["only_shape"]).unwrap_err(),
            "Syntax error: not enough arguments"
        );
        assert_eq!(
            Xsdrawstl::readstl_parse_args(&["s", "f.stl", "-bogus"]).unwrap_err(),
            "Syntax error: unknown argument '-bogus'"
        );
    }

    #[test]
    fn writestl_ascii_mode_selection() {
        assert!(!Xsdrawstl::writestl_is_ascii_mode(None)); // binary by default
        assert!(Xsdrawstl::writestl_is_ascii_mode(Some("0")));
        assert!(!Xsdrawstl::writestl_is_ascii_mode(Some("1")));
    }
}
