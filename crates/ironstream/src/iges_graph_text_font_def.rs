// FILE: iges_graph_text_font_def.rs
// occt: IGESGraph_TextFontDef

pub struct IGESGraphTextFontDef {
    font_code: i32,
    font_name: String,
    superseded_font_code: i32,
    superseded_font_entity: Option<Box<IGESGraphTextFontDef>>,
    scale: i32,
    ascii_codes: Vec<i32>,
    next_char_origin_x: Vec<i32>,
    next_char_origin_y: Vec<i32>,
    nb_pen_motions: Vec<i32>,
    pen_motions: Vec<Vec<i32>>,
    pen_moves_to_x: Vec<Vec<i32>>,
    pen_moves_to_y: Vec<Vec<i32>>,
}

impl IGESGraphTextFontDef {
    pub fn new() -> Self {
        IGESGraphTextFontDef {
            font_code: 0,
            font_name: String::new(),
            superseded_font_code: 0,
            superseded_font_entity: None,
            scale: 0,
            ascii_codes: Vec::new(),
            next_char_origin_x: Vec::new(),
            next_char_origin_y: Vec::new(),
            nb_pen_motions: Vec::new(),
            pen_motions: Vec::new(),
            pen_moves_to_x: Vec::new(),
            pen_moves_to_y: Vec::new(),
        }
    }

    pub fn init(
        &mut self,
        a_font_code: i32,
        a_font_name: String,
        a_superseded_font: i32,
        a_superseded_entity: Option<Box<IGESGraphTextFontDef>>,
        a_scale: i32,
        all_ascii_codes: Vec<i32>,
        all_next_char_x: Vec<i32>,
        all_next_char_y: Vec<i32>,
        all_pen_motions: Vec<i32>,
        all_pen_flags: Vec<Vec<i32>>,
        all_move_pen_to_x: Vec<Vec<i32>>,
        all_move_pen_to_y: Vec<Vec<i32>>,
    ) {
        let len = all_ascii_codes.len();
        assert_eq!(
            all_next_char_x.len(),
            len,
            "NextCharX length mismatch"
        );
        assert_eq!(
            all_next_char_y.len(),
            len,
            "NextCharY length mismatch"
        );
        assert_eq!(
            all_pen_motions.len(),
            len,
            "PenMotions length mismatch"
        );
        assert_eq!(
            all_pen_flags.len(),
            len,
            "PenFlags length mismatch"
        );
        assert_eq!(
            all_move_pen_to_x.len(),
            len,
            "MovePenToX length mismatch"
        );
        assert_eq!(
            all_move_pen_to_y.len(),
            len,
            "MovePenToY length mismatch"
        );

        self.font_code = a_font_code;
        self.font_name = a_font_name;
        self.superseded_font_code = a_superseded_font;
        self.superseded_font_entity = a_superseded_entity;
        self.scale = a_scale;
        self.ascii_codes = all_ascii_codes;
        self.next_char_origin_x = all_next_char_x;
        self.next_char_origin_y = all_next_char_y;
        self.nb_pen_motions = all_pen_motions;
        self.pen_motions = all_pen_flags;
        self.pen_moves_to_x = all_move_pen_to_x;
        self.pen_moves_to_y = all_move_pen_to_y;
    }

    pub fn font_code(&self) -> i32 {
        self.font_code
    }

    pub fn font_name(&self) -> &str {
        &self.font_name
    }

    pub fn is_superseded_font_entity(&self) -> bool {
        self.superseded_font_entity.is_some()
    }

    pub fn superseded_font_code(&self) -> i32 {
        self.superseded_font_code
    }

    pub fn superseded_font_entity(&self) -> Option<&IGESGraphTextFontDef> {
        self.superseded_font_entity.as_ref().map(|e| &**e)
    }

    pub fn scale(&self) -> i32 {
        self.scale
    }

    pub fn nb_characters(&self) -> usize {
        self.ascii_codes.len()
    }

    pub fn ascii_code(&self, chnum: usize) -> Option<i32> {
        if chnum > 0 && chnum <= self.ascii_codes.len() {
            Some(self.ascii_codes[chnum - 1])
        } else {
            None
        }
    }

    pub fn next_char_origin(&self, chnum: usize) -> Option<(i32, i32)> {
        if chnum > 0 && chnum <= self.next_char_origin_x.len() {
            Some((
                self.next_char_origin_x[chnum - 1],
                self.next_char_origin_y[chnum - 1],
            ))
        } else {
            None
        }
    }

    pub fn nb_pen_motions(&self, chnum: usize) -> Option<i32> {
        if chnum > 0 && chnum <= self.nb_pen_motions.len() {
            Some(self.nb_pen_motions[chnum - 1])
        } else {
            None
        }
    }

    pub fn is_pen_up(&self, chnum: usize, motionnum: usize) -> Option<bool> {
        if chnum > 0 && chnum <= self.pen_motions.len() {
            if motionnum > 0 && motionnum <= self.pen_motions[chnum - 1].len() {
                return Some(self.pen_motions[chnum - 1][motionnum - 1] == 1);
            }
        }
        None
    }

    pub fn next_pen_position(&self, chnum: usize, motionnum: usize) -> Option<(i32, i32)> {
        if chnum > 0 && chnum <= self.pen_moves_to_x.len() {
            if motionnum > 0 && motionnum <= self.pen_moves_to_x[chnum - 1].len() {
                return Some((
                    self.pen_moves_to_x[chnum - 1][motionnum - 1],
                    self.pen_moves_to_y[chnum - 1][motionnum - 1],
                ));
            }
        }
        None
    }
}

impl Default for IGESGraphTextFontDef {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let font_def = IGESGraphTextFontDef::new();
        assert_eq!(font_def.font_code(), 0);
        assert_eq!(font_def.font_name(), "");
        assert!(!font_def.is_superseded_font_entity());
        assert_eq!(font_def.nb_characters(), 0);
    }

    #[test]
    fn test_init() {
        let mut font_def = IGESGraphTextFontDef::new();
        let ascii_codes = vec![65, 66, 67];
        let next_char_x = vec![10, 20, 30];
        let next_char_y = vec![0, 0, 0];
        let pen_motions = vec![2, 2, 2];
        let pen_flags = vec![vec![0, 1], vec![0, 1], vec![0, 1]];
        let pen_x = vec![vec![5, 10], vec![15, 20], vec![25, 30]];
        let pen_y = vec![vec![0, 5], vec![0, 5], vec![0, 5]];

        font_def.init(
            1,
            "TestFont".to_string(),
            0,
            None,
            10,
            ascii_codes,
            next_char_x,
            next_char_y,
            pen_motions,
            pen_flags,
            pen_x,
            pen_y,
        );

        assert_eq!(font_def.font_code(), 1);
        assert_eq!(font_def.font_name(), "TestFont");
        assert_eq!(font_def.nb_characters(), 3);
        assert_eq!(font_def.ascii_code(1), Some(65));
        assert_eq!(font_def.ascii_code(2), Some(66));
        assert_eq!(font_def.scale(), 10);
    }

    #[test]
    fn test_next_char_origin() {
        let mut font_def = IGESGraphTextFontDef::new();
        let ascii_codes = vec![65];
        let next_char_x = vec![25];
        let next_char_y = vec![35];
        let pen_motions = vec![1];
        let pen_flags = vec![vec![0]];
        let pen_x = vec![vec![5]];
        let pen_y = vec![vec![10]];

        font_def.init(
            1,
            "F".to_string(),
            0,
            None,
            10,
            ascii_codes,
            next_char_x,
            next_char_y,
            pen_motions,
            pen_flags,
            pen_x,
            pen_y,
        );

        assert_eq!(font_def.next_char_origin(1), Some((25, 35)));
    }

    #[test]
    fn test_is_pen_up() {
        let mut font_def = IGESGraphTextFontDef::new();
        let ascii_codes = vec![65];
        let next_char_x = vec![10];
        let next_char_y = vec![0];
        let pen_motions = vec![2];
        let pen_flags = vec![vec![0, 1]];
        let pen_x = vec![vec![5, 10]];
        let pen_y = vec![vec![0, 5]];

        font_def.init(
            1,
            "F".to_string(),
            0,
            None,
            10,
            ascii_codes,
            next_char_x,
            next_char_y,
            pen_motions,
            pen_flags,
            pen_x,
            pen_y,
        );

        assert_eq!(font_def.is_pen_up(1, 1), Some(false));
        assert_eq!(font_def.is_pen_up(1, 2), Some(true));
    }
}
