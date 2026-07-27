// FILE: tfunction_driver.rs
// occt: TFunction_Driver, TFunction_Logbook, TFunction_DriverTable
// occt-ref: TFunction_IFunction

/// Execution result of a function driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFunctionExecStatus {
    NotExecuted,
    ExecutionTooOld,
    ExecutionUpToDate,
}

impl Default for TFunctionExecStatus {
    fn default() -> Self { Self::NotExecuted }
}

/// Records touched/impacted labels during function execution.
/// occt: TFunction_Logbook
#[derive(Clone, Debug, Default)]
pub struct TFunctionLogbook {
    pub touched: Vec<u32>,
    pub impacted: Vec<u32>,
    pub valid: Vec<u32>,
    pub done: bool,
}

impl TFunctionLogbook {
    pub fn new() -> Self { Self::default() }

    pub fn set_touched(&mut self, id: u32) {
        if !self.touched.contains(&id) { self.touched.push(id); }
    }

    pub fn set_impacted(&mut self, id: u32) {
        if !self.impacted.contains(&id) { self.impacted.push(id); }
    }

    pub fn is_touched(&self, id: u32) -> bool { self.touched.contains(&id) }
    pub fn is_impacted(&self, id: u32) -> bool { self.impacted.contains(&id) }

    pub fn add_valid(&mut self, id: u32) {
        if !self.valid.contains(&id) { self.valid.push(id); }
    }

    pub fn is_valid(&self, id: u32) -> bool { self.valid.contains(&id) }

    pub fn set_done(&mut self, v: bool) { self.done = v; }
    pub fn is_done(&self) -> bool { self.done }

    pub fn clear(&mut self) {
        self.touched.clear();
        self.impacted.clear();
        self.valid.clear();
        self.done = false;
    }
}

/// Abstract function driver (stub): defines inputs/outputs and execution logic.
/// occt: TFunction_Driver
#[derive(Clone, Debug)]
pub struct TFunctionDriver {
    pub driver_id: u32,
    pub label_id: u32,
    pub arguments: Vec<u32>,   // input label ids
    pub results: Vec<u32>,     // output label ids
    pub exec_status: TFunctionExecStatus,
}

impl TFunctionDriver {
    pub fn new(driver_id: u32) -> Self {
        Self {
            driver_id,
            label_id: 0,
            arguments: Vec::new(),
            results: Vec::new(),
            exec_status: TFunctionExecStatus::NotExecuted,
        }
    }

    pub fn set_label(&mut self, id: u32) { self.label_id = id; }

    pub fn add_argument(&mut self, id: u32) {
        if !self.arguments.contains(&id) { self.arguments.push(id); }
    }

    pub fn add_result(&mut self, id: u32) {
        if !self.results.contains(&id) { self.results.push(id); }
    }

    /// Stub execute: marks all results as valid, sets done.
    pub fn execute(&mut self, log: &mut TFunctionLogbook) -> i32 {
        for &r in &self.results { log.add_valid(r); }
        log.set_done(true);
        self.exec_status = TFunctionExecStatus::ExecutionUpToDate;
        0
    }

    pub fn must_execute(&self, log: &TFunctionLogbook) -> bool {
        self.exec_status == TFunctionExecStatus::NotExecuted
            || self.arguments.iter().any(|a| log.is_touched(*a))
    }

    pub fn nb_arguments(&self) -> usize { self.arguments.len() }
    pub fn nb_results(&self) -> usize { self.results.len() }
}

/// Registry mapping driver_id → driver name/type.
/// occt: TFunction_DriverTable
#[derive(Clone, Debug, Default)]
pub struct TFunctionDriverTable {
    pub entries: Vec<(u32, String)>,
}

impl TFunctionDriverTable {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, driver_id: u32, name: &str) {
        self.entries.push((driver_id, name.to_string()));
    }

    pub fn find(&self, driver_id: u32) -> Option<&str> {
        self.entries.iter().find(|(id,_)| *id == driver_id).map(|(_,n)| n.as_str())
    }

    pub fn has_driver(&self, driver_id: u32) -> bool {
        self.entries.iter().any(|(id,_)| *id == driver_id)
    }

    pub fn nb_drivers(&self) -> usize { self.entries.len() }
}

/// Thin wrapper around a label that is function-enabled.
// occt-ref: TFunction_IFunction
#[derive(Clone, Debug)]
pub struct TFunctionIFunction {
    pub label_id: u32,
    pub driver_id: u32,
    pub status: TFunctionExecStatus,
}

impl TFunctionIFunction {
    pub fn new(label_id: u32, driver_id: u32) -> Self {
        Self { label_id, driver_id, status: TFunctionExecStatus::NotExecuted }
    }

    pub fn invalidate(&mut self) { self.status = TFunctionExecStatus::ExecutionTooOld; }
    pub fn mark_up_to_date(&mut self) { self.status = TFunctionExecStatus::ExecutionUpToDate; }
    pub fn is_up_to_date(&self) -> bool { self.status == TFunctionExecStatus::ExecutionUpToDate }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logbook_touched_impacted() {
        let mut log = TFunctionLogbook::new();
        log.set_touched(10);
        log.set_impacted(20);
        assert!(log.is_touched(10));
        assert!(!log.is_touched(20));
        assert!(log.is_impacted(20));
        log.set_touched(10); // dup
        assert_eq!(log.touched.len(), 1);
    }

    #[test]
    fn driver_execute() {
        let mut d = TFunctionDriver::new(1);
        d.add_result(100);
        d.add_result(101);
        let mut log = TFunctionLogbook::new();
        let rc = d.execute(&mut log);
        assert_eq!(rc, 0);
        assert!(log.is_done());
        assert!(log.is_valid(100));
        assert_eq!(d.exec_status, TFunctionExecStatus::ExecutionUpToDate);
    }

    #[test]
    fn driver_must_execute() {
        let mut d = TFunctionDriver::new(2);
        d.add_argument(5);
        let mut log = TFunctionLogbook::new();
        assert!(d.must_execute(&log)); // NotExecuted
        d.exec_status = TFunctionExecStatus::ExecutionUpToDate;
        assert!(!d.must_execute(&log));
        log.set_touched(5);
        assert!(d.must_execute(&log)); // argument touched
    }

    #[test]
    fn driver_table_find() {
        let mut t = TFunctionDriverTable::new();
        t.add(1, "MyDriver");
        t.add(2, "OtherDriver");
        assert_eq!(t.find(1), Some("MyDriver"));
        assert!(t.has_driver(2));
        assert!(!t.has_driver(99));
        assert_eq!(t.nb_drivers(), 2);
    }

    #[test]
    fn ifunction_lifecycle() {
        let mut f = TFunctionIFunction::new(10, 1);
        assert!(!f.is_up_to_date());
        f.mark_up_to_date();
        assert!(f.is_up_to_date());
        f.invalidate();
        assert!(!f.is_up_to_date());
    }
}
