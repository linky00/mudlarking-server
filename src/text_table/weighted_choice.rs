pub struct WeightedChoice {
    cumulative_weights: Vec<f64>,
}

impl WeightedChoice {
    pub fn new(weights: Vec<u32>) -> Self {
        let max_weight: u32 = weights.iter().sum();
        let mut cumulative_weights = vec![];
        let mut acc = 0.0;
        for weight in weights {
            let weight = weight as f64 / max_weight as f64;
            acc += weight;
            cumulative_weights.push(acc);
        }
        WeightedChoice { cumulative_weights }
    }

    pub fn get(&self, at: f64) -> Result<usize, WeightedChoiceError> {
        for (i, weight) in self.cumulative_weights.iter().enumerate() {
            if at <= *weight {
                return Ok(i);
            }
        }
        Err(WeightedChoiceError())
    }
}

#[derive(Debug)]
pub struct WeightedChoiceError();
