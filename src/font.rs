use ttf_parser::Face;

pub struct FontSizer<'a> {
    face: Face<'a>
}

impl FontSizer<'_> {
    pub fn new(data: &[u8], index: u32) -> FontSizer {
        let face = Face::parse(data, index).unwrap();
        FontSizer { face }
    }

    pub fn get_width(&self, phrase: &str, font_size: f32) -> f32 {
        let units_per_em = self.face.units_per_em() as f32;
        let scale_factor = font_size / units_per_em;

        phrase.chars()
            .filter_map(|char| {
                self.face.glyph_index(char)
                    .and_then(|glyph_id| self.face.glyph_hor_advance(glyph_id))
            })
            .map(|advance| advance as f32 * scale_factor)
            .sum()
    }
}