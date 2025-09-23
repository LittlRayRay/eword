use clap::Parser;
use std::fs;
use std::{io, io::prelude::*};

use rbook::prelude::*;
use rbook::reader::ReaderContent;
use rbook::Epub;

use scraper::{Html, Selector};

use pdf_extract;

mod file;
mod proportion;

use file::{File, FileType};


/// Simple program to count the number of words in ebooks
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// File to wordcount
    #[arg(short, long)]
    filename: String,
}

fn count_words(file: &File) -> Result<u64, &'static str> {
    let mut word_count: u64 = 0;

    match file.file_type {
        FileType::Txt => {
            let file = fs::File::open(&file.filename).unwrap();
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                word_count += line.unwrap().split(' ').collect::<Vec<&str>>().len() as u64;
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
                            word_count += text.split(' ').collect::<Vec<&str>>().len() as u64;
                        }
                    }
                }
            }
        }
        FileType::Pdf => {
            let bytes = std::fs::read(&file.filename).unwrap();
            let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
            for line in out.lines() {
                word_count += line.split(' ').collect::<Vec<&str>>().len() as u64;
            }
        }
    }
    return Ok(word_count);
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    if !fs::exists(&args.filename).unwrap() {
        println!("file {} does not exist", &args.filename);
        return Err("failed to find file");
    }

    let current_file: File;

    match args.filename.split('.').collect::<Vec<&str>>()[1] {
        "txt" => {
            current_file = File {
                filename: args.filename,
                file_type: FileType::Txt,
            }
        }
        "epub" => {
            current_file = File {
                filename: args.filename,
                file_type: FileType::Epub,
            }
        }
        "pdf" => {
            current_file = File {
                filename: args.filename,
                file_type: FileType::Pdf,
            }
        }
        _ => {
            println!("file type not supported!");
            return Err("file type not supported");
        }
    }
    
    let count_r = count_words(&current_file);
    
    if let Ok(count) = count_r {
        println!("file contains: {} words", count);
    } else {
        return Err(count_r.err().unwrap());
    }

    Ok(())
}
