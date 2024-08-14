use crate::TextTable;
use noise::{
    core::worley::{distance_functions::chebyshev, ReturnType},
    Fbm, NoiseFn, Perlin, Worley,
};
use rand::random;
use serde::Serialize;
use uuid::Uuid;

const WORLEY_FREQ: f64 = 0.007;
const WAVE_WIDTH: f32 = 80.0;
const PERLIN_COEF: f64 = 0.13;

type Contents = Vec<Item>;

fn generate_contents(
    max_width: f32,
    max_height: f32,
    text_table: &TextTable,
    debug: bool,
) -> (Contents, Option<Contents>) {
    let mut contents = Contents::new();
    let mut debug_contents = if debug { Some(Contents::new()) } else { None };
    let worley = Worley::new(random())
        .set_distance_function(chebyshev)
        .set_return_type(ReturnType::Value)
        .set_frequency(WORLEY_FREQ);
    let perlin = Fbm::<Perlin>::new(random());
    let lines = (max_height / text_table.font_sizer.get_height()) as u32;
    for n in 0..(lines) {
        let mut x = (perlin.get([n as f64 * PERLIN_COEF, 0.0]) as f32 * 0.5 + 0.5) * WAVE_WIDTH;
        let y = n as f32 * text_table.font_sizer.get_height();
        loop {
            let value = worley.get([x.into(), y.into()]) as f32;
            let value = value * 0.5 + 0.5;
            let text_table_item = text_table.get_item(value, max_width - x);
            let new_width = text_table_item.width;
            if new_width + x > max_width {
                break;
            }
            contents.push(Item::new(text_table_item.text, n, x));
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
    pub fn new(width: f32, height: f32, text_table: &TextTable) -> Self {
        Shore {
            contents: generate_contents(width, height, text_table, false).0,
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
    pub fn new(width: f32, height: f32, text_table: &TextTable) -> Self {
        let contents_with_debug = generate_contents(width, height, text_table, true);
        DebugShore {
            contents: contents_with_debug.0,
            debug_contents: contents_with_debug.1,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Item {
    text: String,
    line: u32,
    offset: f32,
    id: Uuid,
}

impl Item {
    pub fn new(text: String, line: u32, offset: f32) -> Item {
        Item {
            text,
            line,
            offset,
            id: Uuid::new_v4(),
        }
    }
}
