// FILE: iges_appli_drilled_hole.rs
// occt: IGESAppli_DrilledHole

/// Represents a drilled hole through a printed circuit board (PCB).
///
/// IGES Type 406 Form 6 entity.
/// Stores drilled hole specifications including:
/// - Drill diameter size
/// - Finish diameter size
/// - Plating indication
/// - Layer information
#[derive(Clone, Debug)]
pub struct IgesAppliDrilledHole {
    nb_property_values: i32,
    drill_dia_size: f64,
    finish_dia_size: f64,
    plating_flag: i32,
    nb_lower_layer: i32,
    nb_higher_layer: i32,
}

impl IgesAppliDrilledHole {
    /// Creates a new DrilledHole entity with default values.
    pub fn new() -> Self {
        Self {
            nb_property_values: 5,
            drill_dia_size: 0.0,
            finish_dia_size: 0.0,
            plating_flag: 0,
            nb_lower_layer: 0,
            nb_higher_layer: 0,
        }
    }

    /// Initializes the DrilledHole entity with specifications.
    ///
    /// # Arguments
    /// * `nb_prop_val` - Number of property values (always 5)
    /// * `a_size` - Drill diameter size
    /// * `another_size` - Finish diameter size
    /// * `a_plating` - Plating indication (0=not plating, 1=is plating)
    /// * `a_layer` - Lower numbered layer
    /// * `another_layer` - Higher numbered layer
    pub fn init(
        &mut self,
        nb_prop_val: i32,
        a_size: f64,
        another_size: f64,
        a_plating: i32,
        a_layer: i32,
        another_layer: i32,
    ) {
        self.nb_property_values = nb_prop_val;
        self.drill_dia_size = a_size;
        self.finish_dia_size = another_size;
        self.plating_flag = a_plating;
        self.nb_lower_layer = a_layer;
        self.nb_higher_layer = another_layer;
    }

    /// Returns the number of property values (always 5).
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the drill diameter size.
    pub fn drill_dia_size(&self) -> f64 {
        self.drill_dia_size
    }

    /// Returns the finish diameter size.
    pub fn finish_dia_size(&self) -> f64 {
        self.finish_dia_size
    }

    /// Returns whether plating is present.
    /// false = not plating, true = is plating
    pub fn is_plating(&self) -> bool {
        self.plating_flag != 0
    }

    /// Returns the lower numbered layer.
    pub fn nb_lower_layer(&self) -> i32 {
        self.nb_lower_layer
    }

    /// Returns the higher numbered layer.
    pub fn nb_higher_layer(&self) -> i32 {
        self.nb_higher_layer
    }
}

impl Default for IgesAppliDrilledHole {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hole = IgesAppliDrilledHole::new();
        assert_eq!(hole.nb_property_values(), 5);
        assert_eq!(hole.drill_dia_size(), 0.0);
        assert_eq!(hole.finish_dia_size(), 0.0);
        assert!(!hole.is_plating());
        assert_eq!(hole.nb_lower_layer(), 0);
        assert_eq!(hole.nb_higher_layer(), 0);
    }

    #[test]
    fn test_init() {
        let mut hole = IgesAppliDrilledHole::new();
        hole.init(5, 2.5, 3.0, 1, 1, 2);

        assert_eq!(hole.nb_property_values(), 5);
        assert_eq!(hole.drill_dia_size(), 2.5);
        assert_eq!(hole.finish_dia_size(), 3.0);
        assert!(hole.is_plating());
        assert_eq!(hole.nb_lower_layer(), 1);
        assert_eq!(hole.nb_higher_layer(), 2);
    }

    #[test]
    fn test_init_no_plating() {
        let mut hole = IgesAppliDrilledHole::new();
        hole.init(5, 1.5, 2.0, 0, 3, 4);

        assert_eq!(hole.drill_dia_size(), 1.5);
        assert_eq!(hole.finish_dia_size(), 2.0);
        assert!(!hole.is_plating());
        assert_eq!(hole.nb_lower_layer(), 3);
        assert_eq!(hole.nb_higher_layer(), 4);
    }

    #[test]
    fn test_clone() {
        let mut hole1 = IgesAppliDrilledHole::new();
        hole1.init(5, 2.0, 2.5, 1, 0, 1);

        let hole2 = hole1.clone();
        assert_eq!(hole2.drill_dia_size(), 2.0);
        assert_eq!(hole2.finish_dia_size(), 2.5);
        assert!(hole2.is_plating());
    }
}
