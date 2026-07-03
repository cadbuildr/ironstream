// FILE: t_data_xtd_presentation.rs
// occt: TDataXtd_Presentation

use std::collections::VecDeque;

/// Represents the GUID for a presentation driver.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandardGUID {
    data: [u8; 16],
}

impl StandardGUID {
    pub fn new(data: [u8; 16]) -> Self {
        StandardGUID { data }
    }

    pub fn zero() -> Self {
        StandardGUID { data: [0; 16] }
    }

    pub fn get_id() -> Self {
        // "04fb4d00-5690-11d1-8940-080009dc3333"
        let bytes: [u8; 16] = [0x04, 0xfb, 0x4d, 0x00, 0x56, 0x90, 0x11, 0xd1,
                               0x89, 0x40, 0x08, 0x00, 0x09, 0xdc, 0x33, 0x33];
        StandardGUID { data: bytes }
    }
}

/// Enum for color names (simplified representation).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuantityNameOfColor {
    White = 0,
    Black = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
    // ... other colors would go here; for brevity, we use a limited set
    // In a full port, this would be an exhaustive enum
}

impl Default for QuantityNameOfColor {
    fn default() -> Self {
        QuantityNameOfColor::White
    }
}

/// Presentation attribute for shape display parameters.
/// Mirrors OCCT's TDataXtd_Presentation.
#[derive(Clone, Debug)]
pub struct TDataXtdPresentation {
    driver_guid: StandardGUID,
    color: QuantityNameOfColor,
    material_index: i32,
    mode: i32,
    transparency: f64,
    width: f64,
    is_displayed: bool,
    has_own_color: bool,
    has_own_material: bool,
    has_own_transparency: bool,
    has_own_width: bool,
    has_own_mode: bool,
    has_own_selection_mode: bool,
    selection_modes: VecDeque<i32>,
}

impl Default for TDataXtdPresentation {
    fn default() -> Self {
        TDataXtdPresentation {
            driver_guid: StandardGUID::zero(),
            color: QuantityNameOfColor::White,
            material_index: 0,
            mode: 0,
            transparency: 0.0,
            width: 0.0,
            is_displayed: false,
            has_own_color: false,
            has_own_material: false,
            has_own_transparency: false,
            has_own_width: false,
            has_own_mode: false,
            has_own_selection_mode: false,
            selection_modes: VecDeque::new(),
        }
    }
}

impl TDataXtdPresentation {
    /// Creates a new empty TDataXtd_Presentation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the ID of the attribute.
    pub fn get_id() -> StandardGUID {
        StandardGUID::get_id()
    }

    /// Returns the GUID of the driver managing display.
    pub fn driver_guid(&self) -> &StandardGUID {
        &self.driver_guid
    }

    /// Sets the GUID of the driver managing display.
    pub fn set_driver_guid(&mut self, guid: StandardGUID) {
        if self.driver_guid != guid {
            self.driver_guid = guid;
        }
    }

    /// Returns whether the presentation is displayed.
    pub fn is_displayed(&self) -> bool {
        self.is_displayed
    }

    /// Sets the display state.
    pub fn set_displayed(&mut self, displayed: bool) {
        if self.is_displayed != displayed {
            self.is_displayed = displayed;
        }
    }

    /// Returns whether the material index has been set.
    pub fn has_own_material(&self) -> bool {
        self.has_own_material
    }

    /// Returns the material index.
    pub fn material_index(&self) -> i32 {
        self.material_index
    }

    /// Sets the material index.
    pub fn set_material_index(&mut self, index: i32) {
        if !self.has_own_material || self.material_index != index {
            self.material_index = index;
            self.has_own_material = true;
        }
    }

    /// Unsets the material index.
    pub fn unset_material(&mut self) {
        if self.has_own_material {
            self.has_own_material = false;
        }
    }

    /// Returns whether transparency has been set.
    pub fn has_own_transparency(&self) -> bool {
        self.has_own_transparency
    }

    /// Returns the transparency value.
    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    /// Sets the transparency value.
    pub fn set_transparency(&mut self, value: f64) {
        if !self.has_own_transparency || (self.transparency - value).abs() > 1e-10 {
            self.transparency = value;
            self.has_own_transparency = true;
        }
    }

    /// Unsets the transparency.
    pub fn unset_transparency(&mut self) {
        if self.has_own_transparency {
            self.has_own_transparency = false;
        }
    }

    /// Returns whether color has been set.
    pub fn has_own_color(&self) -> bool {
        self.has_own_color
    }

    /// Returns the color.
    pub fn color(&self) -> QuantityNameOfColor {
        self.color
    }

    /// Sets the color.
    pub fn set_color(&mut self, color: QuantityNameOfColor) {
        if !self.has_own_color || self.color != color {
            self.color = color;
            self.has_own_color = true;
        }
    }

    /// Unsets the color.
    pub fn unset_color(&mut self) {
        if self.has_own_color {
            self.has_own_color = false;
        }
    }

    /// Returns whether width has been set.
    pub fn has_own_width(&self) -> bool {
        self.has_own_width
    }

    /// Returns the width.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Sets the width.
    pub fn set_width(&mut self, width: f64) {
        if !self.has_own_width || (self.width - width).abs() > 1e-10 {
            self.width = width;
            self.has_own_width = true;
        }
    }

    /// Unsets the width.
    pub fn unset_width(&mut self) {
        if self.has_own_width {
            self.has_own_width = false;
        }
    }

    /// Returns whether mode has been set.
    pub fn has_own_mode(&self) -> bool {
        self.has_own_mode
    }

    /// Returns the mode.
    pub fn mode(&self) -> i32 {
        self.mode
    }

    /// Sets the mode.
    pub fn set_mode(&mut self, mode: i32) {
        if !self.has_own_mode || self.mode != mode {
            self.mode = mode;
            self.has_own_mode = true;
        }
    }

    /// Unsets the mode.
    pub fn unset_mode(&mut self) {
        if self.has_own_mode {
            self.has_own_mode = false;
        }
    }

    /// Returns whether selection modes have been set.
    pub fn has_own_selection_mode(&self) -> bool {
        self.has_own_selection_mode
    }

    /// Returns the number of selection modes.
    pub fn get_nb_selection_modes(&self) -> usize {
        self.selection_modes.len()
    }

    /// Returns the selection mode at the given index (1-indexed).
    pub fn selection_mode(&self, index: usize) -> i32 {
        if index > 0 && index <= self.selection_modes.len() {
            self.selection_modes[index - 1]
        } else {
            0
        }
    }

    /// Sets the selection mode, clearing any previous modes.
    pub fn set_selection_mode(&mut self, mode: i32, _transaction: bool) {
        if !self.has_own_selection_mode || self.selection_modes.len() > 1
            || (self.selection_modes.len() > 0 && self.selection_modes[0] != mode)
        {
            self.selection_modes.clear();
            self.selection_modes.push_back(mode);
            self.has_own_selection_mode = true;
        }
    }

    /// Adds a selection mode if not already present.
    pub fn add_selection_mode(&mut self, mode: i32, _transaction: bool) {
        if !self.has_own_selection_mode || !self.has_selection_mode(mode) {
            self.selection_modes.push_back(mode);
            self.has_own_selection_mode = true;
        }
    }

    /// Unsets all selection modes.
    pub fn unset_selection_mode(&mut self) {
        if self.has_own_selection_mode {
            self.has_own_selection_mode = false;
            self.selection_modes.clear();
        }
    }

    /// Checks if a selection mode exists.
    fn has_selection_mode(&self, mode: i32) -> bool {
        self.selection_modes.iter().any(|&m| m == mode)
    }

    /// Converts old color enum value to new color name.
    pub fn get_color_name_from_old_enum(old: i32) -> QuantityNameOfColor {
        // Simplified conversion logic (full OCCT logic is extensive)
        match old {
            0 => QuantityNameOfColor::White,
            1 => QuantityNameOfColor::Black,
            2 => QuantityNameOfColor::Red,
            3 => QuantityNameOfColor::Green,
            4 => QuantityNameOfColor::Blue,
            _ => QuantityNameOfColor::White,
        }
    }

    /// Converts new color name to old enum value.
    pub fn get_old_color_name_from_new_enum(color: QuantityNameOfColor) -> i32 {
        match color {
            QuantityNameOfColor::White => 0,
            QuantityNameOfColor::Black => 1,
            QuantityNameOfColor::Red => 2,
            QuantityNameOfColor::Green => 3,
            QuantityNameOfColor::Blue => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        let pres = TDataXtdPresentation::new();
        assert!(!pres.is_displayed());
        assert_eq!(pres.material_index(), 0);
        assert_eq!(pres.transparency(), 0.0);
        assert_eq!(pres.width(), 0.0);
        assert_eq!(pres.mode(), 0);
        assert_eq!(pres.get_nb_selection_modes(), 0);
    }

    #[test]
    fn test_set_displayed() {
        let mut pres = TDataXtdPresentation::new();
        pres.set_displayed(true);
        assert!(pres.is_displayed());
    }

    #[test]
    fn test_material_index() {
        let mut pres = TDataXtdPresentation::new();
        assert!(!pres.has_own_material());
        pres.set_material_index(42);
        assert!(pres.has_own_material());
        assert_eq!(pres.material_index(), 42);
        pres.unset_material();
        assert!(!pres.has_own_material());
    }

    #[test]
    fn test_transparency() {
        let mut pres = TDataXtdPresentation::new();
        assert!(!pres.has_own_transparency());
        pres.set_transparency(0.5);
        assert!(pres.has_own_transparency());
        assert!((pres.transparency() - 0.5).abs() < 1e-10);
        pres.unset_transparency();
        assert!(!pres.has_own_transparency());
    }

    #[test]
    fn test_color() {
        let mut pres = TDataXtdPresentation::new();
        assert!(!pres.has_own_color());
        pres.set_color(QuantityNameOfColor::Red);
        assert!(pres.has_own_color());
        assert_eq!(pres.color(), QuantityNameOfColor::Red);
        pres.unset_color();
        assert!(!pres.has_own_color());
    }

    #[test]
    fn test_width() {
        let mut pres = TDataXtdPresentation::new();
        assert!(!pres.has_own_width());
        pres.set_width(2.5);
        assert!(pres.has_own_width());
        assert!((pres.width() - 2.5).abs() < 1e-10);
        pres.unset_width();
        assert!(!pres.has_own_width());
    }

    #[test]
    fn test_mode() {
        let mut pres = TDataXtdPresentation::new();
        assert!(!pres.has_own_mode());
        pres.set_mode(3);
        assert!(pres.has_own_mode());
        assert_eq!(pres.mode(), 3);
        pres.unset_mode();
        assert!(!pres.has_own_mode());
    }

    #[test]
    fn test_selection_modes() {
        let mut pres = TDataXtdPresentation::new();
        assert!(!pres.has_own_selection_mode());
        pres.set_selection_mode(1, true);
        assert!(pres.has_own_selection_mode());
        assert_eq!(pres.get_nb_selection_modes(), 1);
        assert_eq!(pres.selection_mode(1), 1);

        pres.add_selection_mode(2, true);
        assert_eq!(pres.get_nb_selection_modes(), 2);
        assert_eq!(pres.selection_mode(2), 2);

        pres.unset_selection_mode();
        assert!(!pres.has_own_selection_mode());
        assert_eq!(pres.get_nb_selection_modes(), 0);
    }

    #[test]
    fn test_driver_guid() {
        let mut pres = TDataXtdPresentation::new();
        let guid = StandardGUID::get_id();
        pres.set_driver_guid(guid.clone());
        assert_eq!(pres.driver_guid(), &guid);
    }

    #[test]
    fn test_color_enum_conversion() {
        let old = 2;
        let color = TDataXtdPresentation::get_color_name_from_old_enum(old);
        assert_eq!(color, QuantityNameOfColor::Red);

        let back = TDataXtdPresentation::get_old_color_name_from_new_enum(color);
        assert_eq!(back, 2);
    }
}
