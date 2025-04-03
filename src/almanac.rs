use std::{cmp::Ordering, io::Write};

use crate::{
    character::{Character, Team},
    script::Script,
};

#[derive(Debug, Default)]
pub struct AlmanacFields {
    pub intro: Vec<String>,
}

const STYLE: &str = include_str!("style.css");
const INDEX_STYLE: &str = include_str!("index_style.css");

impl Script {
    pub fn write_html<T>(
        &self,
        writer: &mut T,
        first_night_special: &[&Character],
        other_night_special: &[&Character],
    ) where
        T: Write,
    {
        self.write_head(writer);
        self.write_aside(writer);
        self.begin_main(writer);
        self.write_intro_page(writer);
        for character in &self.characters {
            self.write_character_page(writer, character);
        }
        self.write_night_order_page(writer, first_night_special, other_night_special);
        self.end_main(writer);
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

    fn begin_page<T>(&self, writer: &mut T, id: &str, class: Option<&str>)
    where
        T: Write,
    {
        if let Some(class) = class {
            write!(writer, "<div id=\"{id}\" class=\"page {class}\">").unwrap();
        } else {
            write!(writer, "<div id=\"{id}\" class=\"page\">").unwrap();
        }
    }

    fn end_page<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        write!(writer, "</div><div class=\"page-separator\"></div>").unwrap();
    }

    fn begin_main<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        write!(writer, "<main>").unwrap();
    }

    fn end_main<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        write!(writer, "</main>").unwrap();
    }

    fn write_aside<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        write!(
            writer,
            "<aside><a href=\"#intro\" class=\"intro\">Intro</a>"
        )
        .unwrap();
        for character in &self.characters {
            write!(
                writer,
                "<a href=\"#{}\" class=\"{}\">{}</a>",
                character.id,
                character.team.to_str(),
                character.name
            )
            .unwrap();
        }
        write!(
            writer,
            "<a href=\"#night-order\" class=\"night-order\">Night Order</a></aside>"
        )
        .unwrap();
    }

    fn write_intro_page<T>(&self, writer: &mut T)
    where
        T: Write,
    {
        self.begin_page(writer, "intro", None);

        for line in &self.almanac.intro {
            write!(writer, "<p class=\"intro\">{}</p>", line).unwrap();
        }

        self.end_page(writer);
    }

    fn write_character_page<T>(&self, writer: &mut T, character: &Character)
    where
        T: Write,
    {
        self.begin_page(writer, &character.id, Some(character.team.to_str()));

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
        if !character.advice.is_empty() {
            for line in &character.advice {
                write!(writer, "<p class=\"advice\">{}</p>", line).unwrap();
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

    fn write_night_order_page<T>(
        &self,
        writer: &mut T,
        first_night_special: &[&Character],
        other_night_special: &[&Character],
    ) where
        T: Write,
    {
        self.begin_page(writer, "night-order", None);

        write!(
            writer,
            "<h2 class=\"night-order\">NIGHT ORDER</h2><div class=\"night-order-container\">"
        )
        .unwrap();

        let mut sorted: Vec<_> = self
            .characters
            .iter()
            .filter(|character| character.first_night > 0.0)
            .collect();
        sorted.extend(first_night_special);
        sorted.sort_unstable_by(|a, b| {
            a.first_night
                .partial_cmp(&b.first_night)
                .unwrap_or(Ordering::Equal)
        });

        self.write_night_order(writer, "FIRST NIGHT", &sorted);

        let mut sorted: Vec<_> = self
            .characters
            .iter()
            .filter(|character| character.other_night > 0.0)
            .collect();
        sorted.extend(other_night_special);
        sorted.sort_unstable_by(|a, b| {
            a.other_night
                .partial_cmp(&b.other_night)
                .unwrap_or(Ordering::Equal)
        });

        self.write_night_order(writer, "OTHER NIGHTS", &sorted);

        self.end_page(writer);
    }

    fn write_night_order<T>(&self, writer: &mut T, header: &str, characters: &[&Character])
    where
        T: Write,
    {
        write!(
            writer,
            "<div class=\"night-order-list\"><h3 class=\"night-order-type\">{header}</h3><div class=\"night-order-list-container\">"
        )
        .unwrap();

        for character in characters {
            write!(writer, "<div class=\"night-order-entry\">").unwrap();
            if let Some(image) = character.image.first() {
                write!(writer, "<img src=\"{}\" />", image).unwrap()
            } else {
                write!(writer, "<div></div>").unwrap();
            }
            write!(writer, "<p>{}</p></div>", character.name).unwrap();
        }

        write!(writer, "</div></div>").unwrap();
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

pub fn write_index<T>(writer: &mut T, entries: &[(String, String)])
where
    T: Write,
{
    write!(
        writer,
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Toto's Script Index</title><style>{INDEX_STYLE}</style></head><body>",
    )
    .unwrap();

    for (id, name) in entries {
        write!(writer, "<div class=\"entry\"><a class=\"almanac\" href=\"/{id}.html\">{name}</a><a class=\"json\" href=\"/{id}.official.json\">Json</a></div>").unwrap();
    }

    write!(writer, "</body>").unwrap();
}
