use crate::FontSizer;
use crate::TextTable;
use noise::{Fbm, NoiseFn, Perlin};
use rand;
use serde::Serialize;
use uuid::Uuid;

const WAVE_WIDTH: f32 = 80.0;
const PERLIN_COEF: f64 = 0.1;

#[derive(Serialize)]
pub struct Shore {
    contents: Vec<Item>,
}

impl Shore {
    pub fn new(width: f32, lines: u16, text_table: &TextTable, font_sizer: &FontSizer) -> Shore {
        Shore {
            contents: Self::generate_contents(width, lines, text_table, font_sizer),
        }
    }

    fn generate_contents(
        max_width: f32,
        lines: u16,
        text_table: &TextTable,
        font_sizer: &FontSizer,
    ) -> Vec<Item> {
        let mut contents = Vec::new();
        let noise = Fbm::<Perlin>::new(rand::random());
        for n in 0..(lines) {
            let mut total_width =
                (noise.get([n as f64 * PERLIN_COEF, 0.0]) as f32 * 0.5 + 0.5) * WAVE_WIDTH;
            loop {
                let text = text_table.get_text();
                let text_squashed = text.replace(' ', "");
                let new_width = font_sizer.get_width(&text_squashed, 16.0);
                if new_width + total_width > max_width {
                    break;
                }
                contents.push(Item::new(text, n, total_width));
                total_width += new_width;
            }
        }

        contents
    }
}

#[derive(Clone, Serialize)]
pub struct Item {
    text: String,
    line: u16,
    offset: f32,
    id: Uuid,
}

impl Item {
    pub fn new(text: String, line: u16, offset: f32) -> Item {
        Item {
            text,
            line,
            offset,
            id: Uuid::new_v4(),
        }
    }
}
