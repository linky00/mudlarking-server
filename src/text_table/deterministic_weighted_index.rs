pub struct DeterministicWeightedIndex {
    cumulative_weights: Vec<f32>,
}

impl DeterministicWeightedIndex {
    pub fn new(weights: Vec<u32>) -> Self {
        let max_weight: u32 = weights.iter().sum();
        let mut cumulative_weights = vec![];
        let mut acc = 0.0;
        for weight in weights {
            let weight = weight as f32 / max_weight as f32;
            acc += weight;
            cumulative_weights.push(acc);
        }
        DeterministicWeightedIndex { cumulative_weights }
    }

    pub fn sample(&self, at: f32) -> Result<usize, DeterministicWeightedIndexError> {
        for (i, weight) in self.cumulative_weights.iter().enumerate() {
            if at <= *weight {
                return Ok(i);
            }
        }
        Err(DeterministicWeightedIndexError {})
    }
}

#[derive(Debug)]
pub struct DeterministicWeightedIndexError {}
