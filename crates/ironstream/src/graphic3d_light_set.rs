// FILE: graphic3d_light_set.rs
// occt: Graphic3d_LightSet
// occt: Graphic3d_LightSet::Iterator
// occt: Graphic3d_LightSet::IterationFilter

use core::fmt;

/// Type of light source
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TypeOfLightSource {
    Ambient = 0,
    Directional = 1,
    Point = 2,
    Spot = 3,
}

pub const TYPE_OF_LIGHT_SOURCE_NB: usize = 4;

/// Iteration filter flags
#[derive(Debug, Clone, Copy)]
pub struct IterationFilter(u32);

impl IterationFilter {
    pub const NONE: IterationFilter = IterationFilter(0x0000);
    pub const EXCLUDE_AMBIENT: IterationFilter = IterationFilter(0x0002);
    pub const EXCLUDE_DISABLED: IterationFilter = IterationFilter(0x0004);
    pub const EXCLUDE_NO_SHADOW: IterationFilter = IterationFilter(0x0008);
    pub const EXCLUDE_DISABLED_AND_AMBIENT: IterationFilter =
        IterationFilter(0x0002 | 0x0004);
    pub const ACTIVE_SHADOW_CASTERS: IterationFilter = IterationFilter(0x0002 | 0x0004 | 0x0008);

    pub fn new(value: u32) -> Self {
        IterationFilter(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn has_flag(&self, flag: IterationFilter) -> bool {
        (self.0 & flag.0) != 0
    }
}

impl Default for IterationFilter {
    fn default() -> Self {
        IterationFilter::NONE
    }
}

/// Mock light source representation
#[derive(Debug, Clone)]
pub struct CLight {
    enabled: bool,
    light_type: TypeOfLightSource,
    cast_shadows: bool,
}

impl CLight {
    pub fn new(light_type: TypeOfLightSource) -> Self {
        CLight {
            enabled: true,
            light_type,
            cast_shadows: false,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn light_type(&self) -> TypeOfLightSource {
        self.light_type
    }

    pub fn to_cast_shadows(&self) -> bool {
        self.cast_shadows
    }

    pub fn set_cast_shadows(&mut self, cast: bool) {
        self.cast_shadows = cast;
    }
}

/// Iterator through light sources
pub struct LightIterator {
    lights: Vec<CLight>,
    index: usize,
    filter: IterationFilter,
}

impl LightIterator {
    pub fn new(lights: &[CLight], filter: IterationFilter) -> Self {
        let lights = lights.to_vec();
        let mut iter = LightIterator {
            lights,
            index: 0,
            filter,
        };
        iter.skip_filtered();
        iter
    }

    /// Returns TRUE if iterator points to a valid item
    pub fn more(&self) -> bool {
        self.index < self.lights.len()
    }

    /// Returns current item
    pub fn value(&self) -> Option<&CLight> {
        if self.index < self.lights.len() {
            Some(&self.lights[self.index])
        } else {
            None
        }
    }

    /// Moves to the next item
    pub fn next(&mut self) {
        if self.index < self.lights.len() {
            self.index += 1;
        }
        self.skip_filtered();
    }

    /// Skip filtered items
    fn skip_filtered(&mut self) {
        if self.filter.value() == 0 {
            return;
        }

        while self.index < self.lights.len() {
            let light = &self.lights[self.index];

            if self.filter.has_flag(IterationFilter::EXCLUDE_AMBIENT)
                && light.light_type == TypeOfLightSource::Ambient
            {
                self.index += 1;
                continue;
            }

            if self.filter.has_flag(IterationFilter::EXCLUDE_DISABLED) && !light.is_enabled() {
                self.index += 1;
                continue;
            }

            if self.filter.has_flag(IterationFilter::EXCLUDE_NO_SHADOW) && !light.to_cast_shadows()
            {
                self.index += 1;
                continue;
            }

            break;
        }
    }
}

/// Class defining the set of light sources
pub struct LightSet {
    lights: Vec<CLight>,
    light_types: [i32; TYPE_OF_LIGHT_SOURCE_NB],
    light_types_enabled: [i32; TYPE_OF_LIGHT_SOURCE_NB],
    nb_enabled: i32,
    nb_cast_shadows: i32,
    ambient: [f32; 4],
    key_enabled_long: String,
    key_enabled_short: String,
    revision: usize,
    cache_revision: usize,
}

impl LightSet {
    /// Creates a new empty light set
    pub fn new() -> Self {
        LightSet {
            lights: Vec::new(),
            light_types: [0; TYPE_OF_LIGHT_SOURCE_NB],
            light_types_enabled: [0; TYPE_OF_LIGHT_SOURCE_NB],
            nb_enabled: 0,
            nb_cast_shadows: 0,
            ambient: [0.0, 0.0, 0.0, 1.0],
            key_enabled_long: String::new(),
            key_enabled_short: String::new(),
            revision: 0,
            cache_revision: 0,
        }
    }

    /// Return lower light index
    pub fn lower(&self) -> i32 {
        1
    }

    /// Return upper light index
    pub fn upper(&self) -> i32 {
        self.lights.len() as i32
    }

    /// Return TRUE if lights list is empty
    pub fn is_empty(&self) -> bool {
        self.lights.is_empty()
    }

    /// Return number of light sources
    pub fn extent(&self) -> i32 {
        self.lights.len() as i32
    }

    /// Return the light source for specified index within range [lower(), upper()]
    pub fn value(&self, index: i32) -> Option<&CLight> {
        if index >= 1 && (index as usize) <= self.lights.len() {
            Some(&self.lights[(index - 1) as usize])
        } else {
            None
        }
    }

    /// Return TRUE if light source is defined in this set
    pub fn contains(&self, light: &CLight) -> bool {
        self.lights.iter().any(|l| {
            l.is_enabled() == light.is_enabled()
                && l.light_type() == light.light_type()
                && l.to_cast_shadows() == light.to_cast_shadows()
        })
    }

    /// Append new light source
    pub fn add(&mut self, light: CLight) -> bool {
        if !self.contains(&light) {
            self.light_types[light.light_type() as usize] += 1;
            if light.is_enabled() {
                self.light_types_enabled[light.light_type() as usize] += 1;
                if light.light_type() != TypeOfLightSource::Ambient {
                    self.nb_enabled += 1;
                }
                if light.to_cast_shadows() {
                    self.nb_cast_shadows += 1;
                }
            }
            self.lights.push(light);
            self.revision += 1;
            true
        } else {
            false
        }
    }

    /// Remove light source
    pub fn remove(&mut self, light: &CLight) -> bool {
        if let Some(pos) = self.lights.iter().position(|l| {
            l.is_enabled() == light.is_enabled()
                && l.light_type() == light.light_type()
                && l.to_cast_shadows() == light.to_cast_shadows()
        }) {
            let removed = self.lights.remove(pos);
            self.light_types[removed.light_type() as usize] -= 1;
            if removed.is_enabled() {
                self.light_types_enabled[removed.light_type() as usize] -= 1;
                if removed.light_type() != TypeOfLightSource::Ambient {
                    self.nb_enabled -= 1;
                }
                if removed.to_cast_shadows() {
                    self.nb_cast_shadows -= 1;
                }
            }
            self.revision += 1;
            true
        } else {
            false
        }
    }

    /// Returns total amount of lights of specified type
    pub fn nb_lights_of_type(&self, light_type: TypeOfLightSource) -> i32 {
        self.light_types[light_type as usize]
    }

    /// Update light sources revision
    pub fn update_revision(&mut self) -> usize {
        self.cache_revision = self.revision;
        self.revision
    }

    /// Return light sources revision
    pub fn revision(&self) -> usize {
        self.revision
    }

    /// Returns total amount of enabled lights EXCLUDING ambient
    pub fn nb_enabled(&self) -> i32 {
        self.nb_enabled
    }

    /// Returns total amount of enabled lights of specified type
    pub fn nb_enabled_lights_of_type(&self, light_type: TypeOfLightSource) -> i32 {
        self.light_types_enabled[light_type as usize]
    }

    /// Returns total amount of enabled lights casting shadows
    pub fn nb_cast_shadows(&self) -> i32 {
        self.nb_cast_shadows
    }

    /// Returns cumulative ambient color
    pub fn ambient_color(&self) -> &[f32; 4] {
        &self.ambient
    }

    /// Returns a string defining enabled light sources
    pub fn key_enabled_long(&self) -> &str {
        &self.key_enabled_long
    }

    /// Returns a short key of enabled light sources
    pub fn key_enabled_short(&self) -> &str {
        &self.key_enabled_short
    }

    /// Create an iterator for this light set
    pub fn create_iterator(&self, filter: IterationFilter) -> LightIterator {
        LightIterator::new(&self.lights, filter)
    }
}

impl Default for LightSet {
    fn default() -> Self {
        LightSet::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_set_creation() {
        let set = LightSet::new();
        assert!(set.is_empty());
        assert_eq!(set.extent(), 0);
    }

    #[test]
    fn test_light_set_add() {
        let mut set = LightSet::new();
        let light = CLight::new(TypeOfLightSource::Directional);
        assert!(set.add(light));
        assert_eq!(set.extent(), 1);
        assert!(!set.is_empty());
    }

    #[test]
    fn test_light_set_add_duplicate() {
        let mut set = LightSet::new();
        let light = CLight::new(TypeOfLightSource::Point);
        assert!(set.add(light.clone()));
        assert!(!set.add(light));
        assert_eq!(set.extent(), 1);
    }

    #[test]
    fn test_light_set_remove() {
        let mut set = LightSet::new();
        let light = CLight::new(TypeOfLightSource::Spot);
        set.add(light.clone());
        assert!(set.remove(&light));
        assert!(set.is_empty());
    }

    #[test]
    fn test_light_set_value() {
        let mut set = LightSet::new();
        let light = CLight::new(TypeOfLightSource::Directional);
        set.add(light.clone());
        let retrieved = set.value(1);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_light_set_contains() {
        let mut set = LightSet::new();
        let light = CLight::new(TypeOfLightSource::Point);
        assert!(!set.contains(&light));
        set.add(light.clone());
        assert!(set.contains(&light));
    }

    #[test]
    fn test_light_set_nb_lights_of_type() {
        let mut set = LightSet::new();
        let light1 = CLight::new(TypeOfLightSource::Directional);
        let light2 = CLight::new(TypeOfLightSource::Directional);
        set.add(light1);
        assert_eq!(set.nb_lights_of_type(TypeOfLightSource::Directional), 1);
    }

    #[test]
    fn test_light_set_revision() {
        let mut set = LightSet::new();
        let rev1 = set.revision();
        let light = CLight::new(TypeOfLightSource::Point);
        set.add(light);
        let rev2 = set.revision();
        assert!(rev2 > rev1);
    }

    #[test]
    fn test_iteration_filter() {
        let filter = IterationFilter::EXCLUDE_AMBIENT;
        assert!(filter.has_flag(IterationFilter::EXCLUDE_AMBIENT));
        assert!(!filter.has_flag(IterationFilter::EXCLUDE_DISABLED));
    }

    #[test]
    fn test_light_iterator() {
        let mut set = LightSet::new();
        let light1 = CLight::new(TypeOfLightSource::Directional);
        let light2 = CLight::new(TypeOfLightSource::Point);
        set.add(light1);
        set.add(light2);

        let mut iter = set.create_iterator(IterationFilter::NONE);
        assert!(iter.more());
        let first = iter.value();
        assert!(first.is_some());
        iter.next();
        assert!(iter.more());
    }

    #[test]
    fn test_clight_creation() {
        let light = CLight::new(TypeOfLightSource::Spot);
        assert!(light.is_enabled());
        assert_eq!(light.light_type(), TypeOfLightSource::Spot);
        assert!(!light.to_cast_shadows());
    }

    #[test]
    fn test_clight_setters() {
        let mut light = CLight::new(TypeOfLightSource::Directional);
        light.set_enabled(false);
        assert!(!light.is_enabled());
        light.set_cast_shadows(true);
        assert!(light.to_cast_shadows());
    }

    #[test]
    fn test_type_of_light_source_enum() {
        assert_eq!(TypeOfLightSource::Ambient as u32, 0);
        assert_eq!(TypeOfLightSource::Directional as u32, 1);
        assert_eq!(TypeOfLightSource::Point as u32, 2);
        assert_eq!(TypeOfLightSource::Spot as u32, 3);
    }
}
