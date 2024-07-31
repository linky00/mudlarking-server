use std::fmt;
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
            while current_width < width {
                let text = text_table.get_text();
                let new_width = font_sizer.get_width(&text, 16.0);
                contents.push(Item::new(text, n, current_width));
                current_width += new_width;
            };
        }

        contents
    }
}

// impl fmt::Display for Shore {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut out = String::new();
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 out = format!("{} {}", out, self.get(x, y).text);
//             }
//             out += "\n";
//         }
//         write!(f, "{out}")
//     }
// }

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