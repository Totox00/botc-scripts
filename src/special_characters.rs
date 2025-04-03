use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::character::{Character, Team};

pub struct SpecialCharacters {
    pub dusk: Character,
    pub minions: Character,
    pub demon: Character,
    pub dawn: Character,
}

#[derive(Debug, Deserialize)]
struct NightOrder {
    #[serde(rename = "firstNight")]
    first_night: Vec<String>,
    #[serde(rename = "otherNight")]
    other_night: Vec<String>,
}

pub fn special_characters() -> SpecialCharacters {
    let mut night_order_str = String::new();
    File::open("night-order.json")
        .expect("Night order not found")
        .read_to_string(&mut night_order_str)
        .expect("Failed to read night order file");
    let night_order = serde_json::from_str::<NightOrder>(&night_order_str)
        .expect("Failed to parse night order json");

    SpecialCharacters {
        dusk: special_character("DUSK", "Dusk", &night_order),
        minions: special_character("MINION", "Minion Info", &night_order),
        demon: special_character("DEMON", "Demon Info", &night_order),
        dawn: special_character("DAWN", "Dawn", &night_order),
    }
}

fn special_character(id: &str, name: &str, night_order: &NightOrder) -> Character {
    Character {
        id: String::from(id),
        name: String::from(name),
        team: Team::Special,
        ability: String::new(),
        reminders: vec![],
        reminders_global: vec![],
        first_night_reminder: String::new(),
        other_night_reminder: String::new(),
        first_night: night_order
            .first_night
            .iter()
            .position(|other| other == id)
            .map(|pos| pos + 1)
            .unwrap_or(0) as f32,
        other_night: night_order
            .other_night
            .iter()
            .position(|other| other == id)
            .map(|pos| pos + 1)
            .unwrap_or(0) as f32,
        setup: false,
        official: true,
        patched: false,
        flavour: String::new(),
        overview_short: String::new(),
        overview_long: vec![],
        examples: vec![],
        how_to_run: vec![],
        advice: vec![],
        attribution: vec![],
        image: vec![],
        special: None,
        jinxes: vec![],
        required_characters: vec![],
    }
}
