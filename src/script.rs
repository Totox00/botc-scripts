use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use serde_json::{Map, Value};

use crate::character::{Character, Team};

#[derive(Debug)]
pub struct Script<'a> {
    pub name: String,
    pub author: String,
    pub characters: Vec<&'a Character>,
}

impl Script<'_> {
    pub fn from_source<'a>(
        source: &str,
        character_list: &'a HashMap<String, Character>,
    ) -> Script<'a> {
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
        let mut characters = vec![];

        for character in lines {
            if !character.is_empty() {
                characters.push(character_list.get(character).unwrap_or_else(|| {
                    panic!("Failed to find data for character {character} in script {source}")
                }))
            }
        }

        Script {
            name,
            author,
            characters,
        }
    }

    pub fn write_json<T>(self, writer: &mut T)
    where
        T: Write,
    {
        let mut out: Vec<Value> = vec![self.meta()];
        let mut included_fabled = vec![];

        for character in self.characters {
            if character.team == Team::Fabled {
                included_fabled.push(character.id.to_owned());
            }

            if character.official {
                out.push(Value::String(character.id.to_owned()));
            } else {
                out.push(
                    serde_json::to_value(character).unwrap_or_else(|_| {
                        panic!("Failed to serialize character {}", character.id)
                    }),
                );
            }

            for fabled in &character.required_fabled {
                if !included_fabled.contains(fabled) {
                    included_fabled.push(fabled.to_owned());
                    out.push(Value::String(fabled.to_owned()));
                }
            }
        }

        serde_json::to_writer_pretty(writer, &out)
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

        Value::Object(map)
    }
}
