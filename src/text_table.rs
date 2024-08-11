use corpus::Corpus;
use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::SliceRandom,
    thread_rng, Rng,
};
use weighted_choice::WeightedChoice;

mod corpus;
mod weighted_choice;

const MIN_WORDS: u32 = 2;
const MAX_WORDS: u32 = 4;

pub struct TextTable {
    regions: Vec<Region>,
    weighted_choice: WeightedChoice,
    split_pots: Vec<Vec<String>>,
}

pub struct Region {
    name: String,
    possibilities: Vec<TextPossibility>,
    weighted_index: WeightedIndex<u32>,
}

enum TextPossibility {
    Waste(String),
    Pot,
}

impl TextTable {
    pub fn from_json(json: &str) -> Self {
        TextTable::from_corpus(
            serde_json::from_str(json).expect("JSON should be formatted correctly"),
        )
    }

    fn from_corpus(corpus: Corpus) -> Self {
        let mut region_weights: Vec<u32> = vec![];
        let regions: Vec<Region> = corpus
            .regions
            .iter()
            .map(|region| {
                region_weights.push(region.weight);
                let (mut possibilities, mut weights): (Vec<TextPossibility>, Vec<u32>) = region
                    .items
                    .iter()
                    .map(|waste| (TextPossibility::Waste(waste.text.clone()), waste.weight))
                    .collect();
                possibilities.push(TextPossibility::Pot);
                weights.push(1);
                let weighted_index =
                    WeightedIndex::new(weights).expect("The weights should be correct");
                Region {
                    name: region.name.clone(),
                    possibilities,
                    weighted_index,
                }
            })
            .collect();
        let weighted_choice = WeightedChoice::new(region_weights);

        TextTable {
            regions,
            weighted_choice,
            split_pots: corpus
                .pots
                .iter()
                .map(|pot| pot.split(' ').map(|substr| substr.to_string()).collect())
                .collect(),
        }
    }

    pub fn get_text(&self, region_at: f64) -> String {
        let mut rng = thread_rng();
        let region = &self.regions[self
            .weighted_choice
            .get(region_at)
            .expect("region_at should be between 0 and 1")];
        match &region.possibilities[region.weighted_index.sample(&mut rng)] {
            TextPossibility::Waste(text) => text.to_string(),
            TextPossibility::Pot => {
                let pot = self
                    .split_pots
                    .choose(&mut rng)
                    .expect("The list should not be empty");
                let length = rng.gen_range(MIN_WORDS..=MAX_WORDS) as usize;
                let start = rng.gen_range(0..(pot.len() - length));
                pot[start..(start + length)].join(" ")
            }
        }
    }
}
