mod character;
mod script;

use std::{collections::HashMap, env::args, fs::File, io::Read, path::Path};

use character::Character;
use script::Script;

fn main() {
    let mut character_list_str = String::new();
    File::open("characters.json")
        .expect("Character list not found")
        .read_to_string(&mut character_list_str)
        .expect("Failed to read character list file");
    let mut character_list = HashMap::new();
    for mut character in serde_json::from_str::<Vec<Character>>(&character_list_str)
        .expect("Failed to parse character list json")
    {
        character.official = true;
        character_list.insert(character.id.clone(), character);
    }

    if let Ok(dir) = Path::new("script-gen").join("characters").read_dir() {
        for character_entry in dir.flatten() {
            if character_entry
                .path()
                .extension()
                .is_some_and(|ext| ext == "char")
            {
                let character = Character::from_source(&character_entry.path(), &character_list);
                character_list.insert(character.id.clone(), character);
            }
        }
    }

    let mut args = args().skip(1);
    let out_dir = args.next().expect("No out dir provided");

    for source in args {
        let script = Script::from_source(&source, &character_list);
        let mut writer = File::create(Path::new(&out_dir).join(format!(
            "{}.official.json",
            Path::new(&source).file_name().unwrap().to_str().unwrap()
        )))
        .unwrap_or_else(|_| panic!("Failed to create file for script {source}"));

        script.write_json(&mut writer);
    }
}
