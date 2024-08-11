use crate::{FontSizer, TextTable};
use noise::{
    core::worley::{distance_functions::chebyshev, ReturnType},
    NoiseFn, Worley,
};
use rand::random;
use serde::Serialize;
use uuid::Uuid;

const WORLEY_FREQ: f64 = 0.007;

#[derive(Serialize)]
pub struct Shore {
    contents: Vec<Item>,
}

impl Shore {
    pub fn new(width: f32, height: f32, text_table: &TextTable, font_sizer: &FontSizer) -> Shore {
        Shore {
            contents: Self::generate_contents(width, height, text_table, font_sizer),
        }
    }

    fn generate_contents(
        max_width: f32,
        max_height: f32,
        text_table: &TextTable,
        font_sizer: &FontSizer,
    ) -> Vec<Item> {
        let worley = Worley::new(random())
            .set_distance_function(chebyshev)
            .set_return_type(ReturnType::Value)
            .set_frequency(WORLEY_FREQ);

        let mut contents = Vec::new();
        let lines = (max_height / font_sizer.get_height()) as u16;
        for n in 0..(lines) {
            let mut x = 0.0;
            let y = n as f32 * font_sizer.get_height();
            loop {
                let value = worley.get([x.into(), y.into()]);
                let value = value * 0.5 + 0.5;
                let text = text_table.get_text(value);
                let text = text.replace(' ', "");
                let new_width = font_sizer.get_width(&text);
                if new_width + x > max_width {
                    break;
                }
                contents.push(Item::new(text, n, x));
                x += new_width;
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
