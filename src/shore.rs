use serde::Serialize;
use uuid::Uuid;
use crate::TextTable;
use crate::FontSizer;

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

    fn generate_contents(width: f32, lines: u16, text_table: &TextTable, font_sizer: &FontSizer) -> Vec<Item> {
        let mut contents = Vec::new();

        for n in 0..(lines) {
            let mut current_width = 0.0;
            loop {
                let text = text_table.get_text();
                let text_squashed = text.replace(' ', "");
                let new_width = font_sizer.get_width(&text_squashed, 16.0);
                if new_width + current_width > width {
                    break;
                }
                contents.push(Item::new(text, n, current_width));
                current_width += new_width;
            };
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