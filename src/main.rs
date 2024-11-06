mod character;
mod patch;
mod script;

use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use character::Character;
use patch::read_patches;
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

    let mut image_list_str = String::new();
    File::open("official-images")
        .expect("Official image list not found")
        .read_to_string(&mut image_list_str)
        .expect("Failed to read official image list file");
    let mut image_list = HashMap::new();
    for image in image_list_str.lines().filter(|str| !str.is_empty()) {
        let mut iter = image.split(' ');
        let id = iter.next().unwrap().to_owned();
        let images: Vec<String> = iter
            .map(|image| format!("https://botc.app/assets/{image}.webp"))
            .collect();

        image_list.insert(id, images);
    }

    let patches = read_patches();

    load_dir(
        &Path::new("script-gen").join("characters"),
        &mut character_list,
    );

    let mut args = args().skip(1);
    let out_dir = args.next().expect("No out dir provided");

    for source in args {
        let mut script = Script::from_source(&source, &character_list);
        let mut writer = File::create(Path::new(&out_dir).join(format!(
            "{}.official.json",
            Path::new(&source).file_name().unwrap().to_str().unwrap()
        )))
        .unwrap_or_else(|_| panic!("Failed to create file for script {source}"));

        script.resolve_required(&character_list);
        script.apply_patches(&patches, &image_list);
        script.write_json(&mut writer);
    }
}

fn load_dir(path: &Path, character_list: &mut HashMap<String, Character>) {
    if let Ok(dir) = path.read_dir() {
        for character_entry in dir.flatten() {
            if character_entry.file_type().is_ok_and(|f| f.is_dir()) {
                load_dir(&character_entry.path(), character_list);
            } else if character_entry
                .path()
                .extension()
                .is_some_and(|ext| ext == "char")
            {
                let character = Character::from_source(&character_entry.path(), character_list);
                character_list.insert(character.id.clone(), character);
            }
        }
    }
}
