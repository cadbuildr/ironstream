// FILE: iges_select_update_last_change.rs
// occt: IGESSelect_UpdateLastChange

//! Modifier that updates the IGES file header last change date to the current system date
//! and optionally upgrades the IGES version to 5.1 (version code 9) if older.
//!
//! This modifier acts on IGES Global Section Item 25 (last change date) and Item 23 (IGES version).
//! The date is formatted as YYMMDD.HHMMSS (before Y2000) or YYYYMMDD.HHMMSS (after Y2000).

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
    last_change_date: String,
    iges_version: i32, // 9 = IGES 5.1
}

impl IGESGlobalSection {
    pub fn new() -> Self {
        IGESGlobalSection {
            last_change_date: String::new(),
            iges_version: 5, // Default to IGES 5.0
        }
    }

    pub fn last_change_date(&self) -> &str {
        &self.last_change_date
    }

    pub fn set_last_change_date(&mut self, date_string: &str) {
        self.last_change_date = date_string.to_string();
    }

    pub fn iges_version(&self) -> i32 {
        self.iges_version
    }

    pub fn set_iges_version(&mut self, version: i32) {
        self.iges_version = version;
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
            format!(
                "{:04}{:02}{:02}.{:02}{:02}{:02}",
                year, month, day, hour, minute, second
            )
        } else {
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

/// Model modifier for updating IGES last change date
pub struct IGESSelectUpdateLastChange;

impl IGESSelectUpdateLastChange {
    /// Creates an UpdateLastChange modifier
    pub fn new() -> Self {
        IGESSelectUpdateLastChange
    }

    /// Performs the modification: updates the last change date and IGES version
    pub fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &mut IGESModel,
        _copy_tool: &InterfaceCopyTool,
    ) {
        let system_date = SystemDate::now();
        let annee = system_date.year();
        let mois = system_date.month();
        let jour = system_date.day();
        let heure = system_date.hour();
        let minute = system_date.minute();
        let seconde = system_date.second();

        let mut gs = target.global_section();

        // Upgrade IGES version to 5.1 (code 9) if older
        if gs.iges_version() < 9 {
            gs.set_iges_version(9);
        }

        // Set last change date with appropriate format for Y2K
        let date_string = if annee < 2000 {
            IGESGlobalSection::new_date_string(annee, mois, jour, heure, minute, seconde, 0)
        } else {
            IGESGlobalSection::new_date_string(annee, mois, jour, heure, minute, seconde, -1)
        };

        gs.set_last_change_date(&date_string);
        target.set_global_section(gs);

        let checks = target.verify_check();
        for check in checks {
            ctx.add_check(check);
        }
    }

    /// Returns a label describing this modifier
    pub fn label(&self) -> String {
        "Update Last Change Date in IGES Global Section".to_string()
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
    }

    #[test]
    fn test_date_string_pre_y2000() {
        let date_str = IGESGlobalSection::new_date_string(1999, 6, 15, 14, 30, 45, 0);
        assert_eq!(date_str, "990615.143045");
    }

    #[test]
    fn test_date_string_post_y2000() {
        let date_str = IGESGlobalSection::new_date_string(2023, 6, 15, 14, 30, 45, -1);
        assert_eq!(date_str, "20230615.143045");
    }

    #[test]
    fn test_global_section_iges_version() {
        let mut gs = IGESGlobalSection::new();
        assert_eq!(gs.iges_version(), 5);

        gs.set_iges_version(9);
        assert_eq!(gs.iges_version(), 9);
    }

    #[test]
    fn test_global_section_last_change_date() {
        let mut gs = IGESGlobalSection::new();
        assert_eq!(gs.last_change_date(), "");

        gs.set_last_change_date("990615.143045");
        assert_eq!(gs.last_change_date(), "990615.143045");
    }

    #[test]
    fn test_iges_model_operations() {
        let mut model = IGESModel::new();
        let mut gs = model.global_section();
        gs.set_iges_version(5);
        gs.set_last_change_date("20230615.143045");
        model.set_global_section(gs);

        let gs = model.global_section();
        assert_eq!(gs.iges_version(), 5);
        assert_eq!(gs.last_change_date(), "20230615.143045");
    }

    #[test]
    fn test_modifier_creation() {
        let modifier = IGESSelectUpdateLastChange::new();
        let label = modifier.label();
        assert_eq!(label, "Update Last Change Date in IGES Global Section");
    }

    #[test]
    fn test_performing_updates_version() {
        let modifier = IGESSelectUpdateLastChange::new();
        let mut ctx = IFSelectContextModif::new();
        let mut model = IGESModel::new();
        let copy_tool = InterfaceCopyTool;

        // Set initial version to old value
        let mut gs = model.global_section();
        gs.set_iges_version(5);
        model.set_global_section(gs);

        modifier.performing(&mut ctx, &mut model, &copy_tool);

        // Version should be upgraded to 9
        assert_eq!(model.global_section().iges_version(), 9);
    }

    #[test]
    fn test_performing_updates_date() {
        let modifier = IGESSelectUpdateLastChange::new();
        let mut ctx = IFSelectContextModif::new();
        let mut model = IGESModel::new();
        let copy_tool = InterfaceCopyTool;

        modifier.performing(&mut ctx, &mut model, &copy_tool);

        let gs = model.global_section();
        let date = gs.last_change_date();
        assert!(!date.is_empty());
        assert!(date.contains("."));
    }

    #[test]
    fn test_performing_preserves_new_version() {
        let modifier = IGESSelectUpdateLastChange::new();
        let mut ctx = IFSelectContextModif::new();
        let mut model = IGESModel::new();
        let copy_tool = InterfaceCopyTool;

        // Set version to 9 already
        let mut gs = model.global_section();
        gs.set_iges_version(9);
        model.set_global_section(gs);

        modifier.performing(&mut ctx, &mut model, &copy_tool);

        // Version should remain 9
        assert_eq!(model.global_section().iges_version(), 9);
    }

    #[test]
    fn test_context_modif_checks() {
        let mut ctx = IFSelectContextModif::new();
        assert!(ctx.checks().is_empty());

        ctx.add_check("Check 1".to_string());
        assert_eq!(ctx.checks().len(), 1);
    }

    #[test]
    fn test_date_string_boundary_y2000() {
        let pre = IGESGlobalSection::new_date_string(2000, 1, 1, 0, 0, 0, 0);
        assert_eq!(pre, "000101.000000");

        let post = IGESGlobalSection::new_date_string(2000, 1, 1, 0, 0, 0, -1);
        assert_eq!(post, "20000101.000000");
    }
}
