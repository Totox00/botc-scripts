use std::{collections::HashMap, ffi::OsStr, fs::File, io::Read, path::Path};

use crate::{character::Jinx, script::Script};

pub struct Patch {
    replace: Option<String>,
    add: Vec<String>,
    jinxes: Vec<Jinx>,
}

pub fn read_patches() -> HashMap<String, Patch> {
    let mut patches = HashMap::new();

    if let Ok(dir) = Path::new("script-gen").join("bootlegger").read_dir() {
        for patch in dir.flatten() {
            if let Some(Some(id)) = patch.path().file_stem().map(OsStr::to_str) {
                let mut buf = String::new();
                File::open(patch.path())
                    .unwrap_or_else(|_| panic!("Failed to open patch file for patch {id}",))
                    .read_to_string(&mut buf)
                    .unwrap_or_else(|_| panic!("Failed to read patch file for patch {id}",));

                let mut replace = None;
                let mut add = vec![];
                let mut jinxes = vec![];

                for line in buf.lines() {
                    if let Some((key, value)) = line.split_once(' ') {
                        match key {
                            "add" => add.push(value.to_owned()),
                            "replace" => {
                                replace = Some(value.to_owned());
                            }
                            "jinx" => {
                                if let Some((other, reason)) = value.split_once(' ') {
                                    jinxes.push(Jinx {
                                        id: other.to_owned(),
                                        reason: reason.to_owned(),
                                    })
                                } else {
                                    panic!("Jinx is missing target id in bootlegger patch for {id}")
                                }
                            }
                            _ => panic!("Invalid key {key} for bootlegger patch for {id}"),
                        }
                    }
                }

                patches.insert(
                    id.to_owned(),
                    Patch {
                        replace,
                        add,
                        jinxes,
                    },
                );
            }
        }
    }

    patches
}

impl Script {
    pub fn apply_patches(
        &mut self,
        patches: &HashMap<String, Patch>,
        image_list: &HashMap<String, Vec<String>>,
    ) {
        if self
            .characters
            .iter()
            .all(|character| patches.get(&character.id).is_none())
        {
            return;
        }

        let character_ids: Vec<String> =
            self.characters.iter().map(|char| char.id.clone()).collect();
        let mut patched_character_ids = vec![];

        for character in self.characters.iter_mut() {
            if let Some(patch) = patches.get(&character.id) {
                if patch.replace.is_none()
                    && patch.add.is_empty()
                    && patch
                        .jinxes
                        .iter()
                        .all(|jinx| !character_ids.contains(&jinx.id))
                {
                    continue;
                }

                patched_character_ids.push(character.id.clone());
                character.patched = true;
                character.id = format!("patched_{}", character.id);

                for jinx in &patch.jinxes {
                    if character_ids.contains(&jinx.id) {
                        character.jinxes.push(jinx.to_owned())
                    }
                }

                for add in &patch.add {
                    character.jinxes.push(Jinx {
                        id: character.id.clone(),
                        reason: add.to_owned(),
                    })
                }

                if let Some(replace) = &patch.replace {
                    character.ability = replace.to_owned();
                    character.jinxes.push(Jinx {
                        id: character.id.clone(),
                        reason: String::from("This character has a modified ability."),
                    })
                }
            }
        }

        let mut require_iter = true;
        while require_iter {
            require_iter = false;

            for character in self.characters.iter_mut() {
                if character
                    .jinxes
                    .iter()
                    .any(|jinx| patched_character_ids.contains(&jinx.id))
                {
                    if !character.patched {
                        patched_character_ids.push(character.id.clone());
                        character.patched = true;
                        require_iter = true;
                        if let Some(images) = image_list.get(&character.id) {
                            character.image = images.to_owned();
                        }

                        character.id = format!("patched_{}", character.id);
                    }

                    for jinx in character.jinxes.iter_mut() {
                        if patched_character_ids.contains(&jinx.id) {
                            jinx.id = format!("patched_{}", jinx.id);
                        }
                    }
                }
            }
        }
    }
}
