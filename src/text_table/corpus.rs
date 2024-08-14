use serde::Deserialize;

#[derive(Deserialize)]
pub struct Corpus {
    pub regions: Vec<Region>,
    pub pots: Vec<String>,
}

#[derive(Deserialize)]
pub struct Region {
    pub name: String,
    pub weight: u32,
    pub items: Vec<Item>,
    pub pot_chance: f32,
}

#[derive(Deserialize)]
pub struct Item {
    pub text: String,
    pub weight: u32,
}
