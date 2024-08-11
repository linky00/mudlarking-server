use crate::{FontSizer, TextTable};
use noise::{
    core::worley::{distance_functions::chebyshev, ReturnType},
    NoiseFn, Worley,
};
use rand::random;
use serde::Serialize;
use uuid::Uuid;

const WORLEY_FREQ: f64 = 0.007;

type Contents = Vec<Item>;

fn generate_contents(
    max_width: f32,
    max_height: f32,
    text_table: &TextTable,
    font_sizer: &FontSizer,
    debug: bool,
) -> (Contents, Option<Contents>) {
    let mut contents = Contents::new();
    let mut debug_contents = if debug { Some(Contents::new()) } else { None };
    let worley = Worley::new(random())
        .set_distance_function(chebyshev)
        .set_return_type(ReturnType::Value)
        .set_frequency(WORLEY_FREQ);
    let lines = (max_height / font_sizer.get_height()) as u16;
    for n in 0..(lines) {
        let mut x = 0.0;
        let y = n as f32 * font_sizer.get_height();
        loop {
            let value = worley.get([x.into(), y.into()]);
            let value = value * 0.5 + 0.5;
            let text = text_table.get_text(value);
            let squashed_text = text.replace(' ', "");
            let new_width = font_sizer.get_width(&squashed_text);
            if new_width + x > max_width {
                break;
            }
            contents.push(Item::new(text, n, x));
            if let Some(debug_contents) = &mut debug_contents {
                debug_contents.push(Item::new(text_table.get_region(value).name.clone(), n, x));
            }
            x += new_width;
        }
    }

    (contents, debug_contents)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shore {
    contents: Contents,
}

impl Shore {
    pub fn new(width: f32, height: f32, text_table: &TextTable, font_sizer: &FontSizer) -> Self {
        Shore {
            contents: generate_contents(width, height, text_table, font_sizer, false).0,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugShore {
    contents: Contents,
    debug_contents: Option<Contents>,
}

impl DebugShore {
    pub fn new(width: f32, height: f32, text_table: &TextTable, font_sizer: &FontSizer) -> Self {
        let contents_with_debug = generate_contents(width, height, text_table, font_sizer, true);
        DebugShore {
            contents: contents_with_debug.0,
            debug_contents: contents_with_debug.1,
        }
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
