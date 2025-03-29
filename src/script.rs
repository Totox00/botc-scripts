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
