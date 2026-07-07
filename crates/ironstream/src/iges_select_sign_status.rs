// FILE: iges_select_sign_status.rs
// occt: IGESSelect_SignStatus

/// Signature providing entity status information in the form "i,j,k,l":
/// - i: BlankStatus (0=Visible, 1=Blanked) or B/V
/// - j: SubordinateStatus (0=Independent, 1=Physical, 2=Logical, 3=Both) or I/P/L/D
/// - k: UseFlag
/// - l: Hierarchy
pub struct IgesSelectSignStatus {}

impl IgesSelectSignStatus {
    /// Creates a SignStatus.
    pub fn new() -> Self {
        IgesSelectSignStatus {}
    }

    /// Returns the status as a formatted string "i,j,k,l".
    ///
    /// # Arguments
    /// - `_entity`: The IGES entity
    /// - `blank`: BlankStatus (0 or 1)
    /// - `subordinate`: SubordinateStatus (0, 1, 2, or 3)
    /// - `use_flag`: UseFlag value
    /// - `hierarchy`: Hierarchy value
    pub fn value(
        &self,
        _entity: Option<&dyn std::any::Any>,
        blank: i32,
        subordinate: i32,
        use_flag: i32,
        hierarchy: i32,
    ) -> String {
        format!("{},{},{},{}", blank, subordinate, use_flag, hierarchy)
    }

    /// Performs matching with optional shortcuts and wildcards.
    ///
    /// # Arguments
    /// - `_entity`: The IGES entity
    /// - `text`: Pattern to match (e.g., "V,*,*,*" or "B" or "I")
    /// - `exact`: If true, requires exact match; if false, allows shortcuts
    /// - `blank`: Current BlankStatus
    /// - `subordinate`: Current SubordinateStatus
    ///
    /// Returns true if the pattern matches
    pub fn matches(
        &self,
        _entity: Option<&dyn std::any::Any>,
        text: &str,
        exact: bool,
        blank: i32,
        subordinate: i32,
    ) -> bool {
        // Mirrors IGESSelect_SignStatus::Matches (IGESSelect_SignStatus.cxx):
        // the pattern is scanned position by position, commas advancing the
        // position counter (0=Blank, 1=Subordinate, 2=UseFlag, 3=Hierarchy).
        // Letter shortcuts: B/V for Blank, I/P/L/D for Subordinate. A star (or
        // nothing) between commas means "this status is OK".
        // Exact mode: every checked position must match (mismatch -> false).
        // Non-exact mode: the first matching position returns true.
        // Note: this port receives only Blank and Subordinate, so UseFlag and
        // Hierarchy positions cannot be checked and are skipped.
        let has_comma = text.contains(',');
        let mut vir = 0; // number of commas passed, i.e. current position
        let mut checked_any = false;
        for car in text.chars().take(9) {
            if car == ',' {
                vir += 1;
                continue;
            }
            if car == '*' {
                // "a star between commas : this status is OK"
                continue;
            }
            // Determine the addressed position and the value to compare
            let (pos, val) = match car {
                'V' if vir == 0 => (0, 0),
                'B' if vir == 0 => (0, 1),
                'I' | 'P' | 'L' | 'D' => {
                    let v = match car {
                        'I' => 0,
                        'P' => 1,
                        'L' => 2,
                        _ => 3,
                    };
                    if vir == 1 || (!exact && !has_comma) {
                        // "a letter, no comma : only this status is checked"
                        (1, v)
                    } else {
                        (vir, car as i32 - 48)
                    }
                }
                _ => (vir, car as i32 - 48),
            };
            let current = match pos {
                0 => Some(blank),
                1 => Some(subordinate),
                _ => None, // UseFlag / Hierarchy not available in this port
            };
            if let Some(cur) = current {
                checked_any = true;
                if cur == val && !exact {
                    return true;
                }
                if cur != val && exact {
                    return false;
                }
            }
        }
        // Exact mode: no mismatch found -> matches.
        // Non-exact mode: no concrete criterion was given (empty / all stars)
        // -> everything is OK; otherwise no criterion matched -> false.
        if exact {
            true
        } else {
            !checked_any
        }
    }

    /// Returns the signature name.
    pub fn name(&self) -> String {
        "Status".to_string()
    }
}

impl Default for IgesSelectSignStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_status_creation() {
        let ss = IgesSelectSignStatus::new();
        assert_eq!(ss.name(), "Status".to_string());
    }

    #[test]
    fn test_sign_status_value() {
        let ss = IgesSelectSignStatus::new();
        assert_eq!(ss.value(None, 0, 1, 2, 3), "0,1,2,3".to_string());
        assert_eq!(ss.value(None, 1, 0, 4, 5), "1,0,4,5".to_string());
    }

    #[test]
    fn test_sign_status_matches_blank_status() {
        let ss = IgesSelectSignStatus::new();
        assert!(ss.matches(None, "B", false, 1, 0)); // Blanked
        assert!(ss.matches(None, "V", false, 0, 1)); // Visible
        assert!(!ss.matches(None, "B", false, 0, 0)); // Not blanked
    }

    #[test]
    fn test_sign_status_matches_subordinate() {
        let ss = IgesSelectSignStatus::new();
        assert!(ss.matches(None, "I", false, 0, 0)); // Independent
        assert!(ss.matches(None, "P", false, 0, 1)); // Physically Dependent
        assert!(ss.matches(None, "L", false, 0, 2)); // Logically Dependent
        assert!(ss.matches(None, "D", false, 0, 3)); // Dependent (both)
    }

    #[test]
    fn test_sign_status_matches_wildcard() {
        let ss = IgesSelectSignStatus::new();
        assert!(ss.matches(None, "*", false, 0, 0));
        assert!(ss.matches(None, "*", false, 1, 3));
    }

    #[test]
    fn test_sign_status_matches_exact_mode() {
        let ss = IgesSelectSignStatus::new();
        assert!(ss.matches(None, "0,1,2,3", true, 0, 1));
        assert!(!ss.matches(None, "1,0,2,3", true, 0, 1));
    }
}
