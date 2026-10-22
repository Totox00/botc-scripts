use std::{collections::HashMap, fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NightOrderData {
    #[serde(rename = "firstNight")]
    first_night: Vec<String>,
    #[serde(rename = "otherNight")]
    other_night: Vec<String>,
}

pub struct NightOrder {
    first: HashMap<String, f64>,
    other: HashMap<String, f64>,
}

impl NightOrder {
    pub fn read() -> Self {
        let mut night_order_str = String::new();
        File::open("night-order.json")
            .expect("Night order not found")
            .read_to_string(&mut night_order_str)
            .expect("Failed to read night order file");
        let night_order = serde_json::from_str::<NightOrderData>(&night_order_str)
            .expect("Failed to parse night order json");

        let mut first = HashMap::new();
        let mut other = HashMap::new();

        for (position, character) in night_order.first_night.into_iter().enumerate() {
            first.insert(character, position as f64);
        }

        for (position, character) in night_order.other_night.into_iter().enumerate() {
            other.insert(character, position as f64);
        }

        Self { first, other }
    }

    pub fn first_night(&self, character: &str) -> Option<f64> {
        self.first.get(character).copied()
    }

    pub fn other_night(&self, character: &str) -> Option<f64> {
        self.other.get(character).copied()
    }
}
