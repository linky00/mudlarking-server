use crate::FontSizer;
use corpus::Corpus;
use deterministic_weighted_index::DeterministicWeightedIndex;
use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::SliceRandom,
    thread_rng, Rng,
};
use std::collections::HashMap;

mod corpus;
mod deterministic_weighted_index;

const MIN_WORDS: u32 = 2;
const DEFAULT_MAX_WORDS: u32 = 4;

pub struct Region {
    pub name: String,
    items: Vec<Item>,
    shortest_item: Item,
    weighted_index: WeightedIndex<u32>,
    pot_chance: f32,
}

#[derive(Clone)]
pub struct Item {
    pub text: String,
    pub width: f32,
}

pub struct TextTable<'a> {
    regions: Vec<Region>,
    region_idx_by_name: HashMap<String, usize>,
    weighted_index: DeterministicWeightedIndex,
    split_pots: Vec<Vec<String>>,
    pub font_sizer: FontSizer<'a>,
}

impl<'a> TextTable<'a> {
    pub fn from_json(json: &str, font_sizer: FontSizer<'a>) -> Self {
        TextTable::from_corpus(
            serde_json::from_str(json).expect("JSON should be formatted correctly"),
            font_sizer,
        )
    }

    fn from_corpus(corpus: Corpus, font_sizer: FontSizer<'a>) -> Self {
        let mut region_weights: Vec<u32> = vec![];
        let regions: Vec<Region> = corpus
            .regions
            .iter()
            .map(|region| {
                region_weights.push(region.weight);
                let (items, weights): (Vec<Item>, Vec<u32>) = region
                    .items
                    .iter()
                    .map(|corpus_item| {
                        let text = corpus_item.text.clone();
                        let width = font_sizer.get_width(&corpus_item.text);
                        (Item { text, width }, corpus_item.weight)
                    })
                    .collect();
                let shortest_item = items
                    .iter()
                    .min_by(|a, b| {
                        a.width
                            .partial_cmp(&b.width)
                            .expect("Should be no NaNs in widths")
                    })
                    .expect("There should be more than zero items in corpus")
                    .clone();
                let weighted_index =
                    WeightedIndex::new(weights).expect("The weights should be correct");
                Region {
                    name: region.name.clone(),
                    items,
                    shortest_item,
                    weighted_index,
                    pot_chance: region.pot_chance,
                }
            })
            .collect();
        let region_idx_by_name: HashMap<String, usize> = regions
            .iter()
            .enumerate()
            .map(|(i, region)| (region.name.clone(), i))
            .collect();
        let weighted_index = DeterministicWeightedIndex::new(region_weights);
        let split_pots = corpus
            .pots
            .iter()
            .map(|pot| pot.split(' ').map(|substr| substr.to_string()).collect())
            .collect();

        TextTable {
            regions,
            region_idx_by_name,
            weighted_index,
            split_pots,
            font_sizer,
        }
    }

    pub fn get_item_by_value(&self, region_value: f32, max_width: f32) -> Item {
        let region = self.get_region_by_value(region_value);
        self.get_item(region, max_width)
    }

    pub fn get_region_by_value(&self, region_value: f32) -> &Region {
        &self.regions[self
            .weighted_index
            .sample(region_value)
            .unwrap_or_else(|_| panic!("{region_value} should be between 0 and 1"))]
    }

    pub fn get_item_by_name(&self, region_name: &str, max_width: f32) -> Option<Item> {
        let region = &self.regions[*self.region_idx_by_name.get(region_name)?];
        Some(self.get_item(region, max_width))
    }

    fn get_item(&self, region: &Region, max_width: f32) -> Item {
        if thread_rng().gen::<f32>() < region.pot_chance {
            self.get_sherd(region, max_width, DEFAULT_MAX_WORDS)
        } else {
            self.get_ground(region, max_width)
        }
    }

    fn get_sherd(&self, region: &Region, max_width: f32, max_words: u32) -> Item {
        let pot = self
            .split_pots
            .choose(&mut thread_rng())
            .expect("The list should not be empty");
        let length = thread_rng().gen_range(MIN_WORDS..=DEFAULT_MAX_WORDS) as usize;
        let start = thread_rng().gen_range(0..(pot.len() - length));
        let text = pot[start..(start + length)].join(" ");
        let width = self.font_sizer.get_width(&text.replace(' ', ""));
        if width > max_width {
            if max_words == MIN_WORDS {
                return self.get_ground(region, max_width);
            } else {
                return self.get_sherd(region, max_width, max_words - 1);
            }
        }
        Item { width, text }
    }

    fn get_ground(&self, region: &Region, max_width: f32) -> Item {
        let chosen_item = region.items[region.weighted_index.sample(&mut thread_rng())].clone();
        if chosen_item.width < max_width {
            chosen_item
        } else {
            region.shortest_item.clone()
        }
    }
}
