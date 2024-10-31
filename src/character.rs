use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub team: Team,
    #[serde(default)]
    pub ability: String,
    #[serde(default = "Vec::new")]
    pub reminders: Vec<String>,
    #[serde(rename = "remindersGlobal", default = "Vec::new")]
    pub reminders_global: Vec<String>,
    #[serde(rename = "firstNightReminder", default)]
    pub first_night_reminder: String,
    #[serde(rename = "otherNightReminder", default)]
    pub other_night_reminder: String,
    #[serde(rename = "firstNight", default)]
    pub first_night: f32,
    #[serde(rename = "otherNight", default)]
    pub other_night: f32,
    #[serde(default)]
    pub official: bool,
    #[serde(rename = "flavor", default)]
    pub flavour: String,
    #[serde(default)]
    pub attribution: String,
    pub image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum Team {
    #[serde(rename = "townsfolk")]
    Townsfolk,
    #[serde(rename = "outsider")]
    Outsider,
    #[serde(rename = "minion")]
    Minion,
    #[serde(rename = "demon")]
    Demon,
    #[serde(rename = "traveller", alias = "traveler")]
    Traveller,
    #[serde(rename = "fabled")]
    Fabled,
    #[default]
    #[serde(rename = "special")]
    Special,
}

impl Character {
    pub fn from_source(
        source_path: &PathBuf,
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
            .unwrap_or_else(|_| panic!("Failed to open script source file for script {source}",))
            .read_to_string(&mut buf)
            .unwrap_or_else(|_| panic!("Failed to read script source file for script {source}",));

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
        let mut attribution = String::new();
        let mut flavour = String::new();

        while let Some(line) = lines.next() {
            match line {
                "attribution" => {
                    for line in lines.by_ref() {
                        if line.is_empty() {
                            break;
                        } else {
                            attribution.push_str(line);
                            attribution.push('\n');
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
                _ => {
                    if let Some((key, value)) = line.split_once(' ') {
                        match key {
                            "reminder" => {
                                if let Some((count, value)) = value.split_once(' ') {
                                    for _ in 0..count.parse().unwrap() {
                                        reminders.push(value.to_owned());
                                    }
                                }
                            }
                            "globalreminder" => {
                                if let Some((count, value)) = value.split_once(' ') {
                                    for _ in 0..count.parse().unwrap() {
                                        reminders_global.push(value.to_owned());
                                    }
                                }
                            }
                            "firstnight" => first_night_reminder = value.to_owned(),
                            "othernight" => other_night_reminder = value.to_owned(),
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
                                    _ => panic!("Invalid night for character {source}"),
                                }
                            }

                            _ => panic!("Invalid key {key} in character {source}"),
                        }
                    }
                }
            }
        }

        let mut buf = vec![];
        let image =
            if File::open(Path::new(source_path.parent().unwrap()).join(format!("{source}.png")))
                .map(|mut file| file.read_to_end(&mut buf))
                .is_ok()
            {
                Some(format!(
                    "data:image/png;base64,{}",
                    BASE64_STANDARD.encode(buf)
                ))
            } else {
                None
            };

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
            official: false,
            attribution: attribution.trim().to_owned(),
            flavour: flavour.trim().to_owned(),
            image,
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
