use crate::{character::{Character, Team}, night_order::NightOrder};

pub struct SpecialCharacters {
    pub dusk: Character,
    pub minions: Character,
    pub demon: Character,
    pub dawn: Character,
}

pub fn special_characters(night_order: &NightOrder) -> SpecialCharacters {
    SpecialCharacters {
        dusk: special_character("dusk", "Dusk", night_order),
        minions: special_character("minioninfo", "Minion Info", night_order),
        demon: special_character("demoninfo", "Demon Info", night_order),
        dawn: special_character("dawn", "Dawn", night_order),
    }
}

fn special_character(id: &str, name: &str, night_order: &NightOrder) -> Character {
    Character {
        id: String::from(id),
        name: String::from(name),
        edition: None,
        team: Team::Special,
        ability: String::new(),
        reminders: vec![],
        reminders_global: vec![],
        first_night_reminder: String::new(),
        other_night_reminder: String::new(),
        first_night: night_order.first_night(id).unwrap_or(0.0),
        other_night: night_order.other_night(id).unwrap_or(0.0),
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
