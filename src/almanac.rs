use std::io::Write;

use crate::{
    character::{Character, Team},
    script::Script,
};

#[derive(Debug, Default)]
pub struct AlmanacFields {
    pub intro: Vec<String>,
}

const STYLE: &str = include_str!("style.css");

impl Script {
    pub fn write_html<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        self.write_head(writer);
        self.write_intro_page(writer);
        for character in &self.characters {
            self.write_character_page(writer, character);
        }
        self.write_end(writer);
    }

    fn write_head<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        write!(
            writer,
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>{}</title><style>{STYLE}</style></head><body>",
            self.name
        )
        .unwrap();
    }

    fn write_end<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        write!(writer, "</body></html>").unwrap();
    }

    fn begin_page<T>(&self, writer: &mut T, class: Option<&str>)
    where
        T: Write,
    {
        if let Some(class) = class {
            write!(writer, "<div class=\"page {class}\">").unwrap();
        } else {
            write!(writer, "<div class=\"page\">").unwrap();
        }
    }

    fn end_page<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        write!(writer, "</div><div class=\"page-separator\"></div>").unwrap();
    }

    fn write_intro_page<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        self.begin_page(writer, None);

        for line in &self.almanac.intro {
            write!(writer, "<p class=\"intro\">{}</p>", line).unwrap();
        }

        self.end_page(writer);
    }

    fn write_character_page<T>(&self, writer: &mut T, character: &Character)
    where
        T: Write,
    {
        self.begin_page(writer, Some(character.team.to_str()));

        write!(writer, "<p class=\"team\">{}</p>", character.team.to_str()).unwrap();
        if let Some(image) = character.image.first() {
            write!(writer, "<img class=\"char-image\" src=\"{}\" />", image).unwrap();
        }
        write!(writer, "<h2 class=\"name\">{}</h2>", character.name).unwrap();
        write!(writer, "<p class=\"ability\">{}</p>", character.ability).unwrap();
        write!(writer, "<hr />").unwrap();
        if !character.flavour.is_empty() {
            write!(writer, "<p class=\"flavour\">\"{}\"</p>", character.flavour).unwrap();
        }

        if !character.overview_short.is_empty() {
            write!(
                writer,
                "<p class=\"overview-short\">{}</p>",
                character.overview_short
            )
            .unwrap();
        }
        if !character.overview_long.is_empty() {
            write!(writer, "<ul>").unwrap();
            for line in &character.overview_long {
                write!(writer, "<li>{}</li>", line).unwrap();
            }
            write!(writer, "</ul>").unwrap();
        }
        if !character.examples.is_empty() {
            write!(writer, "<h3>EXAMPLES</h3>").unwrap();
            for line in &character.examples {
                write!(writer, "<p>{}</p>", line).unwrap();
            }
        }
        if !character.how_to_run.is_empty() {
            write!(writer, "<h3>HOW TO RUN</h3>").unwrap();
            for line in &character.how_to_run {
                write!(writer, "<p>{}</p>", line).unwrap();
            }
        }
        if !character.attribution.is_empty() {
            write!(writer, "<h3>ATTRIBUTION</h3>").unwrap();
            for line in &character.attribution {
                write!(writer, "<p>{}</p>", line).unwrap();
            }
        }

        self.end_page(writer);
    }
}

impl Team {
    fn to_str(&self) -> &str {
        match self {
            Team::Townsfolk => "townsfolk",
            Team::Outsider => "outsider",
            Team::Minion => "minion",
            Team::Demon => "demon",
            Team::Traveller => "traveller",
            Team::Fabled => "fabled",
            Team::Special => "special",
        }
    }
}
