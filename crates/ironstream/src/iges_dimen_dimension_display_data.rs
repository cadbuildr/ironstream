// FILE: iges_dimen_dimension_display_data.rs
// occt: IGESDimen_DimensionDisplayData

/// Defines IGES Dimension Display Data, Type <406> Form <30>
/// in package IGESDimen
pub struct IgesDimen_DimensionDisplayData {
    nb_property_values: i32,
    dimension_type: i32,
    label_position: i32,
    character_set: i32,
    l_string: String,
    decimal_symbol: i32,
    witness_line_angle: f64,
    text_alignment: i32,
    text_level: i32,
    text_placement: i32,
    arrow_head_orientation: i32,
    initial_value: f64,
    supplementary_notes: Vec<i32>,
    start_index: Vec<i32>,
    end_index: Vec<i32>,
}

impl IgesDimen_DimensionDisplayData {
    /// Create a new DimensionDisplayData entity
    pub fn new() -> Self {
        IgesDimen_DimensionDisplayData {
            nb_property_values: 0,
            dimension_type: 0,
            label_position: 0,
            character_set: 0,
            l_string: String::new(),
            decimal_symbol: 0,
            witness_line_angle: 0.0,
            text_alignment: 0,
            text_level: 0,
            text_placement: 0,
            arrow_head_orientation: 0,
            initial_value: 0.0,
            supplementary_notes: Vec::new(),
            start_index: Vec::new(),
            end_index: Vec::new(),
        }
    }

    pub fn init(
        &mut self,
        num_props: i32,
        a_dim_type: i32,
        a_label_pos: i32,
        a_char_set: i32,
        a_string: String,
        a_symbol: i32,
        an_ang: f64,
        an_align: i32,
        a_level: i32,
        a_place: i32,
        an_orient: i32,
        init_val: f64,
        notes: Vec<i32>,
        start_ind: Vec<i32>,
        end_ind: Vec<i32>,
    ) {
        self.nb_property_values = num_props;
        self.dimension_type = a_dim_type;
        self.label_position = a_label_pos;
        self.character_set = a_char_set;
        self.l_string = a_string;
        self.decimal_symbol = a_symbol;
        self.witness_line_angle = an_ang;
        self.text_alignment = an_align;
        self.text_level = a_level;
        self.text_placement = a_place;
        self.arrow_head_orientation = an_orient;
        self.initial_value = init_val;
        self.supplementary_notes = notes;
        self.start_index = start_ind;
        self.end_index = end_ind;
    }

    /// Returns the number of property values (14)
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the dimension type
    pub fn dimension_type(&self) -> i32 {
        self.dimension_type
    }

    /// Returns the preferred label position
    pub fn label_position(&self) -> i32 {
        self.label_position
    }

    /// Returns the character set interpretation
    pub fn character_set(&self) -> i32 {
        self.character_set
    }

    /// Returns the label string
    pub fn l_string(&self) -> &str {
        &self.l_string
    }

    pub fn decimal_symbol(&self) -> i32 {
        self.decimal_symbol
    }

    /// Returns the witness line angle in radians
    pub fn witness_line_angle(&self) -> f64 {
        self.witness_line_angle
    }

    /// Returns the text alignment
    pub fn text_alignment(&self) -> i32 {
        self.text_alignment
    }

    /// Returns the text level
    pub fn text_level(&self) -> i32 {
        self.text_level
    }

    /// Returns the preferred text placement
    pub fn text_placement(&self) -> i32 {
        self.text_placement
    }

    /// Returns the arrowhead orientation
    pub fn arrow_head_orientation(&self) -> i32 {
        self.arrow_head_orientation
    }

    /// Returns the primary dimension initial value
    pub fn initial_value(&self) -> f64 {
        self.initial_value
    }

    /// Returns the number of supplementary notes or zero
    pub fn nb_supplementary_notes(&self) -> usize {
        self.supplementary_notes.len()
    }

    /// Returns the Index'th supplementary note
    /// raises exception if index <= 0 or index > nb_supplementary_notes()
    pub fn supplementary_note(&self, index: usize) -> i32 {
        if index == 0 || index > self.supplementary_notes.len() {
            panic!("Index out of bounds");
        }
        self.supplementary_notes[index - 1]
    }

    /// Returns the Index'th note start index
    /// raises exception if index <= 0 or index > nb_supplementary_notes()
    pub fn start_index(&self, index: usize) -> i32 {
        if index == 0 || index > self.start_index.len() {
            panic!("Index out of bounds");
        }
        self.start_index[index - 1]
    }

    /// Returns the Index'th note end index
    /// raises exception if index <= 0 or index > nb_supplementary_notes()
    pub fn end_index(&self, index: usize) -> i32 {
        if index == 0 || index > self.end_index.len() {
            panic!("Index out of bounds");
        }
        self.end_index[index - 1]
    }
}

impl Default for IgesDimen_DimensionDisplayData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_display_data_creation() {
        let data = IgesDimen_DimensionDisplayData::new();
        assert_eq!(data.nb_property_values(), 0);
        assert_eq!(data.dimension_type(), 0);
    }

    #[test]
    fn test_dimension_display_data_init() {
        let mut data = IgesDimen_DimensionDisplayData::new();
        data.init(
            14,
            1,
            2,
            3,
            "DIAMETER".to_string(),
            4,
            1.57,
            5,
            6,
            7,
            8,
            25.4,
            vec![1, 2],
            vec![0, 5],
            vec![3, 8],
        );

        assert_eq!(data.nb_property_values(), 14);
        assert_eq!(data.dimension_type(), 1);
        assert_eq!(data.l_string(), "DIAMETER");
        assert_eq!(data.witness_line_angle(), 1.57);
        assert_eq!(data.nb_supplementary_notes(), 2);
    }
}
