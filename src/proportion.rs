use crate::file::{File, FileType};
use std::{fs, io, io::prelude::*};

pub fn find_occurences(file: &File, word: String) -> u64 {
    let mut occ_count = 0; 

    match file.file_type {
        FileType::Txt => {
            let file = fs::File::open(&file.filename).unwrap();
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                if line.unwrap().contains(&word) {
                    occ_count += 1;
                }
            }
        }

        _ => {}
    }

    occ_count
}


