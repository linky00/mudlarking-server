use rand::{distributions::{Distribution, WeightedIndex}, seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};

const MIN_WORDS: i32 = 2;
const MAX_WORDS: i32 = 5;

#[derive(Serialize, Deserialize)]
struct Corpus {
    waste: Vec<CorpusWaste>,
    pots: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct CorpusWaste {
    text: String,
    weight: i32,
}

#[derive(Clone)]
enum TextPossibility {
    Waste(String),
    Pot
}

#[derive(Clone)]
pub struct TextTable {
    possibilities: Vec<TextPossibility>,
    weights: Vec<i32>,
    split_pots: Vec<Vec<String>>,
}

impl TextTable {
    pub fn from_json(json: &str) -> Self {
        TextTable::from_corpus(serde_json::from_str(json).expect("JSON is formatted correctly"))
    }

    fn from_corpus(corpus: Corpus) -> Self {
        let (mut possibilities, mut weights): (Vec<TextPossibility>, Vec<i32>) = corpus.waste.iter().map(
            |waste| (TextPossibility::Waste(waste.text.clone()), waste.weight)
        ).collect();
        possibilities.push(TextPossibility::Pot);
        weights.push(1);

        TextTable {
            possibilities,
            weights,
            split_pots: corpus.pots.iter().map(|pot| pot.split(' ').map(|substr| substr.to_string()).collect()).collect(),
        }
    }

    pub fn get_text(&self) -> String {
        let dist = WeightedIndex::new(self.weights.clone()).expect("The weights are correct");
        let mut rng = thread_rng();

        match &self.possibilities[dist.sample(&mut rng)] {
            TextPossibility::Waste(text) => text.to_string(),
            TextPossibility::Pot => {
                let pot = self.split_pots.choose(&mut rng).expect("The list is not empty");
                let length = rng.gen_range(MIN_WORDS..=MAX_WORDS) as usize;
                let start = rng.gen_range(0..(pot.len() - length));
                pot[start..(start + length)].join(" ")
            }
        }
    }
}