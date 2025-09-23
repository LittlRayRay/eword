use clap::Parser;
use std::fs;

mod file;
mod word_utils;

use file::{File, FileType};


/// Simple program to count the number of words in ebooks
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// File to wordcount
    #[arg(short, long)]
    filename: String,
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
    
    let count_r = word_utils::get_word_count(&current_file);
    
    if let Ok(count) = count_r {
        println!("file contains: {} words", count);
    } else {
        return Err(count_r.err().unwrap());
    }

    Ok(())
}
