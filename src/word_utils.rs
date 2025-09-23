use crate::file::{File, FileType};
use std::{fs, io, io::prelude::*};

use rbook::prelude::*;
use rbook::reader::ReaderContent;
use rbook::Epub;

use scraper::{Html, Selector};

use pdf_extract;

struct ParseResult {
    word_count: u64,
    occurences: u64,
}

fn parse_file(file: &File, word: &str) -> Result<ParseResult, &'static str> {
    let mut word_count: u64 = 0;
    let mut occurences: u64 = 0;

    match file.file_type {
        FileType::Txt => {
            let file = fs::File::open(&file.filename).unwrap();
            let reader = io::BufReader::new(file);

            for line_o in reader.lines() {
                if let Ok(line) = line_o {
                    let words = line.split(' ').collect::<Vec<&str>>();
                    word_count += words.len() as u64;
                    occurences += words.contains(&word) as u64;
                }
            }
        }
        FileType::Epub => {
            let epub = Epub::open(&file.filename).unwrap();
            let mut reader = epub.reader();

            let selector = Selector::parse("p,h1,h2,h3,h4,h5,h6,li,blockquote,pre").unwrap();

            for content_o in &mut reader {
                if let Ok(content) = content_o {
                    let doc = Html::parse_document(content.content());

                    for element in doc.select(&selector) {
                        for text in element.text() {
                            let words = text.split(' ').collect::<Vec<&str>>();
                            word_count += words.len() as u64;
                            occurences += words.contains(&word) as u64;
                        }
                    }
                }
            }
        }
        FileType::Pdf => {
            let bytes = std::fs::read(&file.filename).unwrap();
            let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
            for line in out.lines() {
                let words = line.split(' ').collect::<Vec<&str>>();
                word_count += words.len() as u64;
                occurences += words.contains(&word) as u64;
            }
        }
    }
    return Ok(ParseResult{word_count, occurences});
}

pub fn get_word_count(file: &File) -> Result<u64, &'static str> {
    Ok(parse_file(file, "")?.word_count)
}

pub fn get_word_occurences(file: &File, word: &str) -> Result<u64, &'static str> {
    Ok(parse_file(file, word)?.occurences)
}

pub fn get_word_percentage(file: &File, word: &str) -> Result<f32, &'static str> {
    if let Ok(res) = parse_file(file, word) {
        return Ok(res.occurences as f32 / res.word_count as f32);
    } else {
        return Err("failed to parse file");
    }
}
