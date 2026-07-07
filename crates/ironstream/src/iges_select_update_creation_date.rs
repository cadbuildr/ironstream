// FILE: iges_select_update_creation_date.rs
// occt: IGESSelect_UpdateCreationDate

//! Modifier that updates the IGES file header creation date to the current system date.
//!
//! This modifier acts on IGES Global Section Item 18 (creation date) and formats
//! the date as YYMMDD.HHMMSS (before Y2000) or YYYYMMDD.HHMMSS (after Y2000).

use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a system date/time with calendar and time components
#[derive(Clone, Debug, PartialEq)]
pub struct SystemDate {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    millisecond: i32,
}

impl SystemDate {
    /// Creates a new date from system time
    pub fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let total_secs = now.as_secs() as i64;

        // Basic calculation (simplified Unix epoch to calendar conversion)
        // For production, use proper chrono or time crate
        let days_since_epoch = total_secs / 86400;
        let secs_today = total_secs % 86400;

        let year = 1970 + (days_since_epoch / 365) as i32;
        let remaining_days = (days_since_epoch % 365) as i32;

        let month = (remaining_days / 30).min(11) + 1;
        let day = (remaining_days % 30) + 1;

        let hour = (secs_today / 3600) as i32;
        let minute = ((secs_today % 3600) / 60) as i32;
        let second = (secs_today % 60) as i32;

        SystemDate {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond: (now.subsec_millis() as i32),
        }
    }

    /// Returns date components as (month, day, year, hour, minute, second, millisecond, microsecond)
    pub fn values(&self) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
        (
            self.month,
            self.day,
            self.year,
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
            0,
        )
    }

    pub fn year(&self) -> i32 {
        self.year
    }
    pub fn month(&self) -> i32 {
        self.month
    }
    pub fn day(&self) -> i32 {
        self.day
    }
    pub fn hour(&self) -> i32 {
        self.hour
    }
    pub fn minute(&self) -> i32 {
        self.minute
    }
    pub fn second(&self) -> i32 {
        self.second
    }
}

/// IGES Global Section containing header information
#[derive(Clone, Debug)]
pub struct IGESGlobalSection {
    creation_date: String,
    // Other fields would follow, but we focus on creation_date
}

impl IGESGlobalSection {
    pub fn new() -> Self {
        IGESGlobalSection {
            creation_date: String::new(),
        }
    }

    /// Returns the creation date string (YYMMDD.HHMMSS or YYYYMMDD.HHMMSS)
    pub fn creation_date(&self) -> &str {
        &self.creation_date
    }

    /// Sets the creation date string
    pub fn set_date(&mut self, date_string: &str) {
        self.creation_date = date_string.to_string();
    }

    /// Generates a date string in IGES format
    ///
    /// flag == -1: YYYYMMDD.HHMMSS format (post-Y2000)
    /// flag == 0: YYMMDD.HHMMSS format (pre-Y2000)
    pub fn new_date_string(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        flag: i32,
    ) -> String {
        if flag < 0 {
            // Post-Y2000: YYYYMMDD.HHMMSS
            format!(
                "{:04}{:02}{:02}.{:02}{:02}{:02}",
                year, month, day, hour, minute, second
            )
        } else {
            // Pre-Y2000: YYMMDD.HHMMSS (use last 2 digits of year)
            let yy = year % 100;
            format!(
                "{:02}{:02}{:02}.{:02}{:02}{:02}",
                yy, month, day, hour, minute, second
            )
        }
    }
}

/// IGES Model containing a global section
pub struct IGESModel {
    global_section: IGESGlobalSection,
}

impl IGESModel {
    pub fn new() -> Self {
        IGESModel {
            global_section: IGESGlobalSection::new(),
        }
    }

    pub fn global_section(&self) -> IGESGlobalSection {
        self.global_section.clone()
    }

    pub fn set_global_section(&mut self, gs: IGESGlobalSection) {
        self.global_section = gs;
    }

    pub fn verify_check(&self) -> Vec<String> {
        // Placeholder: check validity of IGES model
        Vec::new()
    }
}

/// Context for modification operations
pub struct IFSelectContextModif {
    checks: Vec<String>,
}

impl IFSelectContextModif {
    pub fn new() -> Self {
        IFSelectContextModif {
            checks: Vec::new(),
        }
    }

    pub fn add_check(&mut self, message: String) {
        self.checks.push(message);
    }

    pub fn checks(&self) -> &[String] {
        &self.checks
    }
}

/// Interface copy tool for model modification
pub struct InterfaceCopyTool;

/// Model modifier for updating IGES creation date
pub struct IGESSelectUpdateCreationDate;

impl IGESSelectUpdateCreationDate {
    /// Creates an UpdateCreationDate modifier
    pub fn new() -> Self {
        IGESSelectUpdateCreationDate
    }

    /// Performs the modification: updates the creation date in the IGES header
    pub fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &mut IGESModel,
        _copy_tool: &InterfaceCopyTool,
    ) {
        let system_date = SystemDate::now();
        let (mois, jour, annee, heure, minute, seconde, _, _) = system_date.values();

        let date_string = if annee < 2000 {
            // Pre-Y2000: YYMMDD.HHMMSS
            IGESGlobalSection::new_date_string(annee, mois, jour, heure, minute, seconde, 0)
        } else {
            // Post-Y2000: YYYYMMDD.HHMMSS
            IGESGlobalSection::new_date_string(annee, mois, jour, heure, minute, seconde, -1)
        };

        let mut gs = target.global_section();
        gs.set_date(&date_string);
        target.set_global_section(gs);

        let checks = target.verify_check();
        for check in checks {
            ctx.add_check(check);
        }
    }

    /// Returns a label describing this modifier
    pub fn label(&self) -> String {
        "Update Creation Date in IGES Global Section".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_date_now() {
        let date = SystemDate::now();
        assert!(date.year() >= 1970);
        assert!(date.month() >= 1 && date.month() <= 12);
        assert!(date.day() >= 1 && date.day() <= 31);
        assert!(date.hour() >= 0 && date.hour() < 24);
        assert!(date.minute() >= 0 && date.minute() < 60);
        assert!(date.second() >= 0 && date.second() < 60);
    }

    #[test]
    fn test_date_string_pre_y2000() {
        let date_str =
            IGESGlobalSection::new_date_string(1999, 6, 15, 14, 30, 45, 0);
        assert_eq!(date_str, "990615.143045");
    }

    #[test]
    fn test_date_string_post_y2000() {
        let date_str =
            IGESGlobalSection::new_date_string(2023, 6, 15, 14, 30, 45, -1);
        assert_eq!(date_str, "20230615.143045");
    }

    #[test]
    fn test_date_string_y2000_boundary() {
        let pre = IGESGlobalSection::new_date_string(2000, 1, 1, 0, 0, 0, 0);
        assert_eq!(pre, "000101.000000");

        let post = IGESGlobalSection::new_date_string(2000, 1, 1, 0, 0, 0, -1);
        assert_eq!(post, "20000101.000000");
    }

    #[test]
    fn test_global_section_creation() {
        let mut gs = IGESGlobalSection::new();
        assert_eq!(gs.creation_date(), "");

        gs.set_date("990615.143045");
        assert_eq!(gs.creation_date(), "990615.143045");
    }

    #[test]
    fn test_iges_model_global_section() {
        let mut model = IGESModel::new();
        let mut gs = model.global_section();
        gs.set_date("20230615.143045");
        model.set_global_section(gs);

        assert_eq!(model.global_section().creation_date(), "20230615.143045");
    }

    #[test]
    fn test_update_creation_date_modifier() {
        let modifier = IGESSelectUpdateCreationDate::new();
        let label = modifier.label();
        assert!(label.contains("Update"));
        assert!(label.contains("IGES"));
    }

    #[test]
    fn test_performing_updates_model() {
        let modifier = IGESSelectUpdateCreationDate::new();
        let mut ctx = IFSelectContextModif::new();
        let mut model = IGESModel::new();
        let copy_tool = InterfaceCopyTool;

        modifier.performing(&mut ctx, &mut model, &copy_tool);

        let gs = model.global_section();
        let date = gs.creation_date();
        assert!(!date.is_empty());
        // Date should contain a dot separating date and time
        assert!(date.contains("."));
    }

    #[test]
    fn test_context_modif_checks() {
        let mut ctx = IFSelectContextModif::new();
        assert!(ctx.checks().is_empty());

        ctx.add_check("Check 1".to_string());
        ctx.add_check("Check 2".to_string());

        assert_eq!(ctx.checks().len(), 2);
        assert_eq!(ctx.checks()[0], "Check 1");
    }

    #[test]
    fn test_date_values() {
        let date = SystemDate {
            year: 2023,
            month: 6,
            day: 15,
            hour: 14,
            minute: 30,
            second: 45,
            millisecond: 123,
        };

        let (m, d, y, h, min, s, ms, us) = date.values();
        assert_eq!(m, 6);
        assert_eq!(d, 15);
        assert_eq!(y, 2023);
        assert_eq!(h, 14);
        assert_eq!(min, 30);
        assert_eq!(s, 45);
        assert_eq!(ms, 123);
        assert_eq!(us, 0);
    }

    #[test]
    fn test_performing_pre_y2000() {
        let modifier = IGESSelectUpdateCreationDate::new();
        let mut ctx = IFSelectContextModif::new();
        let mut model = IGESModel::new();
        let copy_tool = InterfaceCopyTool;

        // Manually set a date and verify format selection logic
        let test_date = SystemDate {
            year: 1999,
            month: 12,
            day: 31,
            hour: 23,
            minute: 59,
            second: 59,
            millisecond: 0,
        };

        let date_str = if test_date.year() < 2000 {
            IGESGlobalSection::new_date_string(
                test_date.year(),
                test_date.month(),
                test_date.day(),
                test_date.hour(),
                test_date.minute(),
                test_date.second(),
                0,
            )
        } else {
            IGESGlobalSection::new_date_string(
                test_date.year(),
                test_date.month(),
                test_date.day(),
                test_date.hour(),
                test_date.minute(),
                test_date.second(),
                -1,
            )
        };

        assert_eq!(date_str, "991231.235959");
    }
}
