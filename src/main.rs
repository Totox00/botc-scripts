mod almanac;
mod character;
mod patch;
mod script;
mod special_characters;

use std::{
    collections::HashMap,
    env::args,
    fs::{create_dir_all, File},
    io::Read,
    path::Path,
};

use almanac::write_index;
use character::Character;
use patch::read_patches;
use script::Script;
use special_characters::{special_characters, SpecialCharacters};

fn main() {
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
        if let Some(image) = image_list.get(&character.id) {
            character.image = image.clone();
        }
        character_list.insert(character.id.clone(), character);
    }

    let SpecialCharacters {
        dusk,
        minions,
        demon,
        dawn,
    } = special_characters();
    let first_night_special = [&dusk, &minions, &demon, &dawn];
    let other_night_special = [&dusk, &dawn];

    let patches = read_patches();

    load_dir(
        &Path::new("script-gen").join("characters"),
        &mut character_list,
    );

    let mut args = args().skip(1);
    let out_dir = args.next().expect("No out dir provided");
    create_dir_all(&out_dir).expect("Failed to create out dir");

    let mut index_entries = vec![];

    for source in args {
        let file_name = Path::new(&source).file_name().unwrap().to_str().unwrap();
        let mut script = Script::from_source(&source, &character_list);
        let mut json_writer =
            File::create(Path::new(&out_dir).join(format!("{file_name}.official.json",)))
                .unwrap_or_else(|_| panic!("Failed to create script file for script {source}"));
        let mut html_writer = File::create(Path::new(&out_dir).join(format!("{file_name}.html",)))
            .unwrap_or_else(|_| panic!("Failed to create almanac file for script {source}"));

        script.resolve_required(&character_list);
        script.apply_patches(&patches, &image_list);
        script.write_json(&mut json_writer, file_name);
        script.write_html(&mut html_writer, &first_night_special, &other_night_special);
        index_entries.push((
            Path::new(&source)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            script.name,
        ));
    }

    let mut index_writer =
        File::create(Path::new(&out_dir).join("index.html")).expect("Failed to create index file");
    write_index(&mut index_writer, &index_entries);
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
