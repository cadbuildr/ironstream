// FILE: xsdrawstep.rs
// occt: XSDRAWSTEP
//
// Faithful port of OCCT XSDRAWSTEP (Draw/TKXSDRAWSTEP/XSDRAWSTEP.cxx/.hxx),
// the plugin class registering the STEP data-exchange Draw commands:
//   stepwrite, testwritestep, stepread, testreadstep, steptrans,
//   countexpected, dumpassembly, stepfileunits, ReadStep, WriteStep
// all under group "DE: STEP".
//
// The Draw_Interpretor is modelled by a small local command table; the
// registration payload (command names, help strings, group, once-only
// activation guard) and the mode-argument parsing used by the commands
// (`stepwrite` model-type characters, `ReadStep`/`WriteStep` +/- mode
// flags) are real and tested.

/// Local model of one Draw_Interpretor command registration
/// (`Draw_Interpretor::Add(name, help, file, func, group)`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepDrawCommandEntry {
    pub name: String,
    pub help: String,
    pub group: String,
}

/// Local model of Draw_Interpretor: an ordered command registry.
#[derive(Debug, Default)]
pub struct StepDrawInterpretor {
    commands: Vec<StepDrawCommandEntry>,
}

impl StepDrawInterpretor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors Draw_Interpretor::Add.
    pub fn add(&mut self, name: &str, help: &str, group: &str) {
        self.commands.push(StepDrawCommandEntry {
            name: name.to_string(),
            help: help.to_string(),
            group: group.to_string(),
        });
    }

    pub fn commands(&self) -> &[StepDrawCommandEntry] {
        &self.commands
    }

    pub fn find(&self, name: &str) -> Option<&StepDrawCommandEntry> {
        self.commands.iter().find(|c| c.name == name)
    }
}

/// Local model of STEPControl_StepModelType, the write model types
/// selectable by the `stepwrite` / `WriteStep` mode argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepDrawModelType {
    AsIs,
    FacetedBrep,
    ShellBasedSurfaceModel,
    ManifoldSolidBrep,
    GeometricCurveSet,
}

/// XCAF transfer mode flags parsed from the `ReadStep` mode string
/// (`c`olor, `n`ame, `l`ayer, props (`v`), `m`eta, toggled by '+'/'-').
/// All reader modes default to true in STEPCAFControl_Reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepDrawCafModes {
    pub color: bool,
    pub name: bool,
    pub layer: bool,
    pub props: bool,
    pub meta: bool,
}

impl Default for StepDrawCafModes {
    fn default() -> Self {
        StepDrawCafModes {
            color: true,
            name: true,
            layer: true,
            props: true,
            meta: true,
        }
    }
}

/// Port of the XSDRAWSTEP plugin class.
#[derive(Debug, Default)]
pub struct Xsdrawstep {
    activated: bool,
}

impl Xsdrawstep {
    pub const GROUP: &'static str = "DE: STEP";

    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors the `switch (theArgVec[1][0])` of the static `stepwrite`
    /// command (and the identical mode switch inside `WriteStep`):
    /// a/0 AsIs, f/1 FacetedBrep, s/2 ShellBasedSurfaceModel,
    /// m/3 ManifoldSolidBrep, w/4 GeometricCurveSet; anything else is the
    /// "1st arg = mode, incorrect [give fsmw]" error.
    pub fn stepwrite_mode_from_char(mode_char: char) -> Option<StepDrawModelType> {
        match mode_char {
            'a' | '0' => Some(StepDrawModelType::AsIs),
            'f' | '1' => Some(StepDrawModelType::FacetedBrep),
            's' | '2' => Some(StepDrawModelType::ShellBasedSurfaceModel),
            'm' | '3' => Some(StepDrawModelType::ManifoldSolidBrep),
            'w' | '4' => Some(StepDrawModelType::GeometricCurveSet),
            _ => None,
        }
    }

    /// Mirrors the mode-string loop of the static `ReadStep` command:
    /// '-' and '+' set the toggle applied by the following letters,
    /// 'c' color, 'n' name, 'l' layer, 'v' props, 'm' meta; any other
    /// character is a syntax error (None).
    pub fn readstep_parse_modes(mode_str: &str) -> Option<StepDrawCafModes> {
        let mut modes = StepDrawCafModes::default();
        let mut toggle = true;
        for c in mode_str.chars() {
            match c {
                '-' => toggle = false,
                '+' => toggle = true,
                'c' => modes.color = toggle,
                'n' => modes.name = toggle,
                'l' => modes.layer = toggle,
                'v' => modes.props = toggle,
                'm' => modes.meta = toggle,
                _ => return None,
            }
        }
        Some(modes)
    }

    /// Mirrors XSDRAWSTEP::Factory(Draw_Interpretor&): registers the ten
    /// STEP commands under group "DE: STEP". The `static bool aIsActivated`
    /// guard is modelled as instance state: a second call is a no-op.
    pub fn factory(&mut self, di: &mut StepDrawInterpretor) {
        if self.activated {
            return;
        }
        self.activated = true;

        let a_group = Self::GROUP;
        di.add("stepwrite", "stepwrite mode[0-4 afsmw] shape", a_group);
        di.add(
            "testwritestep",
            "testwritestep [file_1.stp ... file_n.stp] shape [-stream]",
            a_group,
        );
        di.add(
            "stepread",
            "stepread  [file] [f or r (type of model full or reduced)]",
            a_group,
        );
        di.add(
            "testreadstep",
            "testreadstep [file_1 ... file_n] shape [-stream]",
            a_group,
        );
        di.add("steptrans", "steptrans shape stepax1 stepax2", a_group);
        di.add("countexpected", "TEST", a_group);
        di.add("dumpassembly", "TEST", a_group);
        di.add("stepfileunits", "stepfileunits name_file", a_group);
        di.add(
            "ReadStep",
            "Doc filename [mode] [-stream]\
             \n\t\t: Read STEP file to a document.\
             \n\t\t:  -stream read using istream reading interface (testing)",
            a_group,
        );
        di.add(
            "WriteStep",
            "Doc filename [mode=a [multifile_prefix] [label]] [-stream]\
             \n\t\t: Write DECAF document to STEP file\
             \n\t\t:   mode can be: a or 0 : AsIs (default)\
             \n\t\t:                f or 1 : FacettedBRep        s or 2 : ShellBasedSurfaceModel\
             \n\t\t:                m or 3 : ManifoldSolidBrep   w or 4 : GeometricCurveSet/WireFrame\
             \n\t\t:   multifile_prefix: triggers writing assembly components as separate files,\
             \n\t\t:                     and defines common prefix for their names\
             \n\t\t:   label  tag of the sub-assembly label to save only that sub-assembly\
             \n\t\t:  -stream read using ostream writing interface (testing)",
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
    fn factory_registers_all_ten_step_commands() {
        let mut di = StepDrawInterpretor::new();
        let mut plugin = Xsdrawstep::new();
        plugin.factory(&mut di);
        let names: Vec<&str> = di.commands().iter().map(|c| c.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "stepwrite",
                "testwritestep",
                "stepread",
                "testreadstep",
                "steptrans",
                "countexpected",
                "dumpassembly",
                "stepfileunits",
                "ReadStep",
                "WriteStep"
            ]
        );
        assert!(di.commands().iter().all(|c| c.group == "DE: STEP"));
        assert_eq!(
            di.find("stepwrite").unwrap().help,
            "stepwrite mode[0-4 afsmw] shape"
        );
        assert_eq!(
            di.find("stepread").unwrap().help,
            "stepread  [file] [f or r (type of model full or reduced)]"
        );
    }

    #[test]
    fn factory_is_once_only() {
        let mut di = StepDrawInterpretor::new();
        let mut plugin = Xsdrawstep::new();
        plugin.factory(&mut di);
        assert!(plugin.is_activated());
        plugin.factory(&mut di);
        assert_eq!(di.commands().len(), 10);
    }

    #[test]
    fn stepwrite_mode_characters() {
        assert_eq!(
            Xsdrawstep::stepwrite_mode_from_char('a'),
            Some(StepDrawModelType::AsIs)
        );
        assert_eq!(
            Xsdrawstep::stepwrite_mode_from_char('0'),
            Some(StepDrawModelType::AsIs)
        );
        assert_eq!(
            Xsdrawstep::stepwrite_mode_from_char('f'),
            Some(StepDrawModelType::FacetedBrep)
        );
        assert_eq!(
            Xsdrawstep::stepwrite_mode_from_char('2'),
            Some(StepDrawModelType::ShellBasedSurfaceModel)
        );
        assert_eq!(
            Xsdrawstep::stepwrite_mode_from_char('m'),
            Some(StepDrawModelType::ManifoldSolidBrep)
        );
        assert_eq!(
            Xsdrawstep::stepwrite_mode_from_char('4'),
            Some(StepDrawModelType::GeometricCurveSet)
        );
        assert_eq!(Xsdrawstep::stepwrite_mode_from_char('x'), None);
    }

    #[test]
    fn readstep_mode_string_parsing() {
        let all_on = Xsdrawstep::readstep_parse_modes("").unwrap();
        assert_eq!(all_on, StepDrawCafModes::default());
        assert!(all_on.color && all_on.meta);

        let no_color = Xsdrawstep::readstep_parse_modes("-c").unwrap();
        assert!(!no_color.color);
        assert!(no_color.name && no_color.layer && no_color.props && no_color.meta);

        // toggle applies to following letters until switched back by '+'
        let mixed = Xsdrawstep::readstep_parse_modes("-cn+l").unwrap();
        assert!(!mixed.color && !mixed.name && mixed.layer);

        assert!(Xsdrawstep::readstep_parse_modes("z").is_none());
    }

    #[test]
    fn writestep_help_mentions_all_modes() {
        let mut di = StepDrawInterpretor::new();
        let mut plugin = Xsdrawstep::new();
        plugin.factory(&mut di);
        let help = &di.find("WriteStep").unwrap().help;
        assert!(help.contains("a or 0 : AsIs (default)"));
        assert!(help.contains("GeometricCurveSet/WireFrame"));
        assert!(help.contains("multifile_prefix"));
    }
}
