use std::fmt;
use serde::Serialize;
use uuid::Uuid;
use crate::TextTable;

#[derive(Serialize)]
pub struct Shore {
    width: u16,
    height: u16,
    contents: Vec<Item>,
}

impl Shore {
    pub fn new(width: u16, height: u16, text_table: TextTable) -> Shore {
        Shore {
            width,
            height,
            contents: Self::generate_contents(width, height, text_table),
        }
    }

    fn generate_contents(width: u16, height: u16, text_table: TextTable) -> Vec<Item> {
        let mut contents = Vec::new();

        for _ in 0..(width * height) {
            let text = text_table.get_text();
            contents.push(Item::new(text));
        }

        contents
    }

    fn get(&self, x: u16, y: u16) -> &Item {
        &self.contents[(self.width * y + x) as usize]
    }
}

impl fmt::Display for Shore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                out = format!("{} {}", out, self.get(x, y).text);
            }
            out += "\n";
        }
        write!(f, "{out}")
    }
}

#[derive(Clone, Serialize)]
struct Item {
    text: String,
    id: Uuid,
}

impl Item {
    pub fn new(text: String) -> Item {
        Item { 
            text,
            id: Uuid::new_v4(),
        }
    }
}