use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct RollResult {
    pub rolls: Vec<u8>,
    pub total: u8,
    pub num_rolls: u8,
}

impl RollResult {
    pub fn to_string(&self) -> String {
        let json_string = serde_json::to_string_pretty(&self).unwrap();

        json_string
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dice {
    pub sides: u8,
}

impl Dice {
    pub fn roll(&self, times: u8) -> RollResult {
        let num_rolls = times;

        let mut rolls: Vec<u8> = Vec::new();

        let mut rng = rand::thread_rng();

        for _i in 0..num_rolls {
            rolls.push(rng.gen_range(1..self.sides))
        }

        let total = rolls.iter().sum();

        RollResult {
            rolls,
            total,
            num_rolls,
        }
    }
}
