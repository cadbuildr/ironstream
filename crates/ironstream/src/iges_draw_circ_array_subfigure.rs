// FILE: iges_draw_circ_array_subfigure.rs
// occt: IGESDraw_CircArraySubfigure

/// Defines IGES Circular Array Subfigure Instance Entity,
/// Type <414> Form Number <0> in package IGESDraw
///
/// Used to produce copies of object called the base entity,
/// arranging them around the edge of an imaginary circle
/// whose center and radius are specified
pub struct IgesDrawCircArraySubfigure {
    base_entity: Option<Box<dyn std::any::Any>>,
    nb_locations: i32,
    center: (f64, f64, f64),
    radius: f64,
    start_angle: f64,
    delta_angle: f64,
    do_dont_flag: bool,
    positions: Vec<i32>,
}

impl IgesDrawCircArraySubfigure {
    /// Create a new CircArraySubfigure
    pub fn new() -> Self {
        IgesDrawCircArraySubfigure {
            base_entity: None,
            nb_locations: 0,
            center: (0.0, 0.0, 0.0),
            radius: 0.0,
            start_angle: 0.0,
            delta_angle: 0.0,
            do_dont_flag: false,
            positions: Vec::new(),
        }
    }

    /// This method is used to set the fields of the class CircArraySubfigure
    /// - base_entity   : Base entity
    /// - nb_locs       : Total number of possible instance locations
    /// - center        : Coordinates of Center of imaginary circle
    /// - radius        : Radius of imaginary circle
    /// - st_angle      : Start angle in radians
    /// - del_angle     : Delta angle in radians
    /// - flag          : DO-DON'T flag to control which portion to display
    /// - all_num_pos   : All position to be or not to be processed
    pub fn init(
        &mut self,
        nb_locs: i32,
        center: (f64, f64, f64),
        radius: f64,
        st_angle: f64,
        del_angle: f64,
        flag: bool,
        all_num_pos: Vec<i32>,
    ) {
        self.nb_locations = nb_locs;
        self.center = center;
        self.radius = radius;
        self.start_angle = st_angle;
        self.delta_angle = del_angle;
        self.do_dont_flag = flag;
        self.positions = all_num_pos;
    }

    /// returns the base entity, copies of which are produced
    pub fn base_entity(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.base_entity
    }

    /// returns total number of possible instance locations
    pub fn nb_locations(&self) -> i32 {
        self.nb_locations
    }

    /// returns the center of the imaginary circle
    pub fn center_point(&self) -> (f64, f64, f64) {
        self.center
    }

    /// returns the Transformed center of the imaginary circle
    pub fn transformed_center_point(&self) -> (f64, f64, f64) {
        self.center
    }

    /// returns the radius of the imaginary circle
    pub fn circle_radius(&self) -> f64 {
        self.radius
    }

    /// returns the start angle in radians
    pub fn start_angle(&self) -> f64 {
        self.start_angle
    }

    /// returns the delta angle in radians
    pub fn delta_angle(&self) -> f64 {
        self.delta_angle
    }

    /// returns 0 if all elements to be displayed
    pub fn list_count(&self) -> i32 {
        self.positions.len() as i32
    }

    /// returns True if (list_count = 0) all elements are to be displayed
    pub fn display_flag(&self) -> bool {
        self.list_count() == 0
    }

    /// returns 0 if half or fewer of the elements of the array are defined
    /// returns 1 if half or more of the elements are defined
    pub fn do_dont_flag(&self) -> bool {
        self.do_dont_flag
    }

    /// returns whether Index is to be processed (DO)
    /// or not to be processed(DON'T)
    /// if list_count = 0 return the_do_dont_flag
    pub fn position_num(&self, index: i32) -> bool {
        if self.list_count() == 0 {
            return self.do_dont_flag;
        }
        if index <= 0 || index > self.list_count() {
            panic!("Index out of bounds");
        }
        self.positions[(index - 1) as usize] != 0
    }

    /// returns the Index'th value position
    pub fn list_position(&self, index: i32) -> i32 {
        if index <= 0 || index > self.list_count() {
            panic!("Index out of bounds");
        }
        self.positions[(index - 1) as usize]
    }
}

impl Default for IgesDrawCircArraySubfigure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let circ = IgesDrawCircArraySubfigure::new();
        assert_eq!(circ.nb_locations(), 0);
        assert_eq!(circ.circle_radius(), 0.0);
        assert_eq!(circ.list_count(), 0);
        assert!(circ.display_flag());
    }

    #[test]
    fn test_init() {
        let mut circ = IgesDrawCircArraySubfigure::new();
        let positions = vec![1, 0, 1, 0];
        circ.init(8, (1.0, 2.0, 3.0), 5.5, 0.0, std::f64::consts::PI, true, positions);

        assert_eq!(circ.nb_locations(), 8);
        assert_eq!(circ.circle_radius(), 5.5);
        assert_eq!(circ.center_point(), (1.0, 2.0, 3.0));
        assert_eq!(circ.start_angle(), 0.0);
        assert!(circ.do_dont_flag());
    }

    #[test]
    fn test_list_operations() {
        let mut circ = IgesDrawCircArraySubfigure::new();
        let positions = vec![1, 0, 1];
        circ.init(3, (0.0, 0.0, 0.0), 1.0, 0.0, 1.0, false, positions);

        assert_eq!(circ.list_count(), 3);
        assert!(!circ.display_flag());
        assert_eq!(circ.list_position(1), 1);
        assert_eq!(circ.list_position(2), 0);
        assert_eq!(circ.list_position(3), 1);
    }

    #[test]
    fn test_position_num() {
        let mut circ = IgesDrawCircArraySubfigure::new();
        let positions = vec![1, 0, 1];
        circ.init(3, (0.0, 0.0, 0.0), 1.0, 0.0, 1.0, false, positions);

        assert!(circ.position_num(1));
        assert!(!circ.position_num(2));
        assert!(circ.position_num(3));
    }

    #[test]
    fn test_empty_list_display_flag() {
        let circ = IgesDrawCircArraySubfigure::new();
        assert!(circ.display_flag());
    }
}
