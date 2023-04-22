use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum MarkdownType {
    H1,
    H2,
    H3,
    P,
    Strong,
    Italic,
}

#[derive(Debug, PartialEq)]
pub struct Markdown {
    pub style: MarkdownType,
    pub content: String,
}

impl FromStr for Markdown {
    type Err = std::str::Utf8Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let h1_regex = Regex::new(r"^#\s").unwrap();

        if h1_regex.is_match(line) {
            return Ok(Markdown {
                style: MarkdownType::H1,
                content: line.to_string(),
            });
        }

        Ok(Markdown {
            style: MarkdownType::P,
            content: line.to_string(),
        })
    }
}

pub fn render(origin_string: String) -> Vec<Markdown> {
    origin_string
        .split('\n')
        .into_iter()
        .map(|line| match Markdown::from_str(line) {
            Ok(markdown) => markdown,
            Err(_) => {
                println!("not valid markdown");
                return Markdown { style: MarkdownType::P, content: line.to_string() };
            }
        }).collect()
}