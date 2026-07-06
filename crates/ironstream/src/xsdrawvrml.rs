// FILE: xsdrawvrml.rs
// occt: XSDRAWVRML
//
// Faithful port of OCCT XSDRAWVRML (Draw/TKXSDRAWVRML/XSDRAWVRML.cxx/.hxx),
// the plugin class registering the VRML Draw commands under group
// "XDE translation commands": ReadVrml, WriteVrml, loadvrml, writevrml.
//
// The Draw_Interpretor is modelled by a small local command registry; the
// registration payload (command names, help strings, group, once-only
// guard), the coordinate-system parser (`parseCoordinateSystem`), the
// `writevrml` version/type clamping and representation selection, and the
// VrmlData status -> error-string mapping of `loadvrml` are real and
// tested.

/// Local model of RWMesh_CoordinateSystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlDrawCoordinateSystem {
    Zup,
    Yup,
}

/// Local model of VrmlAPI_RepresentationOfShape used by writevrml.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlDrawRepresentation {
    Shaded,
    WireFrame,
    Both,
}

/// Local model of VrmlData_ErrorStatus (as matched by loadvrml).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlDrawSceneStatus {
    StatusOk,
    EmptyData,
    UnrecoverableError,
    GeneralError,
    EndOfFile,
    NotVrmlFile,
    CannotOpenFile,
    VrmlFormatError,
    NumericInputError,
    IrrelevantNumber,
    BooleanInputError,
    StringInputError,
    NodeNameUnknown,
    NonPositiveSize,
    ReadUnknownNode,
    NonSupportedFeature,
    OutputStreamUndefined,
    NotImplemented,
}

/// Local model of one registered Draw command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrmlDrawCommandEntry {
    pub name: String,
    pub help: String,
    pub group: String,
}

/// Local model of Draw_Interpretor for XSDRAWVRML registration.
#[derive(Debug, Default)]
pub struct VrmlDrawInterpretor {
    commands: Vec<VrmlDrawCommandEntry>,
}

impl VrmlDrawInterpretor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: &str, help: &str, group: &str) {
        self.commands.push(VrmlDrawCommandEntry {
            name: name.to_string(),
            help: help.to_string(),
            group: group.to_string(),
        });
    }

    pub fn commands(&self) -> &[VrmlDrawCommandEntry] {
        &self.commands
    }

    pub fn find(&self, name: &str) -> Option<&VrmlDrawCommandEntry> {
        self.commands.iter().find(|c| c.name == name)
    }
}

/// Port of the XSDRAWVRML plugin class.
#[derive(Debug, Default)]
pub struct Xsdrawvrml {
    activated: bool,
}

impl Xsdrawvrml {
    pub const GROUP: &'static str = "XDE translation commands";

    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors the static `parseCoordinateSystem` helper: case-insensitive
    /// "zup" / "yup"; anything else fails.
    pub fn parse_coordinate_system(the_arg: &str) -> Option<VrmlDrawCoordinateSystem> {
        let lower = the_arg.to_ascii_lowercase();
        match lower.as_str() {
            "zup" => Some(VrmlDrawCoordinateSystem::Zup),
            "yup" => Some(VrmlDrawCoordinateSystem::Yup),
            _ => None,
        }
    }

    /// Mirrors the `writevrml` parameter bounding:
    /// version clamped to [1,2] (default 2), type clamped to [0,2]
    /// (default 1) then mapped 0->Shaded, 1->WireFrame, 2->Both.
    pub fn writevrml_bound_parameters(
        version_arg: Option<i32>,
        type_arg: Option<i32>,
    ) -> (i32, VrmlDrawRepresentation) {
        let mut a_version = version_arg.unwrap_or(2);
        let mut a_type = type_arg.unwrap_or(1);
        a_version = a_version.max(1).min(2);
        a_type = a_type.max(0).min(2);
        let repr = match a_type {
            0 => VrmlDrawRepresentation::Shaded,
            1 => VrmlDrawRepresentation::WireFrame,
            _ => VrmlDrawRepresentation::Both,
        };
        (a_version, repr)
    }

    /// Mirrors the status switch in `loadvrml`: returns the error string
    /// printed as " ++ VRML Error: <str> in line N", or None for
    /// VrmlData_StatusOK (shape is stored instead).
    pub fn loadvrml_status_string(status: VrmlDrawSceneStatus) -> Option<&'static str> {
        match status {
            VrmlDrawSceneStatus::StatusOk => None,
            VrmlDrawSceneStatus::EmptyData => Some("EmptyData"),
            VrmlDrawSceneStatus::UnrecoverableError => Some("UnrecoverableError"),
            VrmlDrawSceneStatus::GeneralError => Some("GeneralError"),
            VrmlDrawSceneStatus::EndOfFile => Some("EndOfFile"),
            VrmlDrawSceneStatus::NotVrmlFile => Some("NotVrmlFile"),
            VrmlDrawSceneStatus::CannotOpenFile => Some("CannotOpenFile"),
            VrmlDrawSceneStatus::VrmlFormatError => Some("VrmlFormatError"),
            VrmlDrawSceneStatus::NumericInputError => Some("NumericInputError"),
            VrmlDrawSceneStatus::IrrelevantNumber => Some("IrrelevantNumber"),
            VrmlDrawSceneStatus::BooleanInputError => Some("BooleanInputError"),
            VrmlDrawSceneStatus::StringInputError => Some("StringInputError"),
            VrmlDrawSceneStatus::NodeNameUnknown => Some("NodeNameUnknown"),
            VrmlDrawSceneStatus::NonPositiveSize => Some("NonPositiveSize"),
            VrmlDrawSceneStatus::ReadUnknownNode => Some("ReadUnknownNode"),
            VrmlDrawSceneStatus::NonSupportedFeature => Some("NonSupportedFeature"),
            VrmlDrawSceneStatus::OutputStreamUndefined => Some("OutputStreamUndefined"),
            VrmlDrawSceneStatus::NotImplemented => Some("NotImplemented"),
        }
    }

    /// Defaults of the `ReadVrml` command: file coordinate system Yup,
    /// system coordinate system Zup (before any option is parsed).
    pub fn readvrml_default_coordinate_systems(
    ) -> (VrmlDrawCoordinateSystem, VrmlDrawCoordinateSystem) {
        (
            VrmlDrawCoordinateSystem::Yup,
            VrmlDrawCoordinateSystem::Zup,
        )
    }

    /// Mirrors XSDRAWVRML::Factory(Draw_Interpretor&): registers the four
    /// VRML commands under "XDE translation commands". The
    /// `static bool anInitActor` guard is modelled as instance state.
    pub fn factory(&mut self, di: &mut VrmlDrawInterpretor) {
        if self.activated {
            return;
        }
        self.activated = true;

        let a_group = Self::GROUP;
        di.add(
            "ReadVrml",
            "ReadVrml docName filePath [-fileCoordSys {Zup|Yup}] [-fileUnit Unit]\
             \n\t\t:                   [-systemCoordSys {Zup|Yup}] [-noCreateDoc] [-fillIncomplete \
             {ON|OFF}]\
             \n\t\t: Read Vrml file into XDE document.\
             \n\t\t:   -fileCoordSys   coordinate system defined by Vrml file; Yup when not specified.\
             \n\t\t:   -fileUnit       length unit of Vrml file content.\
             \n\t\t:   -systemCoordSys result coordinate system; Zup when not specified.\
             \n\t\t:   -noCreateDoc    read into existing XDE document.\
             \n\t\t:   -fillIncomplete fill the document with partially retrieved data even if reader has \
             failed with \
             error; true when not specified",
            a_group,
        );
        di.add(
            "WriteVrml",
            "WriteVrml Doc filename [version VRML#1.0/VRML#2.0 (1/2): 2 by default] \
             [representation shaded/wireframe/both (0/1/2): 0 by default]",
            a_group,
        );
        di.add("loadvrml", "shape file", a_group);
        di.add(
            "writevrml",
            "shape file [version VRML#1.0/VRML#2.0 (1/2): 2 by default] [representation \
             shaded/wireframe/both (0/1/2): 1 by default]",
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
    fn factory_registers_four_vrml_commands() {
        let mut di = VrmlDrawInterpretor::new();
        let mut plugin = Xsdrawvrml::new();
        plugin.factory(&mut di);
        let names: Vec<&str> = di.commands().iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["ReadVrml", "WriteVrml", "loadvrml", "writevrml"]);
        assert!(di
            .commands()
            .iter()
            .all(|c| c.group == "XDE translation commands"));
        assert_eq!(di.find("loadvrml").unwrap().help, "shape file");
        // once-only guard
        plugin.factory(&mut di);
        assert_eq!(di.commands().len(), 4);
    }

    #[test]
    fn coordinate_system_parsing() {
        assert_eq!(
            Xsdrawvrml::parse_coordinate_system("Zup"),
            Some(VrmlDrawCoordinateSystem::Zup)
        );
        assert_eq!(
            Xsdrawvrml::parse_coordinate_system("YUP"),
            Some(VrmlDrawCoordinateSystem::Yup)
        );
        assert_eq!(Xsdrawvrml::parse_coordinate_system("xup"), None);
        let (file_cs, sys_cs) = Xsdrawvrml::readvrml_default_coordinate_systems();
        assert_eq!(file_cs, VrmlDrawCoordinateSystem::Yup);
        assert_eq!(sys_cs, VrmlDrawCoordinateSystem::Zup);
    }

    #[test]
    fn writevrml_bounds_version_and_type() {
        // defaults: version 2, wireframe
        assert_eq!(
            Xsdrawvrml::writevrml_bound_parameters(None, None),
            (2, VrmlDrawRepresentation::WireFrame)
        );
        // clamping
        assert_eq!(
            Xsdrawvrml::writevrml_bound_parameters(Some(0), Some(-3)),
            (1, VrmlDrawRepresentation::Shaded)
        );
        assert_eq!(
            Xsdrawvrml::writevrml_bound_parameters(Some(7), Some(9)),
            (2, VrmlDrawRepresentation::Both)
        );
        assert_eq!(
            Xsdrawvrml::writevrml_bound_parameters(Some(1), Some(1)),
            (1, VrmlDrawRepresentation::WireFrame)
        );
    }

    #[test]
    fn loadvrml_status_strings_match_cxx_switch() {
        assert_eq!(
            Xsdrawvrml::loadvrml_status_string(VrmlDrawSceneStatus::StatusOk),
            None
        );
        assert_eq!(
            Xsdrawvrml::loadvrml_status_string(VrmlDrawSceneStatus::EmptyData),
            Some("EmptyData")
        );
        assert_eq!(
            Xsdrawvrml::loadvrml_status_string(VrmlDrawSceneStatus::VrmlFormatError),
            Some("VrmlFormatError")
        );
        assert_eq!(
            Xsdrawvrml::loadvrml_status_string(VrmlDrawSceneStatus::NonSupportedFeature),
            Some("NonSupportedFeature")
        );
        assert_eq!(
            Xsdrawvrml::loadvrml_status_string(VrmlDrawSceneStatus::NotImplemented),
            Some("NotImplemented")
        );
    }
}
