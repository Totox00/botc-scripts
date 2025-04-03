use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use serde_json::{Map, Value};

use crate::{almanac::AlmanacFields, character::Character};

#[derive(Debug)]
pub struct Script {
    pub name: String,
    pub author: String,
    pub characters: Vec<Character>,
    pub bootlegger_rules: Vec<String>,
    pub almanac: AlmanacFields,
}

static SORT_ORDER: [&str; 35] = [
    "You start knowing",
    "Each night",
    "Each night*",
    "Each day",
    "Once per day",
    "Once per game, at night",
    "Once per game, at night*",
    "Once per game, during the day",
    "Once per game",
    "On your 1st night",
    "On your 1st day",
    "You think",
    "You are",
    "You have",
    "You do not know",
    "You might",
    "You",
    "When you die",
    "When you learn that you died",
    "When",
    "If you die",
    "If you died",
    "If you are \"mad\"",
    "If you",
    "If the Demon dies",
    "If the Demon kills",
    "If the Demon",
    "If both",
    "If there are 5 or more players alive",
    "If",
    "All players",
    "All",
    "The 1st time",
    "The",
    "Minions",
];

impl Script {
    pub fn from_source(source: &str, character_list: &HashMap<String, Character>) -> Script {
        let mut buf = String::new();
        File::open(source)
            .unwrap_or_else(|_| panic!("Failed to open script source file for script {source}"))
            .read_to_string(&mut buf)
            .unwrap_or_else(|_| panic!("Failed to read script source file for script {source}"));

        let mut lines = buf.lines();

        let name = lines
            .next()
            .unwrap_or_else(|| panic!("Script {source} does not have a name"))
            .to_owned();
        let author = lines
            .next()
            .unwrap_or_else(|| panic!("Script {source} does not have an author"))
            .to_owned();
        let mut almanac = AlmanacFields::default();
        let mut characters = vec![];
        let mut bootlegger_rules = vec![];
        let mut sort_characters = true;

        while let Some(line) = lines.next() {
            match line.split_once(' ') {
                Some(("bootlegger", rule)) => {
                    bootlegger_rules.push(rule.to_string());
                }
                _ => match line {
                    "intro" => {
                        for line in lines.by_ref() {
                            if line.is_empty() {
                                break;
                            }
                            almanac.intro.push(line.to_string());
                        }
                    }
                    "keeporder" => {
                        sort_characters = false;
                    }
                    "" => (),
                    _ => characters.push(
                        character_list
                            .get(line)
                            .unwrap_or_else(|| {
                                panic!(
                                    "Failed to find data for character {line} in script {source}"
                                )
                            })
                            .clone(),
                    ),
                },
            }
        }

        if sort_characters {
            characters.sort_unstable_by(|a, b| {
                let cmp = a.team.cmp(&b.team);
                if cmp.is_ne() {
                    return cmp;
                }

                let a_idx = get_sort_idx(&a.ability);
                let b_idx = get_sort_idx(&b.ability);
                let cmp = a_idx.cmp(&b_idx);
                if cmp.is_ne() {
                    return cmp;
                }

                let cmp = a.ability.len().cmp(&b.ability.len());
                if cmp.is_ne() {
                    return cmp;
                }

                let cmp = a.name.len().cmp(&b.name.len());
                if cmp.is_ne() {
                    return cmp;
                }

                return a.name.cmp(&b.name);
            });
        }

        Script {
            name,
            author,
            characters,
            bootlegger_rules,
            almanac,
        }
    }

    pub fn resolve_required(&mut self, character_list: &HashMap<String, Character>) {
        let mut to_add = vec![];

        for character in &self.characters {
            for required in &character.required_characters {
                if !self
                    .characters
                    .iter()
                    .any(|character| character.id == *required)
                {
                    if let Some(required) = character_list.get(required) {
                        to_add.push(required.to_owned());
                    } else {
                        panic!(
                            "Could not find required character {required} for {}",
                            character.id
                        )
                    }
                }
            }
        }

        if !to_add.is_empty() {
            self.characters.extend(to_add);
            self.resolve_required(character_list);
        }
    }

    pub fn write_json<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        let mut out: Vec<Value> = vec![self.meta()];

        for character in &self.characters {
            if character.official && !character.patched {
                out.push(Value::String(character.id.to_owned()));
            } else {
                out.push(
                    serde_json::to_value(character).unwrap_or_else(|_| {
                        panic!("Failed to serialize character {}", character.id)
                    }),
                );
            }
        }

        serde_json::to_writer(writer, &out)
            .unwrap_or_else(|_| panic!("Failed to generate json for script {}", self.name))
    }

    fn meta(&self) -> Value {
        let mut map = Map::new();

        map.insert(String::from("id"), Value::String(String::from("_meta")));
        map.insert(String::from("name"), Value::String(self.name.to_owned()));
        map.insert(
            String::from("author"),
            Value::String(self.author.to_owned()),
        );
        if !self.bootlegger_rules.is_empty() {
            map.insert(
                String::from("bootlegger"),
                Value::Array(
                    self.bootlegger_rules
                        .iter()
                        .cloned()
                        .map(Value::String)
                        .collect(),
                ),
            );
        }

        Value::Object(map)
    }
}

fn get_sort_idx(ability: &str) -> usize {
    for idx in 0..SORT_ORDER.len() {
        if ability.starts_with(SORT_ORDER[idx]) {
            if let Some(next) = SORT_ORDER.get(idx + 1) {
                if !ability.starts_with(next) {
                    return idx;
                }
            } else {
                return idx;
            }
        }
    }

    SORT_ORDER.len()
}
