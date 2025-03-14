use std::{collections::HashMap, fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Character {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub team: Team,
    #[serde(default)]
    pub ability: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reminders: Vec<String>,
    #[serde(
        rename = "remindersGlobal",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reminders_global: Vec<String>,
    #[serde(
        rename = "firstNightReminder",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub first_night_reminder: String,
    #[serde(
        rename = "otherNightReminder",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub other_night_reminder: String,
    #[serde(rename = "firstNight", default, skip_serializing_if = "is_zero")]
    pub first_night: f32,
    #[serde(rename = "otherNight", default, skip_serializing_if = "is_zero")]
    pub other_night: f32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub setup: bool,
    #[serde(default, skip_serializing)]
    pub official: bool,
    #[serde(default, skip_serializing)]
    pub patched: bool,
    #[serde(rename = "flavor", default, skip_serializing_if = "String::is_empty")]
    pub flavour: String,
    #[serde(default, skip_serializing)]
    pub overview_short: String,
    #[serde(default, skip_serializing)]
    pub overview_long: Vec<String>,
    #[serde(default, skip_serializing)]
    pub examples: Vec<String>,
    #[serde(default, skip_serializing)]
    pub how_to_run: Vec<String>,
    #[serde(default, skip_serializing)]
    pub advice: Vec<String>,
    #[serde(default, skip_serializing)]
    pub attribution: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jinxes: Vec<Jinx>,
    #[serde(skip)]
    pub required_characters: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq, Clone)]
pub enum Team {
    #[serde(rename = "townsfolk")]
    Townsfolk,
    #[serde(rename = "outsider")]
    Outsider,
    #[serde(rename = "minion")]
    Minion,
    #[serde(rename = "demon")]
    Demon,
    #[serde(rename = "traveller")]
    Traveller,
    #[serde(rename = "fabled")]
    Fabled,
    #[default]
    #[serde(rename = "special")]
    Special,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AppSpecial {
    bag_disabled: bool,
    bag_duplicate: bool,
    grimoire: bool,
    grimoire_global_demon: bool,
    cards: Vec<String>,
    replace_reveal: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Jinx {
    pub id: String,
    pub reason: String,
}

impl Character {
    pub fn from_source(
        source_path: &Path,
        character_list: &HashMap<String, Character>,
    ) -> Character {
        let source = source_path
            .file_stem()
            .unwrap()
            .to_ascii_lowercase()
            .into_string()
            .unwrap();
        let mut buf = String::new();
        File::open(source_path)
            .unwrap_or_else(|_| {
                panic!("Failed to open character source file for character {source}",)
            })
            .read_to_string(&mut buf)
            .unwrap_or_else(|_| {
                panic!("Failed to read character source file for character {source}",)
            });

        let mut lines = buf.lines();

        let name = lines
            .next()
            .unwrap_or_else(|| panic!("Character {source} does not have a name"))
            .to_owned();
        let team = lines
            .next()
            .unwrap_or_else(|| panic!("Character {source} does not have a team"))
            .to_owned();
        let ability = lines
            .next()
            .unwrap_or_else(|| panic!("Character {source} does not have an ability"))
            .to_owned();

        let mut reminders = vec![];
        let mut reminders_global = vec![];
        let mut first_night_reminder = String::new();
        let mut other_night_reminder = String::new();
        let mut first_night = 0f32;
        let mut other_night = 0f32;
        let mut setup = false;
        let mut flavour = String::new();
        let mut overview_short = String::new();
        let mut overview_long = vec![];
        let mut examples = vec![];
        let mut how_to_run = vec![];
        let mut advice = vec![];
        let mut attribution = vec![];
        let mut special = AppSpecial::default();
        let mut jinxes = vec![];
        let mut required_characters = vec![];
        let img_path = Path::new(source_path.parent().unwrap()).join(format!("{source}.png"));
        let mut image = if img_path.exists() {
            let mut components = img_path.components();
            components.next();
            components.next();
            vec![format!(
                "https://raw.githubusercontent.com/Totox00/botc-scripts/refs/heads/main/script-gen/characters/{}", components.as_path().to_str().unwrap()
            )]
        } else {
            vec![]
        };

        while let Some(line) = lines.next() {
            match line {
                "setup" => setup = true,
                "bagdisabled" => special.bag_disabled = true,
                "bagduplicate" => special.bag_duplicate = true,
                "grimoire" => special.grimoire = true,
                "replacereveal" => special.replace_reveal = true,
                "grimoire_global_demon" => special.grimoire_global_demon = true,
                "attribution" => {
                    for line in lines.by_ref() {
                        if line.is_empty() {
                            break;
                        } else {
                            attribution.push(line.to_string());
                        }
                    }
                }
                "flavour" | "flavor" => {
                    for line in lines.by_ref() {
                        if line.is_empty() {
                            break;
                        } else {
                            flavour.push_str(line);
                            flavour.push('\n');
                        }
                    }
                }
                "examples" => {
                    for line in lines.by_ref() {
                        if line.is_empty() {
                            break;
                        } else {
                            examples.push(line.to_string());
                        }
                    }
                }
                "howtorun" => {
                    for line in lines.by_ref() {
                        if line.is_empty() {
                            break;
                        } else {
                            how_to_run.push(line.to_string());
                        }
                    }
                }
                "advice" => {
                    for line in lines.by_ref() {
                        if line.is_empty() {
                            break;
                        } else {
                            advice.push(line.to_string());
                        }
                    }
                }
                _ => {
                    if let Some((key, value)) = line.split_once(' ') {
                        match key {
                            "reminder" => {
                                if let Some((count, value)) = value.split_once(' ') {
                                    for _ in 0..count.parse().unwrap_or_else(|_| {
                                        panic!("Reminder for {source} does not have a count")
                                    }) {
                                        reminders.push(value.to_owned());
                                    }
                                }
                            }
                            "globalreminder" => {
                                if let Some((count, value)) = value.split_once(' ') {
                                    for _ in 0..count.parse().unwrap_or_else(|_| {
                                        panic!("Reminder for {source} does not have a count")
                                    }) {
                                        reminders_global.push(value.to_owned());
                                    }
                                }
                            }
                            "image" => image = value.split(' ').map(String::from).collect(),
                            "firstnight" => first_night_reminder = value.to_owned(),
                            "othernight" => other_night_reminder = value.to_owned(),
                            "everynight" => {
                                first_night_reminder = value.to_owned();
                                other_night_reminder = value.to_owned();
                            }
                            "wakes" => {
                                let mut split = value.split(' ');
                                let night = split.next().unwrap_or_else(|| {
                                    panic!("Waking pattern for {source} is missing night")
                                });
                                let relation = split.next().unwrap_or_else(|| {
                                    panic!("Waking pattern for {source} is missing relation")
                                });
                                let other_id = split.next().unwrap_or_else(|| {
                                    panic!("Waking pattern for {source} is missing other id")
                                });

                                let other_char = character_list.get(other_id).unwrap_or_else(|| panic!("Could not find character with id {other_id} for character {source}"));
                                let offset = match relation {
                                    "before" => -0.1,
                                    "after" => 0.1,
                                    _ => panic!("Invalid relation for character {source}"),
                                };

                                match night {
                                    "first" => first_night = other_char.first_night + offset,
                                    "other" => other_night = other_char.other_night + offset,
                                    "every" => {
                                        first_night = other_char.first_night + offset;
                                        other_night = other_char.other_night + offset;
                                    }
                                    _ => panic!("Invalid night for character {source}"),
                                }
                            }
                            "overview" => {
                                overview_short = value.to_owned();
                                for line in lines.by_ref() {
                                    if line.is_empty() {
                                        break;
                                    } else {
                                        overview_long.push(line.to_string());
                                    }
                                }
                            }
                            "requires" => required_characters.push(value.to_owned()),
                            "card" => special.cards.push(value.to_owned()),
                            "jinx" => {
                                if let Some((id, reason)) = value.split_once(' ') {
                                    jinxes.push(Jinx {
                                        id: id.to_owned(),
                                        reason: reason.to_owned(),
                                    });
                                } else {
                                    panic!("Invalid jinx for {source}");
                                }
                            }
                            _ => panic!("Invalid key {key} in character {source}"),
                        }
                    }
                }
            }
        }

        Character {
            id: source,
            name,
            team: Team::from(team.as_str()),
            ability,
            reminders,
            reminders_global,
            first_night_reminder,
            other_night_reminder,
            first_night,
            other_night,
            setup,
            official: false,
            patched: false,
            flavour: flavour.trim().to_owned(),
            overview_short: overview_short.trim().to_owned(),
            overview_long,
            examples,
            how_to_run,
            advice,
            attribution,
            image,
            required_characters,
            special: if special.any() {
                Some(special.as_serializable())
            } else {
                None
            },
            jinxes,
        }
    }
}

impl From<&str> for Team {
    fn from(value: &str) -> Self {
        match value {
            "Townsfolk" => Team::Townsfolk,
            "Outsider" => Team::Outsider,
            "Minion" => Team::Minion,
            "Demon" => Team::Demon,
            "Traveller" => Team::Traveller,
            "Fabled" => Team::Fabled,
            "Special" => Team::Special,
            _ => panic!("Invalid team {value}"),
        }
    }
}

impl AppSpecial {
    pub fn any(&self) -> bool {
        self.bag_disabled
            || self.bag_duplicate
            || self.grimoire
            || self.grimoire_global_demon
            || self.replace_reveal
            || !self.cards.is_empty()
    }

    pub fn as_serializable(&self) -> Value {
        let mut out = vec![];

        if self.bag_disabled {
            let mut map = Map::new();
            map.insert(
                String::from("type"),
                Value::String(String::from("selection")),
            );
            map.insert(
                String::from("name"),
                Value::String(String::from("bag-disabled")),
            );
            out.push(Value::Object(map));
        }

        if self.bag_duplicate {
            let mut map = Map::new();
            map.insert(
                String::from("type"),
                Value::String(String::from("selection")),
            );
            map.insert(
                String::from("name"),
                Value::String(String::from("bag-duplicate")),
            );
            out.push(Value::Object(map));
        }

        if self.grimoire {
            let mut map = Map::new();
            map.insert(String::from("type"), Value::String(String::from("signal")));
            map.insert(
                String::from("name"),
                Value::String(String::from("grimoire")),
            );
            map.insert(String::from("time"), Value::String(String::from("night")));
            out.push(Value::Object(map));
        }

        if self.grimoire_global_demon {
            let mut map = Map::new();
            map.insert(String::from("type"), Value::String(String::from("signal")));
            map.insert(
                String::from("name"),
                Value::String(String::from("grimoire")),
            );
            map.insert(String::from("time"), Value::String(String::from("night")));
            map.insert(String::from("global"), Value::String(String::from("demon")));
            out.push(Value::Object(map));
        }

        if self.replace_reveal {
            let mut map = Map::new();
            map.insert(String::from("type"), Value::String(String::from("reveal")));
            map.insert(
                String::from("name"),
                Value::String(String::from("replace-character")),
            );
            out.push(Value::Object(map));
        }

        for card in &self.cards {
            let mut map = Map::new();
            map.insert(String::from("type"), Value::String(String::from("signal")));
            map.insert(String::from("name"), Value::String(String::from("card")));
            map.insert(String::from("value"), Value::String(card.to_owned()));
            out.push(Value::Object(map));
        }

        Value::Array(out)
    }
}

fn is_zero(n: &f32) -> bool {
    *n == 0f32
}

fn is_false(b: &bool) -> bool {
    !b
}
