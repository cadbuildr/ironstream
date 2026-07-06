// FILE: step_data_step_reader_data.rs
// occt: StepData_StepReaderData

// Local helper mirroring Interface_ParamType (external plumbing, subset)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceParamType {
    Integer,
    Real,
    Ident,
    Text,
    Enum,
    Void,
    Sub,
    Misc,
}

// Local helper mirroring Interface_FileParameter (external plumbing)
#[derive(Clone)]
pub struct InterfaceFileParameter {
    ptype: InterfaceParamType,
    value: String,
}

impl InterfaceFileParameter {
    pub fn new(ptype: InterfaceParamType, value: &str) -> Self {
        InterfaceFileParameter {
            ptype,
            value: value.to_string(),
        }
    }
    pub fn param_type(&self) -> InterfaceParamType {
        self.ptype
    }
    pub fn c_value(&self) -> &str {
        &self.value
    }
}

// Local helper mirroring Interface_Check (external plumbing)
#[derive(Default)]
pub struct InterfaceCheck {
    fails: Vec<String>,
}

impl InterfaceCheck {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_fail(&mut self, mess: &str) {
        self.fails.push(mess.to_string());
    }
    pub fn nb_fails(&self) -> usize {
        self.fails.len()
    }
    pub fn has_failed(&self) -> bool {
        !self.fails.is_empty()
    }
    pub fn fail(&self, num: usize) -> Option<&str> {
        self.fails.get(num.checked_sub(1)?).map(|s| s.as_str())
    }
}

struct Record {
    ident: i32, // >0: entity #id; -1: SCOPE; -2: ENDSCOPE; <0/0: sub-list
    rectype: String,
    params: Vec<InterfaceFileParameter>,
}

// Data container for a STEP file being read: a set of records
// (1-based), each with an identifier, a type and parameters.
// Mirrors the record/parameter access and Read* semantics of
// StepData_StepReaderData for a representative subset of its API.
pub struct StepDataStepReaderData {
    nbhead: usize, // count of header records, skipped by find_next_record
    records: Vec<Record>,
}

impl StepDataStepReaderData {
    // Creates a StepReaderData; nbheader records belong to the Header
    pub fn new(nbheader: usize) -> Self {
        StepDataStepReaderData {
            nbhead: nbheader,
            records: Vec::new(),
        }
    }

    // SetRecord: appends a record (records are contiguous, 1-based).
    // ident > 0 for a true entity (#id), -1 SCOPE, -2 ENDSCOPE,
    // other non-positive values for sub-lists
    pub fn set_record(&mut self, ident: i32, rectype: &str) -> usize {
        self.records.push(Record {
            ident,
            rectype: rectype.to_string(),
            params: Vec::new(),
        });
        self.records.len()
    }

    // AddStepParam: adds a parameter to record num
    pub fn add_step_param(&mut self, num: usize, value: &str, ptype: InterfaceParamType) {
        if let Some(r) = self.rec_mut(num) {
            r.params.push(InterfaceFileParameter::new(ptype, value));
        }
    }

    fn rec(&self, num: usize) -> Option<&Record> {
        self.records.get(num.checked_sub(1)?)
    }

    fn rec_mut(&mut self, num: usize) -> Option<&mut Record> {
        self.records.get_mut(num.checked_sub(1)?)
    }

    // RecordType (CType)
    pub fn record_type(&self, num: usize) -> &str {
        self.rec(num).map(|r| r.rectype.as_str()).unwrap_or("")
    }

    // RecordIdent: identifier of a record, 0 if none
    pub fn record_ident(&self, num: usize) -> i32 {
        self.rec(num).map(|r| r.ident).unwrap_or(0)
    }

    pub fn nb_records(&self) -> usize {
        self.records.len()
    }

    // NbEntities: only records with a positive ident are true entities
    pub fn nb_entities(&self) -> usize {
        self.records.iter().filter(|r| r.ident > 0).count()
    }

    // NbParams of a record
    pub fn nb_params(&self, num: usize) -> usize {
        self.rec(num).map(|r| r.params.len()).unwrap_or(0)
    }

    // Param (1-based within record)
    pub fn param(&self, num: usize, nump: usize) -> Option<&InterfaceFileParameter> {
        self.rec(num)?.params.get(nump.checked_sub(1)?)
    }

    // FindNextRecord: next record after num which defines an entity,
    // or 0 if finished. Passes the header (first nbhead records) and
    // skips SCOPE/ENDSCOPE and sub-lists (non-positive idents)
    pub fn find_next_record(&self, num: usize) -> usize {
        let mut num1 = if num == 0 { self.nbhead + 1 } else { num + 1 };
        let max = self.nb_records();
        while num1 <= max {
            if self.record_ident(num1) > 0 {
                return num1;
            }
            num1 += 1;
        }
        0
    }

    // CheckNbParams: true if record num has exactly nbreq parameters,
    // else false and a fail is logged
    pub fn check_nb_params(&self, num: usize, nbreq: usize, ach: &mut InterfaceCheck) -> bool {
        if self.nb_params(num) == nbreq {
            return true;
        }
        ach.add_fail(&format!("Count of Parameters is not {}", nbreq));
        false
    }

    // ReadInteger: Integer param is read; a Real param is rounded with
    // a fail; other types / absent param fail
    pub fn read_integer(
        &self,
        num: usize,
        nump: usize,
        mess: &str,
        ach: &mut InterfaceCheck,
        val: &mut i32,
    ) -> bool {
        let mut errmess: Option<String> = None;
        if nump > 0 && nump <= self.nb_params(num) {
            let fp = self.param(num, nump).unwrap();
            match fp.param_type() {
                InterfaceParamType::Integer => {
                    *val = fp.c_value().parse::<i32>().unwrap_or(0);
                }
                InterfaceParamType::Real => {
                    *val = fp.c_value().parse::<f64>().unwrap_or(0.0).round() as i32;
                    errmess = Some(format!("Parameter n0.{} ({}) was rounded", nump, mess));
                }
                _ => {
                    errmess = Some(format!("Parameter n0.{} ({}) not an Integer", nump, mess));
                }
            }
        } else {
            errmess = Some(format!("Parameter n0.{} ({}) absent", nump, mess));
        }
        match errmess {
            None => true,
            Some(e) => {
                ach.add_fail(&e);
                false
            }
        }
    }

    // ReadReal: accepts Real or Integer params
    pub fn read_real(
        &self,
        num: usize,
        nump: usize,
        mess: &str,
        ach: &mut InterfaceCheck,
        val: &mut f64,
    ) -> bool {
        let mut errmess: Option<String> = None;
        if nump > 0 && nump <= self.nb_params(num) {
            let fp = self.param(num, nump).unwrap();
            match fp.param_type() {
                InterfaceParamType::Real | InterfaceParamType::Integer => {
                    *val = fp.c_value().parse::<f64>().unwrap_or(0.0);
                }
                _ => {
                    errmess = Some(format!("Parameter n0.{} ({}) not a Real", nump, mess));
                }
            }
        } else {
            errmess = Some(format!("Parameter n0.{} ({}) absent", nump, mess));
        }
        match errmess {
            None => true,
            Some(e) => {
                ach.add_fail(&e);
                false
            }
        }
    }

    // ReadBoolean: expects an Enum param ".T." or ".F.";
    // on error flag is set to true (OCCT behavior) and false returned
    pub fn read_boolean(
        &self,
        num: usize,
        nump: usize,
        mess: &str,
        ach: &mut InterfaceCheck,
        flag: &mut bool,
    ) -> bool {
        *flag = true;
        let mut errmess: Option<String> = None;
        if nump > 0 && nump <= self.nb_params(num) {
            let fp = self.param(num, nump).unwrap();
            if fp.param_type() == InterfaceParamType::Enum {
                match fp.c_value() {
                    ".T." => *flag = true,
                    ".F." => *flag = false,
                    _ => {
                        errmess = Some(format!(
                            "Parameter n0.{} ({}) : Incorrect Boolean Value. It was set to true",
                            nump, mess
                        ));
                    }
                }
            } else {
                errmess = Some(format!(
                    "Parameter n0.{} ({}) not a Boolean. It was set to true",
                    nump, mess
                ));
            }
        } else {
            errmess = Some(format!(
                "Parameter n0.{} ({}) absent.It was set to true",
                nump, mess
            ));
        }
        match errmess {
            None => true,
            Some(e) => {
                ach.add_fail(&e);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 2 header records + entities #1 CARTESIAN_POINT, sub-list, #2 DIRECTION
    fn sample() -> StepDataStepReaderData {
        let mut d = StepDataStepReaderData::new(2);
        d.set_record(0, "HEADER_A"); // header rec 1
        d.set_record(0, "HEADER_B"); // header rec 2
        let n = d.set_record(1, "CARTESIAN_POINT"); // rec 3, #1
        d.add_step_param(n, "'P1'", InterfaceParamType::Text);
        d.add_step_param(n, "4.5", InterfaceParamType::Real);
        d.add_step_param(n, "7", InterfaceParamType::Integer);
        d.add_step_param(n, ".T.", InterfaceParamType::Enum);
        d.set_record(-3, "$SUB"); // rec 4, sub-list
        d.set_record(2, "DIRECTION"); // rec 5, #2
        d
    }

    #[test]
    fn test_records_and_entities() {
        let d = sample();
        assert_eq!(d.nb_records(), 5);
        assert_eq!(d.nb_entities(), 2);
        assert_eq!(d.record_type(3), "CARTESIAN_POINT");
        assert_eq!(d.record_ident(3), 1);
        assert_eq!(d.record_ident(4), -3);
        assert_eq!(d.nb_params(3), 4);
        assert_eq!(d.nb_params(5), 0);
    }

    #[test]
    fn test_find_next_record_skips_header_and_sublists() {
        let d = sample();
        // from scratch: skips the 2 header records, lands on rec 3
        assert_eq!(d.find_next_record(0), 3);
        // after rec 3: skips the sub-list rec 4, lands on rec 5
        assert_eq!(d.find_next_record(3), 5);
        // finished
        assert_eq!(d.find_next_record(5), 0);
    }

    #[test]
    fn test_read_integer_and_real() {
        let d = sample();
        let mut ach = InterfaceCheck::new();
        let mut ival = 0;
        assert!(d.read_integer(3, 3, "count", &mut ach, &mut ival));
        assert_eq!(ival, 7);
        // a Real param is rounded but reported as a fail
        assert!(!d.read_integer(3, 2, "count", &mut ach, &mut ival));
        assert_eq!(ival, 5); // 4.5 rounds away from zero to 5
        assert_eq!(ach.nb_fails(), 1);
        // a Text param is not an Integer
        assert!(!d.read_integer(3, 1, "count", &mut ach, &mut ival));
        assert_eq!(ach.nb_fails(), 2);

        let mut rval = 0.0;
        assert!(d.read_real(3, 2, "coord", &mut ach, &mut rval));
        assert!((rval - 4.5).abs() < 1e-12);
        // an Integer param is accepted as Real
        assert!(d.read_real(3, 3, "coord", &mut ach, &mut rval));
        assert!((rval - 7.0).abs() < 1e-12);
        // absent parameter fails
        assert!(!d.read_real(3, 9, "coord", &mut ach, &mut rval));
        assert!(ach.has_failed());
    }

    #[test]
    fn test_read_boolean_and_check_nb_params() {
        let d = sample();
        let mut ach = InterfaceCheck::new();
        let mut flag = false;
        assert!(d.read_boolean(3, 4, "sense", &mut ach, &mut flag));
        assert!(flag);
        // Text param is not a Boolean: fails, flag forced to true
        assert!(!d.read_boolean(3, 1, "sense", &mut ach, &mut flag));
        assert!(flag);
        assert_eq!(ach.nb_fails(), 1);

        assert!(d.check_nb_params(3, 4, &mut ach));
        assert!(!d.check_nb_params(3, 2, &mut ach));
        assert_eq!(ach.nb_fails(), 2);
        assert_eq!(ach.fail(2), Some("Count of Parameters is not 2"));
    }
}
